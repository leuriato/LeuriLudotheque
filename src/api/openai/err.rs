use crate::interne::erreurs::TraitErreur;
use crate::chemin::json;

// Erreur ClientInaccessible
pub struct ErreurTokenInaccessible {
    pub erreur: json::err::Erreur,
}

impl ErreurTokenInaccessible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::TokenInaccessible(self))
    }
}

impl TraitErreur for ErreurTokenInaccessible {
    fn message(&self) -> String {
        format!("Token OpenAI manquant.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Erreur ClientInvalide
pub struct ErreurTokenInvalide {
    pub erreur: json::err::Erreur,
}

impl ErreurTokenInvalide {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::TokenInvalide(self))
    }
}

impl TraitErreur for ErreurTokenInvalide {
    fn message(&self) -> String {
        format!("Token OpenAI invalide.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Erreur ConstructionRequete
pub struct ErreurConstructionRequete {
    pub erreur: reqwest::header::InvalidHeaderValue,
}

impl ErreurConstructionRequete {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurConstructionRequete(self))
    }
}

impl TraitErreur for ErreurConstructionRequete {
    fn message(&self) -> String {
        format!("Impossible de contruire la requête OpenAI.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur DemandeRequete
pub struct ErreurDemandeRequete {
    pub erreur: reqwest::Error,
}

impl ErreurDemandeRequete {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurDemandeRequete(self))
    }
}

impl TraitErreur for ErreurDemandeRequete {
    fn message(&self) -> String {
        format!("La requête OpenAI a échoué.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur TraitementRequete
pub struct ErreurTraitementRequete {
    pub erreur: serde_json::Error,
    pub reponse: String,
}

impl ErreurTraitementRequete {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurTraitementRequete(self))
    }
}

impl TraitErreur for ErreurTraitementRequete {
    fn message(&self) -> String {
        format!("Le traitement de la requête OpenAI a échoué.\nRéponse: {}", self.reponse)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}


// Enum Erreur
pub enum Erreur{
    TokenInaccessible(ErreurTokenInaccessible),
    TokenInvalide(ErreurTokenInvalide),
    ErreurConstructionRequete(ErreurConstructionRequete),
    ErreurDemandeRequete(ErreurDemandeRequete),
    ErreurTraitementRequete(ErreurTraitementRequete),
}

impl Erreur {
    fn as_trait(&self) -> &dyn TraitErreur {
        match self {
            Erreur::TokenInaccessible(erreur) => erreur,
            Erreur::TokenInvalide(erreur) => erreur,
            Erreur::ErreurConstructionRequete(erreur) => erreur,
            Erreur::ErreurDemandeRequete(erreur) => erreur,
            Erreur::ErreurTraitementRequete(erreur) => erreur,
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

