use crate::interne::erreurs::TraitErreur;

// Erreur CréationPrompt
pub struct ErreurCreationPrompt {
    pub erreur: serde_json::Error,
}

impl ErreurCreationPrompt {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurCreationPrompt(self))
    }
}

impl TraitErreur for ErreurCreationPrompt {
    fn message(&self) -> String {
        format!("Une erreur est survenue lors de la création du prompt de traduction.")
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

/*/ Erreur Repertoire XDG
pub struct ErreurRepertoireXDG {
    pub xdg_type: XDG,
    pub erreur: std::io::Error,
}
impl ErreurRepertoireXDG {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::ErreurRepertoireXDG(self))
    }
}

impl TraitErreur for ErreurRepertoireXDG {
    fn message(&self) -> String{
        format!("Impossible d'accéder au répertoires XDG de type {:?}.", self.xdg_type)
    }

    fn cause(&self) -> Option<String> {
        Some(format!("{}", self.erreur))
    }
}

// Erreur FichierIntrouvable
pub struct ErreurFichierIntrouvable {
    pub xdg_type: XDG,
    pub nom: String,
}

impl ErreurFichierIntrouvable {
    pub fn as_err<T>(self) -> Result<T, Erreur> {
        Err(Erreur::FichierIntrouvable(self))
    }
}

impl TraitErreur for ErreurFichierIntrouvable {
    fn message(&self) -> String {
        format!("Impossible de trouver {} dans le répertoires XDG de type {:?}.", self.nom, self.xdg_type)
    }
}
*/
// Enum Erreur
pub enum Erreur {
    ErreurCreationPrompt(ErreurCreationPrompt),
}

impl Erreur {
    fn as_trait(&self) -> &dyn TraitErreur {
        match self {
            Erreur::ErreurCreationPrompt(erreur) => erreur,
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

