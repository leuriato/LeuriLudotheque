pub mod err;

use crate::api::openai::objet::ReponseGPT;
use crate::donnees::igdb::extra::obtenir_catalogue;
use crate::donnees::igdb::obtenir_db;
use crate::donnees::igdb::interface::CompatibleSQL;
use crate::interne::erreurs::TraitErreur;
use crate::outils::scan::err::*;
use crate::donnees::{config, objet::*};
use crate::api::{igdb::*, openai};
use crate::outils::trad::igdb::Traduisible;

use std::path::{Path, PathBuf};

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
    let limite = match config.profondeur_recherche {
        Some(l) => l,
        None => 1,
    };

    let dossier: String = match config.repertoire_jeux {
        Some(repertoire) => repertoire.into_os_string().into_string().unwrap(),
        None => return vec![],
    };

    let mut queue: Vec<String> = enfants(dossier);
    let mut profondeurs: Vec<u32> = vec![0; queue.len()];

    let mut jeux: Vec<PathBuf> = vec![];

    while queue.len() > 0 {
        let chemin: String = match queue.pop() {
            Some(valeur) => valeur,
            None => continue,
        };
        let p = profondeurs.pop().unwrap();

        if Path::new(&chemin).is_dir() && p < limite {
            let enfants = enfants(chemin.clone());
            profondeurs.extend(vec![p+1; enfants.len()]);
            queue.extend(enfants);
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

pub async fn identifier_jeu(chemin: PathBuf, db: &sqlx::Pool<sqlx::Sqlite>, traduire: bool) -> Result<(), Erreur> {
    let chemin_str: String = match chemin.as_path().as_os_str().to_str() {
        Some(valeur) => String::from(valeur),
        None => return ErreurIdentification {
                chemin,
                desc: "Impossible de convertir le chemin en String",
                erreur: None
            }.as_err(),
    };

    let mut id_jeu: Option<u32> = None;

    if Jeu::existe_db(chemin_str.clone(), db).await.unwrap() {
        if let Some(id) = Jeu::charger_db(chemin_str.clone(), db).await.unwrap().unwrap().jeu {
            if JeuIGDB::existe_db(id, db).await.unwrap() {
                println!("{} déjà enregistré.", chemin_str);
                return Ok(());
            } else {
                id_jeu = Some(id);
            }
        }
    }

    let nom_fichier: String = match chemin_str.rfind(std::path::MAIN_SEPARATOR_STR) {
        Some(debut) => String::from(&chemin_str[debut + 1..chemin_str.len()]),
        None => chemin_str.clone(),
    };

    let droite = chemin_str.rfind("]");
    let gauche = match droite {
        Some(indice) => chemin_str[0..indice].rfind("["),
        None => None,
    };

    let meta = match gauche {
        Some(gauche_unwrap) => String::from(&chemin_str[gauche_unwrap+1..droite.unwrap()]),
        None => String::new(),
    };


    let mut langue: Option<String> = None;

    if meta.contains(",") {
        let liste: Vec<&str> = meta.split(',').collect();
        for e in liste {
            match e.trim().parse::<u32>() {
                Ok(v) => id_jeu = Some(v),
                Err(_) => langue = Some(String::from(e)),
            }
        }
    } else {
        match meta.trim().parse::<u32>() {
            Ok(v) => id_jeu = Some(v),
            Err(_) => langue = Some(meta)
        }
    }

    let jeu_igdb: JeuIGDB;
    let mut nom_jeu: String;

    if id_jeu.is_none() {
        let mut nom = String::from(&nom_fichier);

        if gauche.is_some() &&  nom.len() >= chemin_str.len() - gauche.unwrap() {
            nom = format!("{}{}", &nom[0..nom.len() + gauche.unwrap() - chemin_str.len()], &nom[nom.len() + droite.unwrap() - chemin_str.len() + 1..nom.len()])
        }

        nom = match nom.find(".") {
            Some(indice) => String::from(&nom[0..indice]),
            None => nom,
        };

        nom = String::from(nom.trim());

        let client = match ClientIGDB::new().await {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurIdentificationIGDB{ erreur, chemin_str }.as_err() ,
        };

        let resultat: Vec<JeuIGDB> = match client.solliciter(requete::requete_trouver_jeu(nom.clone(), None)).await {
            Ok(resultat) => resultat,
            Err(erreur) => return ErreurIdentificationIGDB { erreur, chemin_str }.as_err(),
        };

        if resultat.len() == 0 {
            return ErreurIdentification { chemin, desc: "Pas de resultat correspondant au jeu.", erreur: None}.as_err();
        }

        jeu_igdb = resultat[0].clone();
        std::thread::sleep(std::time::Duration::from_millis(250))
    } else if id_jeu.unwrap() > 0 {
        let client = match ClientIGDB::new().await {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurIdentificationIGDB{ erreur, chemin_str }.as_err() ,
        };

        let resultat: Vec<JeuIGDB> = match client.solliciter(requete::requete_recuperer_jeu(id_jeu.unwrap())).await {
            Ok(resultat) => resultat,
            Err(erreur) => return ErreurIdentificationIGDB { erreur, chemin_str }.as_err(),
        };

        if resultat.len() == 0 {
            return ErreurIdentification { chemin, desc: "Jeu inexistant (identifiant invalide).", erreur: None}.as_err();
        }

        jeu_igdb = resultat[0].clone();
        std::thread::sleep(std::time::Duration::from_millis(250))
    } else {
        jeu_igdb = JeuIGDB::charger_db(0, db).await.unwrap().unwrap();
    }

    match jeu_igdb.enregistrer_db(db).await {
        Ok(_) => {},
        Err(erreur) => return ErreurIdentification { chemin, desc: "Impossible d'enregistrer le jeu.", erreur: Some(erreur.to_string()) }.as_err(),
    }

    nom_jeu = jeu_igdb.name.clone();

    if traduire {
        let prompt = match jeu_igdb.prompt() {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurIdentification { chemin, desc: "Impossible construire le prompt.", erreur: Some(erreur.to_string()) }.as_err(),
        };

        let client = match openai::ClientOpenAI::new().await {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurTraduction { erreur }.as_err(),
        };

        let requete = openai::requete::requete("You do what the user asks and only return json files.".to_string(), prompt);

        let json = match client.solliciter(requete).await {
            Ok(reponse) => reponse.choices[0].clone().message.content,
            Err(erreur) => return ErreurIdentification { chemin, desc: "Erreur pendant la requete à OpenAI.", erreur: Some(erreur.to_string()) }.as_err(),
        };

        let jeu_traduit: JeuIGDB = match serde_json::from_str(&json) {
            Ok(jeu) => jeu,
            Err(erreur) => return ErreurIdentification { chemin, desc: "ChatGPT a répondu n'importe quoi.", erreur: Some(erreur.to_string()) }.as_err(),
        };

        match jeu_traduit.traduire_db(db).await {
            Ok(_) => {},
            Err(erreur) => return ErreurIdentification { chemin, desc: "Impossible d'enregistrer le jeu traduit.", erreur: Some(erreur.to_string()) }.as_err(),
        }

        nom_jeu = jeu_traduit.name.clone();
    }
    if jeu_igdb.id == 0 {
        let mut nom = String::from(nom_fichier);

        if gauche.is_some() &&  nom.len() >= chemin_str.len() - gauche.unwrap() {
            nom = format!("{}{}", &nom[0..nom.len() + gauche.unwrap() - chemin_str.len()], &nom[nom.len() + droite.unwrap() - chemin_str.len() + 1..nom.len()])
        }

        nom = match nom.find(".") {
            Some(indice) => String::from(&nom[0..indice]),
            None => nom,
        };

        nom_jeu = String::from(nom.trim());
    }

    let jeu = Jeu { jeu: Some(jeu_igdb.id), chemin: chemin_str.clone(), nom: nom_jeu, langue: langue.unwrap_or(String::new()).to_uppercase() };

    match jeu.enregistrer_db(db).await {
        Ok(_) => {},
        Err(erreur) => return ErreurIdentification { chemin, desc: "Impossible d'enregistrer le jeu dans le catalogue.", erreur: Some(erreur.to_string()) }.as_err(),
    }

    println!("{} enregistré.", chemin_str);

    Ok(())
}

pub async fn nettoyer_catalogue() {
    let jeux = obtenir_catalogue().await;

    let db = obtenir_db().await.unwrap();
    for jeu in jeux {
        if !std::path::Path::new(&jeu.chemin).exists() {
            println!("{} supprimé.", &jeu.chemin);
            let _ = Jeu::supprimer_db(jeu.chemin, &db).await;
        }
    }
    db.close().await;
}

pub async fn scanner() {
    let nettoyage = nettoyer_catalogue();

    let liste = trouver_jeux();
    nettoyage.await;

    let db = obtenir_db().await.unwrap();
    for chemin in liste {
        //println!("{chemin:?}");
        match identifier_jeu(chemin, &db, false).await {
            Ok(_) => {},
            Err(erreur) => erreur.afficher_erreur(),
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    db.close().await;
}
