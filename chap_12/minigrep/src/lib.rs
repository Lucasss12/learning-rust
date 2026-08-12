use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub recherche: String,
    pub nom_fichier: String,
    pub sensible_casse: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("il n'y a pas assez d'arguments");
        }

        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();
        
        let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();

        Ok(Config { recherche, nom_fichier, sensible_casse })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenu = fs::read_to_string(config.nom_fichier)?;

    let resultats = if config.sensible_casse {
        rechercher(&config.recherche, &contenu)
    } else {
        rechercher_insensible_casse(&config.recherche, &contenu)
    };

    for ligne in resultats {
        println!("{}", ligne);
    }

    Ok(())
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
    let mut resultats = Vec::new();

    for ligne in contenu.lines() {
        if ligne.contains(recherche) {
            resultats.push(ligne);
        }
    }

    resultats
}

pub fn rechercher_insensible_casse<'a>(
    recherche: &str,
    contenu: &'a str
) -> Vec<&'a str> {
    let recherche = recherche.to_lowercase();
    let mut resultats = Vec::new();

    for ligne in contenu.lines() {
        if ligne.to_lowercase().contains(&recherche) {
            resultats.push(ligne);
        }
    }

    resultats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensible_casse() {
        let recherche = "duct";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duck tape.";

        assert_eq!(vec!["sécurité, rapidité, productivité."], rechercher(recherche, contenu));
    }

    #[test]
    fn insensible_casse() {
        let recherche = "rUsT";
        let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

        assert_eq!(
            vec!["Rust:", "C'est pas rustique."],
            rechercher_insensible_casse(recherche, contenu)
        );
    }
}
