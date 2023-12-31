use crate::interne::erreurs::TraitErreur;
use crate::chemin::chemins;

// Erreur LocalisationDB
pub struct ErreurLocalisationDB {
    pub erreur: chemins::err::Erreur,
}

impl ErreurLocalisationDB {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurLocalisationDB(self))
    }
}

impl TraitErreur for ErreurLocalisationDB {
    fn message(&self) -> String {
        format!("Impossible de determiner le chemin de \"games.db\".")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Accès DB
pub struct ErreurAccesDB {
    pub erreur: sqlx::Error,
}

impl ErreurAccesDB {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurAccesDB(self))
    }
}

impl TraitErreur for ErreurAccesDB {
    fn message(&self) -> String{
        format!("Impossible d'accéder à \"games.db\".")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Création DB
pub struct ErreurCreationDB {
    pub erreur: sqlx::Error,
}

impl ErreurCreationDB {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurCreationDB(self))
    }
}

impl TraitErreur for ErreurCreationDB {
    fn message(&self) -> String {
        format!("Impossible de creer la base de données \"games.db\".")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur SQL
pub struct ErreurSQL {
    pub erreur: sqlx::Error,
    pub desc: &'static str,
}

impl ErreurSQL {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurSQL(self))
    }
}

impl TraitErreur for ErreurSQL {
    fn message(&self) -> String {
        format!("Une erreur est survenue lors de {}.", self.desc)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Chargement Impossible
pub struct ErreurChargementImpossible {
    pub erreur: sqlx::Error,
    pub objet: &'static str,
    pub id: u32,
}

impl ErreurChargementImpossible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ChargementImpossible(self))
    }
}

impl TraitErreur for ErreurChargementImpossible {
    fn message(&self) -> String {
        format!("Impossible de charger {} id: {}.", self.objet, self.id)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Enregistrement Impossible
pub struct ErreurEnregistrementImpossible {
    pub erreur: sqlx::Error,
    pub objet: &'static str,
}

impl ErreurEnregistrementImpossible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::EnregistrementImpossible(self))
    }
}

impl TraitErreur for ErreurEnregistrementImpossible {
    fn message(&self) -> String {
        format!("Impossible d'enregistrer {}.", self.objet)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Suppression Impossible
pub struct ErreurSuppressionImpossible {
    pub erreur: sqlx::Error,
    pub objet: &'static str,
    pub id: u32,
}

impl ErreurSuppressionImpossible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::SuppressionImpossible(self))
    }
}

impl TraitErreur for ErreurSuppressionImpossible {
    fn message(&self) -> String {
        format!("Impossible de supprimer {} id: {}.", self.objet, self.id)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur Traduction Impossible
pub struct ErreurTraductionImpossible {
    pub erreur: sqlx::Error,
    pub objet: &'static str,
}

impl ErreurTraductionImpossible {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::TraductionImpossible(self))
    }
}

impl TraitErreur for ErreurTraductionImpossible {
    fn message(&self) -> String {
        format!("Impossible de traduire {}.", self.objet)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Enum Erreur
pub enum Erreur {
    ErreurLocalisationDB(ErreurLocalisationDB),
    ErreurAccesDB(ErreurAccesDB),
    ErreurCreationDB(ErreurCreationDB),
    ErreurSQL(ErreurSQL),
    ChargementImpossible(ErreurChargementImpossible),
    EnregistrementImpossible(ErreurEnregistrementImpossible),
    SuppressionImpossible(ErreurSuppressionImpossible),
    TraductionImpossible(ErreurTraductionImpossible),
}

impl Erreur {
    fn as_trait(&self) -> &dyn TraitErreur {
        match self {
            Erreur::ErreurLocalisationDB(erreur) => erreur,
            Erreur::ErreurAccesDB(erreur) => erreur,
            Erreur::ErreurCreationDB(erreur) => erreur,
            Erreur::ErreurSQL(erreur) => erreur,
            Erreur::ChargementImpossible(erreur) => erreur,
            Erreur::EnregistrementImpossible(erreur) => erreur,
            Erreur::SuppressionImpossible(erreur) => erreur,
            Erreur::TraductionImpossible(erreur) => erreur,
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

