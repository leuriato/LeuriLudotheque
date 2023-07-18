use std::process::Command;

use crate::donnees::config::obtenir_config;

fn obtenir_commande(chemin: String) -> String {
    let config = obtenir_config();

    for emulateur in &config.emulateurs {
        for extension in &emulateur.extensions {
            if chemin.ends_with(format!(".{}", extension).as_str()) {
                return emulateur.commande.clone();
            }
        }
    }

    String::from("echo Aucun émulateur trouvé.")
}

fn creer_commande(chemin: String) -> String {
    let mut commande = obtenir_commande(chemin.clone());
    let chemin_str = chemin.replace("'", "\\'");

    commande = commande.replace("{{chemin}}", "{{/!\\-*-tmp-*-/!\\}}");
    commande = commande.replace("{chemin}", format!("'{chemin_str}'").as_str());
    commande = commande.replace("{{/!\\-*-tmp-*-/!\\}}", "{{chemin}}");

    commande
}


#[cfg(target_os = "linux")]
fn appeler_commande(chemin: String) {
    Command::new("sh")
        .arg("-c")
        .arg(creer_commande(chemin.to_string()))
        .spawn()
        .expect("Le lancement du jeu a échoué.");
}

pub fn lancer_jeu(chemin: String) {
    appeler_commande(chemin);
}
