mod api;
mod chemin;
mod donnees;
mod interne;
mod outils;

use crate::interne::erreurs::TraitErreur;
use crate::outils::trad::igdb::Traduisible;

#[tokio::main]
async fn main() {
    let configuration = match donnees::config::charger_config() {
        Ok(configuration) => configuration,
        Err(erreur) => {
            eprintln!("ERREUR: {:?}", erreur);
            return;
        },
    };
    println!("Configuration: {:?}", configuration);

    //println!("Jeux: {:?}", outils::scan::trouver_jeux());
/*
    let client = match api::igdb::ClientIGDB::new().await {
        Ok(valeur) => valeur,
        Err(erreur) => return erreur.afficher_erreur(),
    };

    let resultat = match client.solliciter(api::igdb::requete::requete_trouver_jeu(format!("Professor Layton and the diabolical box"), None)).await {
        Ok(resultat) => resultat[0].prompt().unwrap(),
        Err(erreur) => return erreur.afficher_erreur(),
    };

    let client = match api::openai::ClientOpenAI::new().await {
        Ok(valeur) => valeur,
        Err(erreur) => return erreur.afficher_erreur(),
    };

    //println!("{:?}", api::openai::requete::requete("You do what the user asks and only return json files.".to_string(), resultat));

    match client.solliciter(api::openai::requete::requete("You do what the user asks and only return json files.".to_string(), resultat)).await {
        Ok(valeur) => println!("Réponse:\n{}", valeur.choices[0].message.content),
        Err(erreur) => return erreur.afficher_erreur(),
    }
*/
    match donnees::db::igdb::creer_db().await {
        Ok(_) => (),
        Err(erreur) => return erreur.afficher_erreur(),
    }
}
