use crate::api::igdb::objet::*;

use super::{err::*, obtenir_db};
use sqlx::Row;

#[async_trait::async_trait]
pub trait CompatibleSQL: Sized {
    fn commande_enregistrer(&self) -> String;

    fn commande_traduire(&self) -> String;

    fn commande_charger(id: u32) -> String;

    fn commande_charger_traduit(id: u32) -> String;

    async fn charger(id: u32) -> Result<Option<Self>, Erreur>;
}

fn guillemeter(input: String) -> String {
    format!("'{}'", input.replace("'", "\\'"))
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name, slug, updated_at)
            FROM collections WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for FranchiseIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name, slug, updated_at)
            FROM franchises WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for CategorieJeuIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name)
            FROM categories_jeu WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for CouvertureIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        CouvertureIGDB::commande_charger(id)
    }

    async fn charger(id: u32) -> Result<Option<CouvertureIGDB>, Erreur> {
        match sqlx::query_as::<_, CouvertureIGDB>(
            &CouvertureIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "couverture", id }.as_err(),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for JeuIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (
                id,
                COALESCE(name_traduit, name) AS name,
                slug,

                COALESCE(storyline_traduit, storyline) AS storyline,
                COALESCE(summary_traduit, summary) AS summary,

                first_release_date,

                collection,
                franchise,
                category,

                rating,
                rating_count,

                cover,

                updated_at)
            FROM jeux WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for GenreIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name, slug, updated_at)
            FROM genres WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for ThemeIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name, slug, updated_at)
            FROM themes WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for MotCleIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name, slug, updated_at)
            FROM mots_cles WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for IllustrationIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        IllustrationIGDB::commande_charger(id)
    }

    async fn charger(id: u32) -> Result<Option<IllustrationIGDB>, Erreur> {
        match sqlx::query_as::<_, IllustrationIGDB>(
            &IllustrationIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "illustration", id }.as_err(),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for CaptureEcranIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        CaptureEcranIGDB::commande_charger(id)
    }

    async fn charger(id: u32) -> Result<Option<CaptureEcranIGDB>, Erreur> {
        match sqlx::query_as::<_, CaptureEcranIGDB>(
            &CaptureEcranIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "capture d'écran", id }.as_err(),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for VideoIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name, video_id) FROM videos WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for CategoriePlateformeIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (id, COALESCE(name_traduit, name) AS name)
            FROM categories_plateforme WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for LogoPlateformeIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        LogoPlateformeIGDB::commande_charger(id)
    }

    async fn charger(id: u32) -> Result<Option<LogoPlateformeIGDB>, Erreur> {
        match sqlx::query_as::<_, LogoPlateformeIGDB>(
            &LogoPlateformeIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "logo plateforme", id }.as_err(),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for PlateformeIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (
                id,
                COALESCE(name_traduit, name) AS name,
                slug,

                COALESCE(storyline_traduit, storyline) AS storyline,
                COALESCE(summary_traduit, summary) AS summary,

                category,

                platform_logo,

                updated_at)
            FROM plateformes WHERE "id" = {}
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
}

#[async_trait::async_trait]
impl CompatibleSQL for LogoEntrepriseIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        LogoEntrepriseIGDB::commande_charger(id)
    }

    async fn charger(id: u32) -> Result<Option<LogoEntrepriseIGDB>, Erreur> {
        match sqlx::query_as::<_, LogoEntrepriseIGDB>(
            &LogoEntrepriseIGDB::commande_charger(id)
        ).fetch_optional(&obtenir_db().await?).await {
            Ok(valeur) => Ok(valeur),
            Err(erreur) => ErreurChargementImpossible { erreur, objet: "logo entreprise", id }.as_err(),
        }
    }
}

#[async_trait::async_trait]
impl CompatibleSQL for EntrepriseIGDB {
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

    fn commande_charger_traduit(id: u32) -> String {
        format!(
            r#"
            SELECT (
                id,
                COALESCE(name_traduit, name) AS name,
                slug,

                COALESCE(description_traduit, description) AS description,

                parent,

                logo,

                start_date,

                updated_at)
            FROM entreprises WHERE "id" = {}
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
}


