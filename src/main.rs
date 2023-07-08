mod api;
mod chemin;
mod donnees;
mod interne;
mod outils;

fn main() {
    let configuration = match donnees::config::charger_config() {
        Ok(configuration) => configuration,
        Err(erreur) => {
            eprintln!("ERREUR: {:?}", erreur);
            return;
        },
    };
    println!("Configuration: {:?}", configuration);

    println!("Jeux: {:?}", outils::scan::trouver_jeux());

    api::igdb::screenshots();

}
