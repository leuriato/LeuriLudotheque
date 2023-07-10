use std::path::Path;
use igdb::model::games::Game;
use crate::donnees::config;

use crate::api::igdb::err::*;
use crate::chemin::{json, chemins};
use crate::interne::erreurs::TraitErreur;

use async_std::task;
use gtk::gdk::keys::constants::w;
use igdb::client::IGDBClient;
use igdb::media_quality::MediaQuality;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Jeu {
    pub igdb_id: Option<String>,
    pub igdb_jeu: Option<igdb::model::games::Game>,
    pub chemin: Option<String>,
    pub image: Option<String>,
}

fn extensions_valables(config: &config::LudothequeConfig) -> Vec<String> {
    let mut ext: Vec<String> = vec![];

    for emulateur in &config.emulateurs {
        ext.extend(emulateur.extensions.clone());
    }

    ext
}

fn enfants(dossier: String) -> Vec<String> {
    match std::fs::read_dir(Path::new(&dossier)) {
        Ok(valeur) => {
            let mut ret: Vec<String> = vec![];
            for v in valeur {
                if v.is_ok() {
                    ret.push(v.unwrap().path().into_os_string().into_string().unwrap())
                }
            }
            ret
        },
        Err(erreur) => {
            eprintln!("ATTENTION: {}", erreur);
            vec![]
        }
    }
}

fn fini_par(chemin: String, ext: String) -> bool {
    /*let chemin_str: String = match chemin.into_os_string().into_string() {
        Ok(valeur) => valeur,
        Err(erreur) => {
            eprintln!("ATTENTION: {:?}", erreur);
            return false;
        }
    };*/

    let n = chemin.len();
    let l = ext.len();

    if n > l {
        return chemin[n-l..n] == ext;
    } else {
        return false;
    }
}


pub fn trouver_jeux() -> Vec<PathBuf> {
    let config = config::obtenir_config();

    let ext = extensions_valables(&config);

    let dossier: String = match config.repertoire_jeux {
        Some(repertoire) => repertoire.into_os_string().into_string().unwrap(),
        None => return vec![],
    };

    let mut queue: Vec<String> = enfants(dossier);

    let mut jeux: Vec<PathBuf> = vec![];

    while queue.len() > 0 {
        let chemin: String = match queue.pop() {
            Some(valeur) => valeur,
            None => continue,
        };

        if Path::new(&chemin).is_dir() {
            queue.extend(enfants(chemin.clone()));
        } else {
            for extension in &ext {
                if fini_par(chemin.clone(), format!(".{}", extension)) {
                    jeux.push(PathBuf::from(chemin.clone()));
                }
            }
        }
    }

    jeux
}
/*
pub fn identifier_jeu(chemin: PathBuf) -> Result<Jeu, Erreur> {
    let nom: String = match chemin.as_path().file_prefix() {
        Some(os_valeur) => match os_valeur.to_str() {
            Some(valeur) => Valeur,
            None => return Err(...),
        },
        None => return Err(...),
    };

    task::block_on(async {
        let igdb_client = obtenir_igdb_client()?;
        let games_client = igdb_client.games();
        let jeu_igdb = games_client.get_first_by_name(&nom).await;

*/
