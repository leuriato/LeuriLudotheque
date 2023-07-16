use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Jeu {
    pub jeu: Option<u32>,
//    pub jeu: Option<JeuIGDB>,
    pub chemin: String,
    pub nom: String,
    pub langue: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct JeuIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,
    pub alternative_names: Option<Vec<NomAlternatifIGDB /*<'static>*/>>,
    pub storyline: Option<String>,
    pub summary: Option<String>,
    pub storyline_traduit: Option<String>,
    pub summary_traduit: Option<String>,

    pub first_release_date: Option<i64>,

    pub collection: Option<CollectionIGDB> /*<'static>*/,
    pub franchise: Option<FranchiseIGDB> /*<'static>*/,
    pub category: Option<u8>,

    pub genres: Option<Vec<GenreIGDB /*<'static>*/>>,
    pub themes: Option<Vec<ThemeIGDB /*<'static>*/>>,
    pub keywords: Option<Vec<MotCleIGDB /*<'static>*/>>,

    pub platforms: Option<Vec<u32>>,

    pub remakes: Option<Vec<u32>>,
    pub remasters: Option<Vec<u32>>,
    pub similar_games: Option<Vec<u32>>,

    pub rating: Option<f64>,
    pub rating_count: Option<u32>,

    pub cover: Option<CouvertureIGDB /*<'static>*/>,
    pub artworks: Option<Vec<IllustrationIGDB /*<'static>*/>>,
    pub screenshots: Option<Vec<CaptureEcranIGDB /*<'static>*/>>,
    pub videos: Option<Vec<VideoIGDB /*<'static>*/>>,

    pub updated_at: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct NomAlternatifIGDB /*<'static>*/ {
    pub id: u32,
    pub comment: Option<String>,
    pub name: String,
    pub game: Option<u32>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct CollectionIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,

    pub updated_at: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct FranchiseIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,

    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategorieJeuIGDB {
    pub id: u32,
    pub name: String,
    pub name_traduit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct GenreIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,

    pub updated_at: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct ThemeIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,

    pub updated_at: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct MotCleIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,

    pub updated_at: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct PlateformeIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,

    pub summary: Option<String>,
    pub summary_traduit: Option<String>,

    pub category: Option<u8>,

    pub platform_logo: Option<LogoPlateformeIGDB /*<'static>*/>,

    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategoriePlateformeIGDB {
    pub id: u32,
    pub name: String,
    pub name_traduit: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct LogoPlateformeIGDB /*<'static>*/ {
    pub id: u32,
    pub url: String,

    pub width: u32,
    pub height: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct EntrepriseIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub slug: Option<String>,
    pub name_traduit: Option<String>,
    //pub slug_traduit: Option<String>,


    pub developed: Option<Vec<u32>>,
    pub published: Option<Vec<u32>>,

    pub description: Option<String>,
    pub description_traduit: Option<String>,

    pub parent: Option<u32>,

    pub logo: Option<LogoEntrepriseIGDB /*<'static>*/>,

    pub start_date: Option<i64>,

    pub updated_at: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct LogoEntrepriseIGDB /*<'static>*/ {
    pub id: u32,
    pub url: String,

    pub width: u32,
    pub height: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct CouvertureIGDB /*<'static>*/ {
    pub id: u32,
    pub url: String,

    pub width: u32,
    pub height: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct IllustrationIGDB /*<'static>*/ {
    pub id: u32,
    pub url: String,

    pub width: u32,
    pub height: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct CaptureEcranIGDB /*<'static>*/ {
    pub id: u32,
    pub url: String,

    pub width: u32,
    pub height: u32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct VideoIGDB /*<'static>*/ {
    pub id: u32,
    pub name: String,
    pub name_traduit: Option<String>,
    pub video_id: Option<String>,
}

