pub mod err;

use crate::donnees::config::obtenir_config;
use crate::outils::trad::igdb::err::*;
use crate::donnees::objet::*;

use serde::Serialize;

pub trait Traduisible where Self: Clone + Serialize {
    fn pre_prompt(&self) -> String {
        format!("")
    }

    fn preparer_traduction(&self) -> Self;

    fn prompt(&self) -> Result<String, Erreur> {
        let copie = &self.preparer_traduction();
        let json = match serde_json::to_string(&copie) {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurCreationPrompt { erreur }.as_err(),
        };

        Ok(format!(
            "{}:\n{}",
            self.pre_prompt(),
            json,
        ))
    }
}

impl Traduisible for JeuIGDB {
    fn pre_prompt(&self) -> String {
        let config = obtenir_config();

        format!(
            "Translate this game into {} (the name of the game should not be modified unless a {} altenative title is provided) and delete \"alternative_names\"",
            config.langue,
            config.langue,
        )
    }

    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.slug = None;
        copie.name_traduit = None;
        //copie.slug_traduit = None;
        if copie.alternative_names.is_some() {
            let vec = copie.alternative_names.to_owned().unwrap();
            for i in 0..vec.len() {
                copie.alternative_names.to_owned().unwrap()[i] = vec[i].preparer_traduction();
            }
        }
        copie.storyline_traduit = None;
        copie.summary_traduit = None;

        copie.first_release_date = None;

        if copie.collection.is_some() {
            copie.collection = Some(
                copie.collection
                    .to_owned()
                    .unwrap()
                    .preparer_traduction()
            );
        }
        if copie.franchise.is_some() {
            copie.franchise = Some(
                copie.franchise
                    .to_owned()
                    .unwrap()
                    .preparer_traduction()
            );
        }
        copie.category = None;

        if copie.genres.is_some() {
            let mut vec = copie.genres.to_owned().unwrap();
            for i in 0..vec.len() {
                vec[i] = vec[i].preparer_traduction();
            }
            copie.genres = Some(vec);
        }
        if copie.themes.is_some() {
            let mut vec = copie.themes.to_owned().unwrap();
            for i in 0..vec.len() {
                vec[i] = vec[i].preparer_traduction();
            }
            copie.themes = Some(vec);
        }
        if copie.keywords.is_some() {
            let mut vec = copie.keywords.to_owned().unwrap();
            for i in 0..vec.len() {
                vec[i] = vec[i].preparer_traduction();
            }
            copie.keywords = Some(vec);
        }

        copie.platforms = None;

        copie.remakes = None;
        copie.remasters = None;
        copie.similar_games = None;

        copie.rating = None;
        copie.rating_count = None;

        copie.cover = None;
        copie.artworks = None;
        copie.screenshots = None;
        if copie.videos.is_some() {
            let mut vec = copie.videos.to_owned().unwrap();
            for i in 0..vec.len() {
                vec[i] = vec[i].preparer_traduction();
            }
            copie.videos = Some(vec);
        }

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for NomAlternatifIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.game = None;

        copie
    }
}

impl Traduisible for CollectionIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.slug = None;
        copie.name_traduit = None;
        //copie.slug_traduit = None;

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for FranchiseIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.slug = None;
        copie.name_traduit = None;
        //copie.slug_traduit = None;

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for GenreIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.slug = None;
        copie.name_traduit = None;
        //copie.slug_traduit = None;

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for ThemeIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.name_traduit = None;
        copie.slug = None;
        //copie.slug_traduit = None;

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for MotCleIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.name_traduit = None;
        copie.slug = None;
        //copie.slug_traduit = None;

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for EntrepriseIGDB {
    fn pre_prompt(&self) -> String {
        let config = obtenir_config();

        format!("Translate this entreprise in {}", config.langue)
    }

    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.name_traduit = None;
        copie.slug = None;
        //copie.slug_traduit = None;

        copie.developed = None;
        copie.published = None;

        copie.description_traduit = None;

        copie.parent = None;

        copie.logo = None;

        copie.start_date = None;

        copie.updated_at = None;

        copie
    }
}

impl Traduisible for VideoIGDB {
    fn preparer_traduction(&self) -> Self {
        let mut copie = self.clone();
        copie.name_traduit = None;

        copie.video_id = None;

        copie
    }
}

