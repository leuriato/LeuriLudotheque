use crate::{chemin::{json, chemins}, interne::erreurs::TraitErreur};

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use xdg::BaseDirectories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LudothequeConfig {
    pub utilisateurs: Vec<Utilisateur>,
    pub emulateurs: Vec<Emulateur>,
    pub repertoire_jeux: Option<PathBuf>,
    pub repertoire_donnees: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utilisateur {
    pub id: u32,
    pub nom: String,
    pub pseudonyme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emulateur {
    pub nom: String,
    pub commande: String,
    pub extensions: Vec<String>,
}

pub fn config_par_defaut() -> LudothequeConfig {
    let xdg_dirs = BaseDirectories::with_prefix("ludotheque")
        .expect("Impossible de trouver les répertoires XDG.");

    LudothequeConfig {
        utilisateurs:
            vec![
                Utilisateur{id: 0, nom: String::from("Utilisateur 0"), pseudonyme: String::from("user0")}
            ],
        emulateurs: vec![],
        repertoire_jeux: dirs::home_dir(),
        repertoire_donnees: Some(xdg_dirs.get_data_home()),
    }
}

pub fn creer_config() -> Result<LudothequeConfig, json::err::Erreur> {
    json::enregistrer_json_xdg(config_par_defaut(), String::from("config.json"), chemins::XDG::CONFIG)
}

pub fn charger_config() -> Result<LudothequeConfig, json::err::Erreur> {
    json::charger_json_xdg(String::from("config.json"), chemins::XDG::CONFIG)
}

pub fn obtenir_config() -> LudothequeConfig {
    match charger_config() {
        Ok(configuration) => configuration,
        Err(erreur) => {
            erreur.afficher_attention();
            println!("INFO: Utilisation de la configuration par défaut.");
            config_par_defaut()
        },
    }
}

