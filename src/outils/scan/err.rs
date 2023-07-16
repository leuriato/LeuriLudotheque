use crate::{interne::erreurs::TraitErreur, api::{igdb::err::Erreur as ErreurIGDB, openai::err::Erreur as ErreurGPT}};

// Erreur Identification
pub struct ErreurIdentification {
    pub erreur: Option<String>,
    pub desc: &'static str,
    pub chemin: std::path::PathBuf,
}

impl ErreurIdentification {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurIdentification(self))
    }
}

impl TraitErreur for ErreurIdentification {
    fn message(&self) -> String {
        format!("Impossible d'identifier le jeu {}: {}", self.chemin.display(), self.desc)
    }

    fn cause(&self) -> Option<String> {
        self.erreur.clone()
    }
}

// Erreur Identification IGDB
pub struct ErreurIdentificationIGDB {
    pub erreur: ErreurIGDB,
    pub chemin_str: String
}
impl ErreurIdentificationIGDB {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurIdentificationIGDB(self))
    }
}

impl TraitErreur for ErreurIdentificationIGDB {
    fn message(&self) -> String{
        format!("Impossible d'identifier le jeu {} avec l'API IGDB.", self.chemin_str)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.to_string()))
    }
}

// Erreur Traduction
pub struct ErreurTraduction {
    pub erreur: ErreurGPT,
}

impl ErreurTraduction {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurTraduction(self))
    }
}

impl TraitErreur for ErreurTraduction {
    fn message(&self) -> String {
        format!("Impossible de traduire le jeu.")
    }
}

// Enum Erreur
pub enum Erreur {
    ErreurIdentification(ErreurIdentification),
    ErreurIdentificationIGDB(ErreurIdentificationIGDB),
    ErreurTraduction(ErreurTraduction)
}

impl Erreur {
    fn as_trait(&self) -> &dyn TraitErreur {
        match self {
            Erreur::ErreurIdentification(erreur) => erreur,
            Erreur::ErreurIdentificationIGDB(erreur) => erreur,
            Erreur::ErreurTraduction(erreur) => erreur,
        }
    }
}

impl TraitErreur for Erreur {
    fn message(&self) -> String {
        self.as_trait().message()
    }

    fn cause(&self) -> Option<String> {
        self.as_trait().cause()
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

