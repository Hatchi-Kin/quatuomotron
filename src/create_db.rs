use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connexion à la base de données (création de la base si elle n'existe pas)
    let conn = Connection::open("eleves.db")?;

    // Création de la table `eleves`
    conn.execute(
        "CREATE TABLE IF NOT EXISTS eleves (
                  id INTEGER PRIMARY KEY,
                  nom TEXT NOT NULL
                  )",
        [],
    )?;

    // Liste de noms d'élèves (noms de héros DC Comics)
    let noms_heros = vec![
        "Superman", "Batman", "Wonder Woman", "Flash", "Green Lantern", "Aquaman", "Cyborg", 
        "Martian Manhunter", "Hawkgirl", "Shazam", "Black Canary", "Green Arrow", "Nightwing", 
        "Robin", "Batgirl", "Supergirl", "Zatanna", "Red Tornado", "Blue Beetle", "Firestorm",
        "Huntress", "Vixen", "Starfire", "Raven", "Beast Boy", "Dr. Fate", "Constantine", 
        "Booster Gold", "Black Lightning", "Hawkman", "Atom", "Swamp Thing", "Captain Atom", 
        "Plastic Man", "Elongated Man", "Steel", "Orion", "Big Barda", "Mr. Miracle", 
        "Etrigan the Demon", "Deadman", "Phantom Stranger", "Animal Man", "The Spectre", 
        "Jonah Hex", "Katana", "Azrael", "Metamorpho", "Fire", "Ice", "Atom Smasher", 
        "Red Hood", "Spoiler", "Black Adam", "Deathstroke", "Lex Luthor", "Joker", 
        "Harley Quinn", "Catwoman", "Riddler", "Penguin", "Two-Face", "Scarecrow", 
        "Poison Ivy", "Clayface", "Killer Croc", "Ra's al Ghul", "Talia al Ghul", 
        "Deadshot", "Captain Cold", "Reverse-Flash", "Gorilla Grodd", "Sinestro", 
        "Brainiac", "Darkseid", "General Zod", "Bizarro", "Cheetah", "Circe", 
        "Black Manta", "Ocean Master", "Trigon", "Ares", "Doomsday", "Parasite", 
        "Atrocitus", "Lobo", "Killer Frost", "Livewire", "Tigress", "Sportsmaster"
    ];

    // Insertion des noms dans la table
    for nom in noms_heros {
        conn.execute(
            "INSERT INTO eleves (nom) VALUES (?1)",
            params![nom],
        )?;
    }

    println!("Base de données d'élèves générée avec succès !");
    Ok(())
}
