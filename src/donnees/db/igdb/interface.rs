use sqlx::{Sqlite, Pool};

use crate::api::igdb::objet::*;
use crate::donnees::db::igdb::err::*;
use crate::donnees::db::igdb::obtenir_db_url;

#[async_trait::async_trait]
pub trait ObjetJeu<'a> {
    /*fn to_hashmap(&self) -> Result<HashMap<String, Value>, Erreur> {
        let json = match serde_json::to_string(self) {
            Ok(valeur) => valeur,
            Err(erreur) => ErreurConversionImpossible {
                erreur,
                from: "struct",
                to: "json (vers hashmap)",
                objet: Box::new(self),
            }.as_err(),
        };
        match serde_json::from_str(&json) {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurConversionImpossible {
                erreur,
                from: "json (depuis struct)",
                to: "hashmap",
                objet: Box::new(self),
            }.as_err(),
        }
    }*/

    //async fn enregistrer(&self) {}

    //async fn enregistrer_traduction(&self) {}

    fn commande_enregistrer(&self) -> String;

    //async fn charger(id: u32) -> Option<&'a Self>;

    //async fn charger_traduit(id: u32) -> Option<&'a Self>;
}

fn adapter(input: String) -> String {
    input.replace("'", "\\'")
}

#[async_trait::async_trait]
impl<'a> ObjetJeu<'a> for CollectionIGDB {
    fn commande_enregistrer(&self) -> String {
       sqlx::query!(
            "INSERT INTO \"collections\" (\"id\", \"name\", \"name_traduit\", \"slug\")
            VALUES (?, ?, ?, ?)",
            self.id,
            self.name,
            self.name_traduit,
            self.slug,
        )
        //"".to_string()
    }
}
