//use serde::{Deserialize, de::DeserializeOwned};

use crate::api::igdb::objet;

#[derive(Debug)]
pub struct Requete<T> {
    pub endpoint: String,
    pub corps: String,
    resultat: Option<T>,
}

pub fn requete_trouver_jeu(
    nom: String,
    plateforme_id: Option<u32>
) -> Requete<Vec<objet::JeuIGDB>> {

    Requete {
        endpoint: format!("games/"),
        corps: format!("{}{}{}{}",
            format!("search \"{}\";", nom),
            concat!(
                "fields name, slug, alternative_names.*, storyline, summary, first_release_date, ",
                "collection.*, franchise.*, category, genres.*, themes.*, keywords.*, platforms, ",
                "remakes, remasters, similar_games, rating, rating_count, cover.*, artworks.*, ",
                "screenshots.*, videos.*, updated_at;"
            ),
            "limit 1;",
            match plateforme_id {
                Some(id) => format!("where platforms = ({})", id),
                None => format!(""),
            }
        ),
        resultat: None,
    }

}
