//use serde::{Deserialize, de::DeserializeOwned};

use crate::api::openai::objet;

fn obtenir_modele() -> String {
    "gpt-3.5-turbo".to_string()
}

fn obtenir_temperature() -> f32 {
    0.2
}

#[derive(Debug)]
pub struct Requete<T> {
    pub corps: String,
    resultat: Option<T>,
}

pub fn requete(
    systeme: String,
    utilisateur: String,
) -> Requete<objet::ReponseGPT> {

    Requete {
        corps: format!(
            "{{
                \"model\": {:?},
                \"messages\": [
                    {{\"role\": \"system\", \"content\": {:?}}},
                    {{\"role\": \"user\", \"content\": {:?}}}
                ],
                \"temperature\": {}
            }}",
            obtenir_modele(),
            systeme,
            utilisateur,
            obtenir_temperature(),
        ),
        resultat: None,
    }

}
