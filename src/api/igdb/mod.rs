pub mod err;
pub mod requete;
pub mod objet;

use crate::api::igdb::err::*;
use crate::api::igdb::requete::*;
use crate::chemin::{json, chemins};
use crate::interne::erreurs::TraitErreur;

use async_std::task;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Client {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Token {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Expire {
    pub expires_at: u64,
}

fn client_modele() -> Client {
    Client {
        client_id: format!("your_client_id"),
        client_secret: format!("your_client_secret"),
    }
}

fn charger_client() -> Result<Client, json::err::Erreur> {
   json::charger_json_xdg(format!("igdb.json"), chemins::XDG::CONFIG)
}

fn enregistrer_client(client: Client) -> Result<Client, json::err::Erreur> {
    json::enregistrer_json_xdg(client, format!("igdb.json"), chemins::XDG::CONFIG)
}

fn charger_token() -> Result<Token, json::err::Erreur> {
    json::charger_json_xdg(format!("token.json"), chemins::XDG::CACHE)
}

fn obtenir_client() -> Result<Client, Erreur> {
    match charger_client() {
        Ok(client) => Ok(client),
        Err(json::err::Erreur::JsonInvalide(erreur)) =>
            ErreurClientInvalide {
                erreur: json::err::Erreur::JsonInvalide(erreur),
            }.as_err(),
        Err(json::err::Erreur::ChargementImpossible(erreur)) => {
            println!("INFO: Client IGDB manquant; création d'un modèle à remplir.");
            let _ = enregistrer_client(client_modele());
            ErreurClientInaccessible {
                erreur: json::err::Erreur::ChargementImpossible(erreur),
            }.as_err()
        },
        Err(erreur) => ErreurClientInaccessible { erreur }.as_err(),
    }
}

fn charger_expire() -> Result<Expire, json::err::Erreur> {
    json::charger_json_xdg(String::from("expire.json"), chemins::XDG::CACHE)
}

fn enregistrer_expire(expire: Expire) -> Result<Expire, json::err::Erreur> {
    json::enregistrer_json_xdg(expire, String::from("expire.json"), chemins::XDG::CACHE)
}

fn determiner_expire(token: &Token) -> Expire {
    let expires_at_systime: SystemTime = SystemTime::now() + Duration::from_secs(token.clone().expires_in);
    let expires_at: u64 = match expires_at_systime.duration_since(UNIX_EPOCH) {
        Ok(valeur) => valeur.as_secs(),
        Err(erreur) => {
            eprintln!("ERREUR: Impossible de déterminer l'expiration du token.\nRaison: {}", erreur);
            0u64
        }
    };

    Expire {
        expires_at,
    }
}

async fn demander_token() -> Result<String, Erreur> {
    let client: Client = obtenir_client()?;

    let url = format!(
        "https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
        client.client_id,
        client.client_secret,
    );

    let webclient = reqwest::Client::new();

    let reponse = match webclient.post(url).send().await {
        Ok(resultat) => resultat.text().await,
        Err(erreur) => return ErreurDemandeToken { erreur }.as_err(),
    };

    match reponse {
        Ok(valeur) => Ok(valeur),
        Err(erreur) => ErreurDemandeToken { erreur }.as_err(),
    }
}

async fn recuperer_token() -> Result<Token, Erreur> {
    let text = match demander_token().await {
        Ok(valeur) => valeur,
        Err(erreur) => return Err(erreur),
    };

    let token: Token = match serde_json::from_str(&text) {
        Ok(token) => token,
        Err(erreur) => return ErreurRecuperationToken { erreur }.as_err(),
    };

    let expire: Expire = determiner_expire(&token);

    match enregistrer_expire(expire) {
        Ok(_) => println!("INFO: Délai d'expiration du token IGDB enregistré."),
        Err(erreur) => {
            ErreurEnregistrementExpire { erreur }.afficher_attention();
        },
    }

    match enregistrer_token(token.clone()) {
        Ok(_) => println!("INFO: Token IGDB enregistré."),
        Err(erreur) => erreur.afficher_attention(),
    }

    Ok(token)
}

fn enregistrer_token(token: Token) -> Result<Token, json::err::Erreur> {
    json::enregistrer_json_xdg(token, String::from("token.json"), chemins::XDG::CACHE)
}

fn verifier_token() -> bool {
    let maintenant: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(valeur) => valeur.as_secs(),
        Err(_) => 0u64,
    };

    let expiration = match charger_expire() {
        Ok(expire) => expire.expires_at,
        Err(_) => return false,
    };

    if maintenant > expiration {
        false
    } else {
        true
    }
}

async fn obtenir_token() -> Result<Token, Erreur> {
    if verifier_token() {
        match charger_token() {
            Ok(token) => Ok(token),
            Err(_) => recuperer_token().await,
        }
    } else {
        recuperer_token().await
    }
}

pub struct ClientIGDB {
    client_id: String,
    access_token: String,
}

impl ClientIGDB {
    pub async fn new() -> Result<ClientIGDB, Erreur> {
        let client_id = obtenir_client()?.client_id;
        let access_token = obtenir_token().await?.access_token;
        Ok(ClientIGDB { client_id, access_token })
    }

    pub async fn demander(&self, endpoint: String, corps: String) -> Result<String, Erreur> {
        let url = format!("https://api.igdb.com/v4/{}", endpoint);

        let client = reqwest::Client::new();

        let client_id = match reqwest::header::HeaderValue::from_str(self.client_id.as_str()) {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurConstructionRequete { erreur }.as_err(),
        };
        let access_token = match reqwest::header::HeaderValue::from_str(format!("Bearer {}", self.access_token).as_str()) {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurConstructionRequete { erreur }.as_err(),
        };

        let mut entete = reqwest::header::HeaderMap::new();
        entete.insert("Client-ID", client_id);
        entete.insert("Authorization", access_token);
        entete.insert("Accept", reqwest::header::HeaderValue::from_static("application/json"));

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
        let reponse = self.demander(requete.endpoint, requete.corps).await?;

        let resultat: T = match serde_json::from_str(&reponse) {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurTraitementRequete { erreur, reponse }.as_err(),
        };

        Ok(resultat)
    }
}

