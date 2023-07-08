pub trait TraitErreur {
    fn message(&self) -> String;

    fn cause(&self) -> Option<String> {
        None
    }

    fn as_string(&self) -> String {
        match self.cause() {
            Some(cause) => format!("{}\nCause: {}", self.message(), cause),
            None => self.message(),
        }
    }

    fn afficher_erreur(&self) {
        eprintln!("ERREUR: {}", self.as_string());
    }

    fn afficher_attention(&self) {
        println!("ATTENTION: {}", self.as_string());
    }
}

impl std::fmt::Display for dyn TraitErreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl std::fmt::Debug for dyn TraitErreur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

