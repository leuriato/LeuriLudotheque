use crate::api::igdb::objet::*;

use super::{err::*, obtenir_db};
use sqlx::{Row, Pool, Sqlite};

#[async_trait::async_trait]
pub trait CompatibleSQL: Sized {
    fn table() -> &'static str;

    fn commande_enregistrer(&self) -> String;
    fn commande_traduire(&self) -> String;
    fn commande_charger(id: u32) -> String;

    async fn existe(id: u32) -> Result<bool, Erreur> {
        match sqlx::query(
            &format!("SELECT * FROM {} WHERE \"jeu\" = {};", Self::table(), id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur.is_some()),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: Self::table(), id }.as_err(),
        }
    }

    async fn supprimer(id: u32) -> Result<(), Erreur> {
        match sqlx::query(
            &format!("DELETE FROM {} WHERE \"jeu\" = {};", Self::table(), id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(_) => Ok(()),
            Err(erreur) => ErreurSuppressionImpossible { erreur, objet: Self::table(), id }.as_err(),
        }
    }

    async fn charger(id: u32) -> Result<Option<Self>, Erreur>;
    async fn charger_traduit(id: u32) -> Result<Option<Self>, Erreur>;

    async fn enregistrer(&self) -> Result<(), Erreur> {
        match sqlx::query(
            &self.commande_enregistrer()
        ).execute(&obtenir_db().await?).await {
            Ok(_) => Ok(()),
            Err(erreur) => ErreurEnregistrementImpossible { erreur, objet: Self::table() }.as_err(),
        }
    }
}

fn guillemeter(input: String) -> String {
    format!("'{}'", input.replace("'", "\\'"))
}

fn determiner(val_traduit: Option<String>, val: String) -> String {
    match val_traduit {
        Some(valeur) => valeur,
        None => val,
    }
}

trait ValeurSQL {
    fn convertir(&self) -> String;
}

impl ValeurSQL for String {
    fn convertir(&self) -> String {
        guillemeter(self.clone())
    }
}

impl ValeurSQL for u8 {
    fn convertir(&self) -> String {
        format!("{}", self)
    }
}

impl ValeurSQL for u32 {
    fn convertir(&self) -> String {
        format!("{}", self)
    }
}

impl ValeurSQL for i64 {
    fn convertir(&self) -> String {
        format!("{}", self)
    }
}

impl ValeurSQL for f32 {
    fn convertir(&self) -> String {
        format!("{}", self)
    }
}

impl ValeurSQL for f64 {
    fn convertir(&self) -> String {
        format!("{}", self)
    }
}

impl ValeurSQL for bool {
    fn convertir(&self) -> String {
        format!("{}", self).to_uppercase()
    }
}

impl<T: ValeurSQL> ValeurSQL for Option<T> {
    fn convertir(&self) -> String {
        match self {
            Some(valeur) => valeur.convertir(),
            None => "NULL".to_string(),
        }
    }
}

impl ValeurSQL for CollectionIGDB {
    fn convertir(&self) -> String {
        format!("{}", self.id)
    }
}

impl ValeurSQL for FranchiseIGDB {
    fn convertir(&self) -> String {
        format!("{}", self.id)
    }
}

impl ValeurSQL for CouvertureIGDB {
    fn convertir(&self) -> String {
        format!("{}", self.id)
    }
}

impl ValeurSQL for LogoPlateformeIGDB {
    fn convertir(&self) -> String {
        format!("{}", self.id)
    }
}

impl ValeurSQL for LogoEntrepriseIGDB {
    fn convertir(&self) -> String {
        format!("{}", self.id)
    }
}

