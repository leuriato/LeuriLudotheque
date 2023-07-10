mod api;
mod chemin;
mod donnees;
mod interne;
mod outils;

use crate::interne::erreurs::TraitErreur;
use crate::outils::trad::igdb::Traduisible;

fn main() {
    let configuration = match donnees::config::charger_config() {
        Ok(configuration) => configuration,
        Err(erreur) => {
            eprintln!("ERREUR: {:?}", erreur);
            return;
        },
    };
    println!("Configuration: {:?}", configuration);

    //println!("Jeux: {:?}", outils::scan::trouver_jeux());

    let client = match api::igdb::ClientIGDB::new() {
        Ok(valeur) => valeur,
        Err(erreur) => return erreur.afficher_erreur(),
    };

    /*match client.demander(format!("platforms/"), format!("search \"switch\";fields *;")) {
        Ok(resultat) => println!("Résultat: {}", resultat),
        Err(erreur) => erreur.afficher_erreur(),
    }*/

    match client.solliciter(api::igdb::requete::requete_trouver_jeu(format!("Professor Layton and the diabolical box"), None)) {
        Ok(resultat) => println!("Résultat:\n{}", resultat[0].prompt().unwrap()),
        Err(erreur) => erreur.afficher_erreur(),
    }
}
