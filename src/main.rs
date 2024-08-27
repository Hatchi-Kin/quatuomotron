use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::{Connection, Result};
use std::io::{self, Write};

fn main() -> Result<()> {
    // Connexion à la base de données SQLite
    let conn = Connection::open("eleves.db")?;

    // Récupération des noms des élèves depuis la base de données
    let mut stmt = conn.prepare("SELECT nom FROM eleves")?;
    let eleves_iter = stmt.query_map([], |row| {
        let nom: String = row.get(0)?;
        Ok(nom)
    })?;

    // Stocker les noms d'élèves dans un vecteur
    let mut eleves: Vec<String> = Vec::new();
    for eleve in eleves_iter {
        eleves.push(eleve?);
    }

    // Vérification du nombre d'élèves
    if eleves.is_empty() {
        println!("Aucun élève trouvé dans la base de données !");
        return Ok(());
    }

    // Choix de la méthode de regroupement
    let mut input = String::new();
    print!("Voulez-vous spécifier le nombre d'élèves par groupe (1) ou le nombre de groupes (2) ? Entrez 1 ou 2 : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let choix: usize = input.trim().parse().expect("Veuillez entrer 1 ou 2.");

    let mut rng = thread_rng();
    eleves.shuffle(&mut rng);

    let groupes: Vec<Vec<String>> = match choix {
        1 => {
            // Spécification du nombre d'élèves par groupe
            input.clear();
            print!("Entrez le nombre d'élèves par groupe : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let taille_groupe: usize = input.trim().parse().expect("Veuillez entrer un nombre valide.");
            former_groupes_par_taille(&eleves, taille_groupe)
        }
        2 => {
            // Spécification du nombre de groupes
            input.clear();
            print!("Entrez le nombre de groupes souhaité : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let nombre_groupes: usize = input.trim().parse().expect("Veuillez entrer un nombre valide.");
            former_groupes_par_nombre(&eleves, nombre_groupes)
        }
        _ => {
            println!("Choix non valide. Terminaison du programme.");
            return Ok(());
        }
    };

    // Affichage des groupes
    for (i, groupe) in groupes.iter().enumerate() {
        println!("Groupe {}: {:?}", i + 1, groupe);
    }

    Ok(())
}

fn former_groupes_par_taille(eleves: &[String], taille_groupe: usize) -> Vec<Vec<String>> {
    let mut groupes: Vec<Vec<String>> = Vec::new();
    let mut groupe_courant = Vec::new();

    for eleve in eleves {
        groupe_courant.push(eleve.clone());

        if groupe_courant.len() == taille_groupe {
            groupes.push(groupe_courant);
            groupe_courant = Vec::new();
        }
    }

    // Gestion des élèves restants
    if !groupe_courant.is_empty() {
        if groupe_courant.len() < taille_groupe / 2 {
            redistribuer_reste(&mut groupes, groupe_courant);
        } else {
            groupes.push(groupe_courant);
        }
    }

    groupes
}

fn former_groupes_par_nombre(eleves: &[String], nombre_groupes: usize) -> Vec<Vec<String>> {
    let taille_groupe = (eleves.len() as f64 / nombre_groupes as f64).ceil() as usize;
    former_groupes_par_taille(eleves, taille_groupe)
}

fn redistribuer_reste(groupes: &mut Vec<Vec<String>>, reste: Vec<String>) {
    let mut index = 0;
    for eleve in reste {
        groupes[index].push(eleve);
        index = (index + 1) % groupes.len();
    }
}