async fn charger_vec_id<T: CompatibleSQL>(
    db: &sqlx::Pool<sqlx::Sqlite>,
    id: u32, table: &'static str,
    champ: &str
) -> Result<Option<Vec<u32>>, Erreur> {
    let mut liste: Vec<u32> = vec![];
    let res = match sqlx::query(
        &format!("SELECT * FROM {} WHERE \"jeu\" = {};", &table, id)
    ).fetch_all(db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurChargementImpossible { erreur, objet: table, id }.as_err(),
    };
    for ligne in res.iter() {
        match ligne.get::<Option<u32>, &str>(champ) {
            Some(id) => liste.push(id),
            None => {},
        }
    }
    if liste.len() > 0 {
        Ok(Some(liste))
    } else {
        Ok(None)
    }
}

async fn charger_vec<T: CompatibleSQL>(
    db: &sqlx::Pool<sqlx::Sqlite>,
    id: u32, table: &'static str,
    champ: &str
) -> Result<Option<Vec<T>>, Erreur> {
    let mut liste: Vec<T> = vec![];
    let res = match sqlx::query(
        &format!("SELECT * FROM {} WHERE \"jeu\" = {};", &table, id)
    ).fetch_all(db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurChargementImpossible { erreur, objet: table, id }.as_err(),
    };
    for ligne in res.iter() {
        match ligne.get::<Option<u32>, &str>(champ) {
            Some(id) => match T::charger(id).await? {
                Some(valeur) => liste.push(valeur),
                None => {},
            },
            None => {},
        }
    }
    if liste.len() > 0 {
        Ok(Some(liste))
    } else {
        Ok(None)
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for CollectionIGDB {
    fn table() -> &'static str {
        "collections"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO collections ("id", "name", "name_traduit", "slug", "updated_at")
            VALUES ({}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),
            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE collections SET "name_traduit" = {} WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM collections WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<CollectionIGDB>, Erreur> {
        match sqlx::query_as::<_, CollectionIGDB>(
            &CollectionIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "collection", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<CollectionIGDB>, Erreur> {
        match CollectionIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                CollectionIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for FranchiseIGDB {
    fn table() -> &'static str {
        "franchises"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO franchises ("id", "name", "name_traduit", "slug", "updated_at")
            VALUES ({}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),
            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE franchises SET "name_traduit" = {} WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM franchises WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<FranchiseIGDB>, Erreur> {
        match sqlx::query_as::<_, FranchiseIGDB>(
            &FranchiseIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "franchise", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<FranchiseIGDB>, Erreur> {
        match FranchiseIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                FranchiseIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for CategorieJeuIGDB {
    fn table() -> &'static str {
        "categories_jeu"
    }

    fn commande_enregistrer(&self) -> String {
        println!("ATTENTION: Impossible d'enregistrer une catégorie de jeu.");
        format!("")
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire une catégorie de jeu.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM categories_jeu WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<CategorieJeuIGDB>, Erreur> {
        match sqlx::query_as::<_, CategorieJeuIGDB>(
            &CategorieJeuIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "categorie jeu", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<CategorieJeuIGDB>, Erreur> {
        match CategorieJeuIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                CategorieJeuIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for CouvertureIGDB {
    fn table() -> &'static str {
        "couvertures"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO couvertures ("id", "url", "width", "height")
            VALUES ({}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.url.convertir(),
            self.width.convertir(),
            self.height.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire une couverture.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM collections WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<CouvertureIGDB>, Erreur> {
        match sqlx::query_as::<_, CouvertureIGDB>(
            &CouvertureIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "couverture", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<CouvertureIGDB>, Erreur> {
        Ok(CouvertureIGDB::charger(id).await?)
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for JeuIGDB {
    fn table() -> &'static str {
        "jeux"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO jeux
                ("id",
                 "name",
                 "name_traduit",
                 "slug",

                 "storyline",
                 "storyline_traduit",
                 "summary",
                 "summary_traduit",

                 "first_release_date",

                 "collection",
                 "franchise",
                 "category",

                 "rating",
                 "rating_count",

                 "cover",

                 "updated_at")
            VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),

            self.storyline.convertir(),
            self.storyline_traduit.convertir(),
            self.summary.convertir(),
            self.summary_traduit.convertir(),

            self.first_release_date.convertir(),

            self.collection.convertir(),
            self.franchise.convertir(),
            self.category.convertir(),

            self.rating.convertir(),
            self.rating_count.convertir(),

            self.cover.convertir(),

            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE jeux SET
                "name_traduit" = {},
                "storyline_traduit" = {},
                "summary_traduit" = {}
            WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.storyline.convertir(),
            self.summary.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM jeux WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<JeuIGDB>, Erreur> {
        let db = obtenir_db().await?;
        let resultat = match sqlx::query(
            &JeuIGDB::commande_charger(id)
        ).fetch_optional(&db).await {
            Ok(Some(valeur)) => valeur,
            Ok(None) => return Ok(None),
            Err(erreur) => return ErreurChargementImpossible { erreur, objet: "jeu", id}.as_err(),
        };

        let genres = charger_vec::<GenreIGDB>(&db, id, "jeux_genres", "genre");
        let themes = charger_vec::<ThemeIGDB>(&db, id, "jeux_themes", "theme");
        let keywords = charger_vec::<MotCleIGDB>(&db, id, "jeux_mots_cles", "mot_cle");

        let platforms = charger_vec_id::<PlateformeIGDB>(&db, id, "jeux_plateformes", "plateforme");

        let remakes = charger_vec_id::<JeuIGDB>(&db, id, "jeux_remakes", "remake");
        let remasters = charger_vec_id::<JeuIGDB>(&db, id, "jeux_remasters", "remaster");
        let similar_games = charger_vec_id::<JeuIGDB>(&db, id, "jeux_similaires", "jeu_similaire");

        let artworks = charger_vec::<IllustrationIGDB>(&db, id, "jeux_illustrations", "illustration");
        let screenshots = charger_vec::<CaptureEcranIGDB>(&db, id, "jeux_captures_ecran", "capture_ecran");
        let videos = charger_vec::<VideoIGDB>(&db, id, "jeux_videos", "video");

        Ok(Some(JeuIGDB {
            id,
            name: resultat.get("name"),
            slug: resultat.get("slug"),
            name_traduit: resultat.get("name_traduit"),
            alternative_names: None,
            storyline: resultat.get("storyline"),
            summary: resultat.get("summary"),
            storyline_traduit: resultat.get("storyline_traduit"),
            summary_traduit: resultat.get("summary_traduit"),

            first_release_date: resultat.get("first_release_date"),

            collection: match resultat.get("collection") {
                Some(id) => CollectionIGDB::charger(id).await?,
                None => None,
            },
            franchise: match resultat.get("franchise"){
                Some(id) => FranchiseIGDB::charger(id).await?,
                None => None,
            },
            category: resultat.get("category"),

            genres: genres.await?,
            themes: themes.await?,
            keywords: keywords.await?,

            platforms: platforms.await?,

            remakes: remakes.await?,
            remasters: remasters.await?,
            similar_games: similar_games.await?,

            rating: resultat.get("rating"),
            rating_count: resultat.get("rating_count"),

            cover: match resultat.get("cover") {
                Some(id) => CouvertureIGDB::charger(id).await?,
                None => None,
            },
            artworks: artworks.await?,
            screenshots: screenshots.await?,
            videos: videos.await?,

            updated_at: resultat.get("updated_at")
        }))
    }

    async fn charger_traduit(id: u32) -> Result<Option<JeuIGDB>, Erreur> {
        match JeuIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                JeuIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    alternative_names: None,

                    summary: Some(determiner(valeur.summary_traduit, valeur.summary.unwrap_or(String::new()))),
                    summary_traduit: None,
                    storyline: Some(determiner(valeur.storyline_traduit, valeur.storyline.unwrap_or(String::new()))),
                    storyline_traduit: None,

                    first_release_date: valeur.first_release_date,

                    collection: valeur.collection,
                    franchise: valeur.franchise,
                    category: valeur.category,

                    genres: valeur.genres,
                    themes: valeur.themes,
                    keywords: valeur.keywords,

                    platforms: valeur.platforms,

                    remakes: valeur.remakes,
                    remasters: valeur.remasters,
                    similar_games: valeur.similar_games,

                    rating: valeur.rating,
                    rating_count: valeur.rating_count,

                    cover: valeur.cover,
                    artworks: valeur.artworks,
                    screenshots: valeur.screenshots,
                    videos: valeur.videos,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }

    async fn enregistrer(&self) -> Result<(), Erreur> {
        async fn supprimer(db: &Pool<Sqlite>, id: u32, table: &str) -> Result<(), Erreur> {
            match sqlx::query(
                &format!(r#"DELETE * FROM {} WHERE "jeu" = {};"#, &table, id)
            ).execute(db).await {
                Ok(_) => Ok(()),
                Err(erreur) => ErreurSQL { erreur, desc: "la suppression d'un jeu dans une table de correspondance"}.as_err(),
            }
        }

        async fn inserer(db: &Pool<Sqlite>, id: u32, liste: Vec<u32>, table: &str, champ: &str) -> Result<(), Erreur> {
            for valeur in liste {
                match sqlx::query(
                    &format!(
                        r#"
                        INSERT INTO {} ("jeu", "{}") VALUES ({}, {});
                        "#,
                        &table,
                        &champ,
                        id,
                        valeur
                    )
                ).execute(db).await {
                    Ok(_) => {},
                    Err(erreur) => return ErreurSQL { erreur, desc: "la suppression d'un jeu dans une table de correspondance"}.as_err(),
                }
            }
            Ok(())
        }

        let db = obtenir_db().await?;

        let genres = supprimer(&db, self.id, "jeux_genres");
        let themes = supprimer(&db, self.id, "jeux_themes");
        let mots_cles = supprimer(&db, self.id, "jeux_mots_cles");

        let remakes = supprimer(&db, self.id, "jeux_remakes");
        let remasters = supprimer(&db, self.id, "jeux_remasters");
        let jeux_similaires = supprimer(&db, self.id, "jeux_similaires");

        let plateformes = supprimer(&db, self.id, "jeux_plateformes");

        let artworks = supprimer(&db, self.id, "jeux_illustrations");
        let screenshots= supprimer(&db, self.id, "jeux_captures_ecran");
        let videos = supprimer(&db, self.id, "jeux_videos");

        genres.await?;
        let liste: Vec<u32> = self.genres.clone().unwrap_or(vec![]).iter().map(|x| x.id).collect();
        let genres = inserer(&db, self.id, liste, "jeux_genres", "genre");

        themes.await?;
        let liste: Vec<u32> = self.themes.clone().unwrap_or(vec![]).iter().map(|x| x.id).collect();
        let themes = inserer(&db, self.id, liste, "jeux_themes", "theme");

        mots_cles.await?;
        let liste: Vec<u32> = self.keywords.clone().unwrap_or(vec![]).iter().map(|x| x.id).collect();
        let mots_cles = inserer(&db, self.id, liste, "jeux_mots_cles", "mot_cle");

        remakes.await?;
        let remakes = inserer(&db, self.id, self.remakes.clone().unwrap_or(vec![]), "jeux_remakes", "remake");

        remasters.await?;
        let remasters = inserer(&db, self.id, self.remasters.clone().unwrap_or(vec![]), "jeux_remasters", "remaster");

        jeux_similaires.await?;
        let jeux_similaires = inserer(&db, self.id, self.similar_games.clone().unwrap_or(vec![]), "jeux_similaires", "jeu_similaire");

        plateformes.await?;
        let plateformes = inserer(&db, self.id, self.platforms.clone().unwrap_or(vec![]), "jeux_genres", "genre");

        artworks.await?;
        let liste: Vec<u32> = self.artworks.clone().unwrap_or(vec![]).iter().map(|x| x.id).collect();
        let artworks = inserer(&db, self.id, liste, "jeux_illustrations", "illustration");

        screenshots.await?;
        let liste: Vec<u32> = self.screenshots.clone().unwrap_or(vec![]).iter().map(|x| x.id).collect();
        let screenshots = inserer(&db, self.id, liste, "jeux_captures_ecran", "capture_ecran");

        videos.await?;
        let liste: Vec<u32> = self.videos.clone().unwrap_or(vec![]).iter().map(|x| x.id).collect();
        let videos = inserer(&db, self.id, liste, "jeux_videos", "videos");

        CompatibleSQL::enregistrer(self).await?;

        genres.await?;
        themes.await?;
        mots_cles.await?;
        remakes.await?;
        remasters.await?;
        jeux_similaires.await?;
        plateformes.await?;
        artworks.await?;
        screenshots.await?;
        videos.await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for GenreIGDB {
    fn table() -> &'static str {
        "genres"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO genres ("id", "name", "name_traduit", "slug", "updated_at")
            VALUES ({}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),
            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE genres SET "name_traduit" = {} WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM genres WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<GenreIGDB>, Erreur> {
        match sqlx::query_as::<_, GenreIGDB>(
            &GenreIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "genre", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<GenreIGDB>, Erreur> {
        match GenreIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                GenreIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for ThemeIGDB {
    fn table() -> &'static str {
        "themes"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO themes ("id", "name", "name_traduit", "slug", "updated_at")
            VALUES ({}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),
            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE themes SET "name_traduit" = {} WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM themes WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<ThemeIGDB>, Erreur> {
        match sqlx::query_as::<_, ThemeIGDB>(
            &ThemeIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "theme", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<ThemeIGDB>, Erreur> {
        match ThemeIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                ThemeIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for MotCleIGDB {
    fn table() -> &'static str {
        "mots_cles"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO mots_cles ("id", "name", "name_traduit", "slug", "updated_at")
            VALUES ({}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),
            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE mots_cles SET "name_traduit" = {} WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM mots_cles WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<MotCleIGDB>, Erreur> {
        match sqlx::query_as::<_, MotCleIGDB>(
            &MotCleIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "mot clé", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<MotCleIGDB>, Erreur> {
        match MotCleIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                MotCleIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for IllustrationIGDB {
    fn table() -> &'static str {
        "illustrations"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO illustrations ("id", "url", "width", "height")
            VALUES ({}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.url.convertir(),
            self.width.convertir(),
            self.height.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire une illustration.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM illustrations WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<IllustrationIGDB>, Erreur> {
        match sqlx::query_as::<_, IllustrationIGDB>(
            &IllustrationIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "illustration", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<IllustrationIGDB>, Erreur> {
        Ok(IllustrationIGDB::charger(id).await?)
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for CaptureEcranIGDB {
    fn table() -> &'static str {
        "captures_ecran"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO captures_ecran ("id", "url", "width", "height")
            VALUES ({}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.url.convertir(),
            self.width.convertir(),
            self.height.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire une capture d'écran.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM captures_ecran WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<CaptureEcranIGDB>, Erreur> {
        match sqlx::query_as::<_, CaptureEcranIGDB>(
            &CaptureEcranIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "capture d'écran", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<CaptureEcranIGDB>, Erreur> {
        Ok(CaptureEcranIGDB::charger(id).await?)
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for VideoIGDB {
    fn table() -> &'static str {
        "videos"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO videos ("id", "name", "name_traduit", "video_id")
            VALUES ({}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.video_id.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE videos SET "name_traduit" = {} WHERE "id" = {};
            "#,
            self.name_traduit.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM videos WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<VideoIGDB>, Erreur> {
        match sqlx::query_as::<_, VideoIGDB>(
            &VideoIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "vidéo", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<VideoIGDB>, Erreur> {
        match VideoIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                VideoIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    video_id: valeur.video_id,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for CategoriePlateformeIGDB {
    fn table() -> &'static str {
        "categories_plateforme"
    }

    fn commande_enregistrer(&self) -> String {
        println!("ATTENTION: Impossible d'enregistrer une catégorie de plateforme.");
        format!("")
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire une catégorie de plateforme.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM categories_plateforme WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<CategoriePlateformeIGDB>, Erreur> {
        match sqlx::query_as::<_, CategoriePlateformeIGDB>(
            &CategoriePlateformeIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "catégorie plateforme", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<CategoriePlateformeIGDB>, Erreur> {
        match CategoriePlateformeIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                CategoriePlateformeIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for LogoPlateformeIGDB {
    fn table() -> &'static str {
        "logos_plateforme"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO logos_plateforme ("id", "url", "width", "height")
            VALUES ({}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.url.convertir(),
            self.width.convertir(),
            self.height.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire le logo d'une plateforme.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM logos_plateforme WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<LogoPlateformeIGDB>, Erreur> {
        match sqlx::query_as::<_, LogoPlateformeIGDB>(
            &LogoPlateformeIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "logo plateforme", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<LogoPlateformeIGDB>, Erreur> {
        Ok(LogoPlateformeIGDB::charger(id).await?)
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for PlateformeIGDB {
    fn table() -> &'static str {
        "plateformes"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO plateformes
                ("id",
                 "name",
                 "name_traduit",
                 "slug",

                 "summary",
                 "summary_traduit",

                 "category",

                 "platform_logo",

                 "updated_at")
            VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),

            self.summary.convertir(),
            self.summary_traduit.convertir(),

            self.category.convertir(),

            self.platform_logo.convertir(),

            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE plateformes SET
                "name_traduit" = {},
                "summary_traduit" = {}
            WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.summary.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM plateformes WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<PlateformeIGDB>, Erreur> {
        let db = obtenir_db().await?;
        let resultat = match sqlx::query(
            &PlateformeIGDB::commande_charger(id)
        ).fetch_optional(&db).await {
            Ok(Some(valeur)) => valeur,
            Ok(None) => return Ok(None),
            Err(erreur) => return ErreurChargementImpossible { erreur, objet: "plateforme", id}.as_err(),
        };

        Ok(Some(PlateformeIGDB {
            id,
            name: resultat.get("name"),
            slug: resultat.get("slug"),
            name_traduit: resultat.get("name_traduit"),

            summary: resultat.get("summary"),
            summary_traduit: resultat.get("summary_traduit"),

            category: resultat.get("category"),

            platform_logo: match resultat.get("platform_logo") {
                Some(id) => LogoPlateformeIGDB::charger(id).await?,
                None => None,
            },

            updated_at: resultat.get("updated_at"),
        }))
    }

    async fn charger_traduit(id: u32) -> Result<Option<PlateformeIGDB>, Erreur> {
        match PlateformeIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                PlateformeIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    summary: Some(determiner(valeur.summary_traduit, valeur.summary.unwrap_or(String::new()))),
                    summary_traduit: None,

                    category: valeur.category,

                    platform_logo: valeur.platform_logo,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for LogoEntrepriseIGDB {
    fn table() -> &'static str {
        "logos_entreprise"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO logos_entreprise ("id", "url", "width", "height")
            VALUES ({}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.url.convertir(),
            self.width.convertir(),
            self.height.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        println!("ATTENTION: Impossible de traduire le logo d'une entreprise.");
        format!("")
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM logos_entreprise WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<LogoEntrepriseIGDB>, Erreur> {
        match sqlx::query_as::<_, LogoEntrepriseIGDB>(
            &LogoEntrepriseIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "logo entreprise", id }.as_err(),
        }
    }

    async fn charger_traduit(id: u32) -> Result<Option<LogoEntrepriseIGDB>, Erreur> {
        Ok(LogoEntrepriseIGDB::charger(id).await?)
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for EntrepriseIGDB {
    fn table() -> &'static str {
        "entreprises"
    }

    fn commande_enregistrer(&self) -> String {
        format!(
            r#"
            INSERT INTO entreprises
                ("id",
                 "name",
                 "name_traduit",
                 "slug",

                 "description",
                 "description_traduit",

                 "parent",

                 "logo",

                 "start_date",

                 "updated_at")
            VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {}, {});
            "#,
            self.id.convertir(),
            self.name.convertir(),
            self.name_traduit.convertir(),
            self.slug.convertir(),

            self.description.convertir(),
            self.description_traduit.convertir(),

            self.parent.convertir(),

            self.logo.convertir(),

            self.start_date.convertir(),

            self.updated_at.convertir(),
        )
    }

    fn commande_traduire(&self) -> String {
        format!(
            r#"
            UPDATE entreprises SET
                "name_traduit" = {},
                "description_traduit" = {}
            WHERE "id" = {};
            "#,
            self.name.convertir(),
            self.description.convertir(),
            self.id.convertir(),
        )
    }

    fn commande_charger(id: u32) -> String {
        format!(
            r#"
            SELECT * FROM entreprises WHERE "id" = {}
            "#,
            id.convertir(),
        )
    }

    async fn charger(id: u32) -> Result<Option<EntrepriseIGDB>, Erreur> {
        let db = obtenir_db().await?;
        let resultat = match sqlx::query(
            &EntrepriseIGDB::commande_charger(id)
        ).fetch_optional(&db).await {
            Ok(Some(valeur)) => valeur,
            Ok(None) => return Ok(None),
            Err(erreur) => return ErreurChargementImpossible { erreur, objet: "entreprise", id}.as_err(),
        };

        let mut developed: Vec<u32> = vec![];
        let mut published: Vec<u32> = vec![];
        let res = match sqlx::query(
            &format!("SELECT * FROM jeux_entreprises WHERE \"entreprise\" = {};", id)
        ).fetch_all(&db).await {
            Ok(valeur) => valeur,
            Err(erreur) => return ErreurChargementImpossible { erreur, objet: "jeux_entreprises", id }.as_err(),
        };
        for ligne in res.iter() {
            match ligne.get::<Option<u32>, &str>("jeu") {
                Some(id) => {
                    match ligne.get::<Option<bool>, &str>("developed") {
                        Some(true) => developed.push(id),
                        _ => {},
                    }
                    match ligne.get::<Option<bool>, &str>("published") {
                        Some(true) => published.push(id),
                        _ => {},
                    }
                }
                None => {},
            }
        }

        Ok(Some(EntrepriseIGDB {
            id,
            name: resultat.get("name"),
            slug: resultat.get("slug"),
            name_traduit: resultat.get("name_traduit"),

            developed: Some(developed),
            published: Some(published),

            description: resultat.get("description"),
            description_traduit: resultat.get("description_traduit"),

            parent: resultat.get("parent"),

            logo: match resultat.get("logo") {
                Some(id) => LogoEntrepriseIGDB::charger(id).await?,
                None => None,
            },

            start_date: resultat.get("start_date"),

            updated_at: resultat.get("updated_at"),
        }))
    }

    async fn charger_traduit(id: u32) -> Result<Option<EntrepriseIGDB>, Erreur> {
        match EntrepriseIGDB::charger(id).await? {
            Some(valeur) => Ok(Some(
                EntrepriseIGDB {
                    id,
                    name: determiner(valeur.name_traduit, valeur.name),
                    name_traduit: None,
                    slug: valeur.slug,

                    published: valeur.published,
                    developed: valeur.developed,

                    description: Some(determiner(valeur.description_traduit, valeur.description.unwrap_or(String::new()))),
                    description_traduit: None,

                    parent: valeur.parent,

                    logo: valeur.logo,

                    start_date: valeur.start_date,

                    updated_at: valeur.updated_at,
                }
            )),
            None => Ok(None),
        }
    }
}


