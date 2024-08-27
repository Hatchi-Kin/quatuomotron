#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize};
use rusqlite::{Connection, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::http::Method;
use rocket::response::status;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

// Structure représentant un élève
#[derive(Serialize, Clone)]
struct Eleve {
    id: usize,
    nom: String,
}

// Structure représentant une réponse avec les groupes générés
#[derive(Serialize)]
struct GroupesResponse {
    groupes: Vec<Vec<Eleve>>,
}

// Route pour récupérer la liste complète des élèves
#[get("/eleves")]
fn list_eleves() -> Result<Json<Vec<Eleve>>, status::Custom<String>> {
    // Connexion à la base de données SQLite
    let conn = Connection::open("eleves.db").map_err(|_| status::Custom(
        rocket::http::Status::InternalServerError,
        "Erreur de connexion à la base de données".to_string(),
    ))?;

    // Récupération des noms des élèves depuis la base de données
    let mut stmt = conn.prepare("SELECT id, nom FROM eleves").map_err(|_| status::Custom(
        rocket::http::Status::InternalServerError,
        "Erreur de préparation de la requête".to_string(),
    ))?;
    
    let eleves_iter = stmt.query_map([], |row| {
        Ok(Eleve {
            id: row.get(0)?,
            nom: row.get(1)?,
        })
    }).map_err(|_| status::Custom(
        rocket::http::Status::InternalServerError,
        "Erreur lors de l'exécution de la requête".to_string(),
    ))?;

    // Stocker les élèves dans un vecteur
    let mut eleves: Vec<Eleve> = Vec::new();
    for eleve in eleves_iter {
        eleves.push(eleve.map_err(|_| status::Custom(
            rocket::http::Status::InternalServerError,
            "Erreur lors de la récupération des élèves".to_string(),
        ))?);
    }

    if eleves.is_empty() {
        return Err(status::Custom(rocket::http::Status::NotFound, "Aucun élève trouvé".to_string()));
    }

    Ok(Json(eleves))
}

// Route pour générer des groupes d'élèves
#[get("/groupes/<method>/<value>")]
fn generate_groupes(
    method: String,
    value: usize
) -> Result<Json<GroupesResponse>, status::Custom<String>> {
    // Connexion à la base de données SQLite
    let conn = Connection::open("eleves.db").map_err(|_| status::Custom(
        rocket::http::Status::InternalServerError,
        "Erreur de connexion à la base de données".to_string(),
    ))?;

    // Récupération des noms des élèves depuis la base de données
    let mut stmt = conn.prepare("SELECT id, nom FROM eleves").map_err(|_| status::Custom(
        rocket::http::Status::InternalServerError,
        "Erreur de préparation de la requête".to_string(),
    ))?;
    
    let eleves_iter = stmt.query_map([], |row| {
        Ok(Eleve {
            id: row.get(0)?,
            nom: row.get(1)?,
        })
    }).map_err(|_| status::Custom(
        rocket::http::Status::InternalServerError,
        "Erreur lors de l'exécution de la requête".to_string(),
    ))?;

    let mut eleves: Vec<Eleve> = Vec::new();
    for eleve in eleves_iter {
        eleves.push(eleve.map_err(|_| status::Custom(
            rocket::http::Status::InternalServerError,
            "Erreur lors de la récupération des élèves".to_string(),
        ))?);
    }

    if eleves.is_empty() {
        return Err(status::Custom(rocket::http::Status::NotFound, "Aucun élève trouvé".to_string()));
    }

    // Mélanger les élèves
    let mut rng = thread_rng();
    eleves.shuffle(&mut rng);

    // Générer des groupes en fonction de la méthode choisie
    let groupes: Vec<Vec<Eleve>> = match method.as_str() {
        "taille" => former_groupes_par_taille(&eleves, value),
        "nombre" => former_groupes_par_nombre(&eleves, value),
        _ => return Err(status::Custom(rocket::http::Status::BadRequest, "Méthode invalide".to_string())),
    };

    Ok(Json(GroupesResponse { groupes }))
}

// Fonction pour former des groupes par taille
fn former_groupes_par_taille(eleves: &[Eleve], taille_groupe: usize) -> Vec<Vec<Eleve>> {
    let mut groupes: Vec<Vec<Eleve>> = Vec::new();
    let mut groupe_courant = Vec::new();

    for eleve in eleves.iter().cloned() {
        groupe_courant.push(eleve);

        if groupe_courant.len() == taille_groupe {
            groupes.push(groupe_courant);
            groupe_courant = Vec::new();
        }
    }

    if !groupe_courant.is_empty() {
        if groupe_courant.len() < taille_groupe / 2 {
            redistribuer_reste(&mut groupes, groupe_courant);
        } else {
            groupes.push(groupe_courant);
        }
    }

    groupes
}

// Fonction pour former des groupes par nombre de groupes
fn former_groupes_par_nombre(eleves: &[Eleve], nombre_groupes: usize) -> Vec<Vec<Eleve>> {
    let taille_groupe = (eleves.len() as f64 / nombre_groupes as f64).ceil() as usize;
    former_groupes_par_taille(eleves, taille_groupe)
}

// Redistribuer les élèves restants de manière équitable
fn redistribuer_reste(groupes: &mut Vec<Vec<Eleve>>, reste: Vec<Eleve>) {
    let mut index = 0;
    for eleve in reste {
        groupes[index].push(eleve);
        index = (index + 1) % groupes.len();
    }
}

// Route pour l'accueil
#[get("/")]
fn index() -> &'static str {
    "Bienvenue sur le serveur API Rocket !"
}

// Route pour le favicon
#[get("/favicon.ico")]
fn favicon() -> &'static str {
    "" // Vous pouvez servir un vrai favicon si nécessaire
}

// Configuration CORS pour permettre les requêtes depuis le frontend Vue.js
#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get].into_iter().map(From::from).collect())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .expect("Erreur lors de la construction du CORS");

    rocket::build()
        .attach(cors)
        .mount("/", routes![index, favicon]) // Ajout des routes de base
        .mount("/api", routes![list_eleves, generate_groupes]) // Routes API
}
