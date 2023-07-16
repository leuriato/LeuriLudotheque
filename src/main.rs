mod api;
mod chemin;
mod donnees;
mod gui;
mod interne;
mod outils;

use outils::{dl::telecharger_couvertures, scan::scanner};

use crate::interne::erreurs::TraitErreur;

#[tokio::main]
async fn main() {
    match donnees::igdb::creer_db().await {
        Ok(_) => (),
        Err(erreur) => return erreur.afficher_erreur(),
    }

    scanner().await;

    telecharger_couvertures().await;

    gui::lancer_application();
}
