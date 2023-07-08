use crate::interne::erreurs::TraitErreur;
use crate::chemin::chemins;

use std::path::PathBuf;

// Erreur Serialisation
pub struct ErreurSerialisation {
    pub fichier: PathBuf,
    pub erreur: serde_json::Error,
}

impl ErreurSerialisation {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurSerialisation(self))
    }
}

impl TraitErreur for ErreurSerialisation {
    fn message(&self) -> String {
        format!("Impossible de sérialiser {:?}.", self.fichier)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Permission
pub struct ErreurPermission {
    pub fichier: PathBuf,
    pub erreur: std::io::Error,
}

impl ErreurPermission {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurPermission(self))
    }
}

impl TraitErreur for ErreurPermission {
    fn message(&self) -> String {
        format!("Impossible d'écrire dans {:?}.", self.fichier)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur JsonInvalide
pub struct ErreurJsonInvalide {
    pub fichier: PathBuf,
    pub erreur: serde_json::Error,
}

impl ErreurJsonInvalide {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::JsonInvalide(self))
    }
}

impl TraitErreur for ErreurJsonInvalide {
    fn message(&self) -> String {
        format!("Le fichier JSON {:?} est invalide.", self.fichier)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur EnregistrementImpossible
pub struct ErreurEnregistrementImpossible {
    pub fichier: String,
    pub erreur: chemins::err::Erreur,
}

impl ErreurEnregistrementImpossible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::EnregistrementImpossible(self))
    }
}

impl TraitErreur for ErreurEnregistrementImpossible {
    fn message(&self) -> String {
        format!("Impossible d'enregistrer {:?}.", self.fichier)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Erreur ChargementImpossible
pub struct ErreurChargementImpossible {
    pub fichier: String,
    pub erreur: chemins::err::Erreur,
}

impl ErreurChargementImpossible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ChargementImpossible(self))
    }
}

impl TraitErreur for ErreurChargementImpossible {
    fn message(&self) -> String {
        format!("Impossible de charger {:?}.", self.fichier)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Enum Erreur
pub enum Erreur {
    ErreurPermission(ErreurPermission),
    ErreurSerialisation(ErreurSerialisation),
    JsonInvalide(ErreurJsonInvalide),
    EnregistrementImpossible(ErreurEnregistrementImpossible),
    ChargementImpossible(ErreurChargementImpossible),
}

impl Erreur {
    fn as_trait(&self) -> &dyn TraitErreur {
        match self {
            Erreur::ErreurPermission(erreur) => erreur,
            Erreur::ErreurSerialisation(erreur) => erreur,
            Erreur::JsonInvalide(erreur) => erreur,
            Erreur::EnregistrementImpossible(erreur) => erreur,
            Erreur::ChargementImpossible(erreur) => erreur,
        }
    }
}

impl TraitErreur for Erreur {
    fn message(&self) -> String {
        self.as_trait().message()
    }
}

impl std::fmt::Display for Erreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl std::fmt::Debug for Erreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

