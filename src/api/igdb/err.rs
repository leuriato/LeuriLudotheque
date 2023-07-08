use crate::interne::erreurs::TraitErreur;
use crate::chemin::json;

// Erreur ClientInaccessible
pub struct ErreurClientInaccessible {
    pub erreur: json::err::Erreur,
}

impl ErreurClientInaccessible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ClientInaccessible(self))
    }
}

impl TraitErreur for ErreurClientInaccessible {
    fn message(&self) -> String {
        format!("Client IGDB manquant.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Erreur ClientInvalide
pub struct ErreurClientInvalide {
    pub erreur: json::err::Erreur,
}

impl ErreurClientInvalide {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ClientInvalide(self))
    }
}

impl TraitErreur for ErreurClientInvalide {
    fn message(&self) -> String {
        format!("Client IGDB invalide.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Erreur DemandeToken
pub struct ErreurDemandeToken {
    pub erreur: reqwest::Error,
}

impl ErreurDemandeToken {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurDemandeToken(self))
    }
}

impl TraitErreur for ErreurDemandeToken {
    fn message(&self) -> String {
        format!("La demande du token IGDB à échouer.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur RecuperationToken
pub struct ErreurRecuperationToken {
    pub erreur: serde_json::Error,
}

impl ErreurRecuperationToken {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurRecuperationToken(self))
    }
}

impl TraitErreur for ErreurRecuperationToken {
    fn message(&self) -> String {
        format!("Impossible de récupérer le token IGDB.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur EnregistrementExpire
pub struct ErreurEnregistrementExpire {
    pub erreur: json::err::Erreur,
}

impl ErreurEnregistrementExpire {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurEnregistrementExpire(self))
    }
}

impl TraitErreur for ErreurEnregistrementExpire {
    fn message(&self) -> String {
        format!("Impossible d'enregistrer l'expiration du token IGDB.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur.as_string()))
    }
}

// Enum Erreur
pub enum Erreur{
    ClientInaccessible(ErreurClientInaccessible),
    ClientInvalide(ErreurClientInvalide),
    ErreurDemandeToken(ErreurDemandeToken),
    ErreurRecuperationToken(ErreurRecuperationToken),
    ErreurEnregistrementExpire(ErreurEnregistrementExpire),
}

impl Erreur {
    fn as_trait(&self) -> &dyn TraitErreur {
        match self {
            Erreur::ClientInaccessible(erreur) => erreur,
            Erreur::ClientInvalide(erreur) => erreur,
            Erreur::ErreurDemandeToken(erreur) => erreur,
            Erreur::ErreurRecuperationToken(erreur) => erreur,
            Erreur::ErreurEnregistrementExpire(erreur) => erreur,
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

