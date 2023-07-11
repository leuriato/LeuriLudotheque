pub mod err;
pub mod requete;
pub mod objet;

use crate::api::openai::err::*;
use crate::api::openai::requete::*;
use crate::chemin::{json, chemins};

use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Token {
    pub secret_key: String,
}

fn token_modele() -> Token {
    Token {
        secret_key: format!("your_openai_api_key"),
    }
}

fn charger_token() -> Result<Token, json::err::Erreur> {
    json::charger_json_xdg(format!("openai.json"), chemins::XDG::CONFIG)
}

fn obtenir_token() -> Result<Token, Erreur> {
    match charger_token() {
        Ok(token) => Ok(token),
        Err(json::err::Erreur::JsonInvalide(erreur)) =>
            ErreurTokenInvalide {
                erreur: json::err::Erreur::JsonInvalide(erreur),
            }.as_err(),
        Err(json::err::Erreur::ChargementImpossible(erreur)) => {
            println!("INFO: Token OpenAI manquant; création d'un modèle à remplir.");
            let _ = enregistrer_token(token_modele());
            ErreurTokenInaccessible {
                erreur: json::err::Erreur::ChargementImpossible(erreur),
            }.as_err()
        },
        Err(erreur) => ErreurTokenInaccessible { erreur }.as_err(),
    }
}

fn enregistrer_token(token: Token) -> Result<Token, json::err::Erreur> {
    json::enregistrer_json_xdg(token, String::from("openai.json"), chemins::XDG::CONFIG)
}

pub struct ClientOpenAI {
    secret_key: String,
}

impl ClientOpenAI {
    pub async fn new() -> Result<ClientOpenAI, Erreur> {
        let secret_key = obtenir_token()?.secret_key;
        Ok(ClientOpenAI { secret_key })
    }

    pub async fn demander(&self, corps: String) -> Result<String, Erreur> {
        let url = format!("https://api.openai.com/v1/chat/completions");

        let client = reqwest::Client::new();

        let secret_key = match reqwest::header::HeaderValue::from_str(format!("Bearer {}", self.secret_key).as_str()) {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurConstructionRequete { erreur }.as_err(),
        };

        let mut entete = reqwest::header::HeaderMap::new();
        entete.insert("Authorization", secret_key);
        entete.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json"));

        let reponse = match client
            .post(url)
            .headers(entete)
            .body(corps)
            .send()
            .await {
            Ok(resultat) => resultat.text().await,
            Err(erreur) => return ErreurDemandeRequete { erreur }.as_err(),
        };

        match reponse {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurDemandeRequete { erreur }.as_err(),
        }
    }

    pub async fn solliciter<T: DeserializeOwned>(&self, requete: Requete<T>) -> Result<T, Erreur> {
        let reponse = self.demander(requete.corps).await?;

        let resultat: T = match serde_json::from_str(&reponse) {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurTraitementRequete { erreur, reponse }.as_err(),
        };

        Ok(resultat)
    }
}

