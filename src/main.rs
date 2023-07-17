mod api;
mod chemin;
mod donnees;
mod gui;
mod interne;
mod outils;

use gui::lancer_application;
use outils::scan::scanner;

#[tokio::main]
async fn main() {
    scanner().await;

    lancer_application();
}
