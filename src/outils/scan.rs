use std::path::Path;
use igdb::model::games::Game;
use crate::donnees::config;

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


pub fn trouver_jeux() -> Vec<Jeu> {
    let config = config::obtenir_config();

    let ext = extensions_valables(&config);

    let dossier: String = match config.repertoire_jeux {
        Some(repertoire) => repertoire.into_os_string().into_string().unwrap(),
        None => return vec![],
    };

    let mut queue: Vec<String> = enfants(dossier);

    let mut jeux: Vec<Jeu> = vec![];

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
                    jeux.append(
                         &mut vec![Jeu {
                            igdb_id: None,
                            igdb_jeu: None,
                            chemin: Some(chemin.clone()),
                            image: None,
                        }
                    ]);
                }
            }
        }
    }

    jeux
}

pub fn scanner_jeux() {

}























