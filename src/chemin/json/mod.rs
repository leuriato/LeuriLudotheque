pub mod err;

use crate::chemin::json::err::*;
use crate::chemin::chemins::{determiner_chemin, trouver_chemin, XDG};

use serde::{Serialize, de::DeserializeOwned};
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

pub fn enregistrer_json<T: Serialize>(json: T, chemin_json: PathBuf) -> Result<T, Erreur> {
    let json_serialise = match serde_json::to_string(&json) {
        Ok(valeur) => valeur,
        Err(erreur) => return Err(
            Erreur::ErreurSerialisation(ErreurSerialisation { fichier: chemin_json, erreur })
        ),
    };

    let mut fichier_json = match File::create(chemin_json.as_path()) {
        Ok(valeur) => valeur,
        Err(erreur) => return Err(
            Erreur::ErreurPermission(ErreurPermission { fichier: chemin_json, erreur })
        ),
    };

    match fichier_json.write_all(json_serialise.as_bytes()) {
        Ok(_) => {},
        Err(erreur) => return Err(
            Erreur::ErreurPermission(ErreurPermission { fichier: chemin_json, erreur })
        ),
    }

    println!("INFO: {:?} enregistré.", &chemin_json);

    Ok(json)
}

pub fn enregistrer_json_xdg<T: Serialize>(json: T, nom: String, xdg_type: XDG) -> Result<T, Erreur> {
    match determiner_chemin(nom.clone(), xdg_type) {
        Ok(chemin) => enregistrer_json(json, chemin),
        Err(erreur) => Err(
            Erreur::EnregistrementImpossible(
                ErreurEnregistrementImpossible { fichier: nom, erreur }
            )
        ),
    }
}

pub fn charger_json<T: DeserializeOwned>(chemin_json: PathBuf) -> Result<T, Erreur> {
    let json_serialise = match std::fs::read_to_string(chemin_json.as_path()) {
        Ok(json_serialise) => json_serialise,
        Err(erreur) => return Err(
            Erreur::ErreurPermission(ErreurPermission { fichier: chemin_json, erreur })
        ),
    };

    match serde_json::from_str(&json_serialise) {
        Ok(json) => return Ok(json),
        Err(erreur) => return Err(
            Erreur::JsonInvalide(ErreurJsonInvalide { fichier: chemin_json, erreur })
        ),
    };
}

pub fn charger_json_xdg<T: DeserializeOwned>(nom: String, xdg_type: XDG) -> Result<T, Erreur> {
    match trouver_chemin(nom.clone(), xdg_type) {
        Ok(chemin) => charger_json(chemin),
        Err(erreur) => Err(
            Erreur::ChargementImpossible(ErreurChargementImpossible { fichier: nom, erreur })
        ),
    }
}

