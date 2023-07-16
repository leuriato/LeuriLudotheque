pub mod err;
pub mod extra;
pub mod interface;

use sqlx::SqlitePool;
use sqlx::{Sqlite, Pool, migrate::MigrateDatabase};

use crate::donnees::igdb::err::*;
use crate::chemin::chemins::{determiner_chemin, XDG};

pub fn obtenir_db_url<'a>() -> Result<String, Erreur> {
    match determiner_chemin("games.db".to_string(), XDG::DATA) {
        Ok(chemin) => Ok(format!("sqlite://{}", chemin.display())),
        Err(erreur) => ErreurLocalisationDB { erreur }.as_err(),
    }
}

async fn db_existe() -> Result<bool, Erreur> {
    let db_url = obtenir_db_url()?;

    match Sqlite::database_exists(&db_url).await {
        Ok(valeur) => Ok(valeur),
        Err(erreur) => ErreurAccesDB { erreur }.as_err(),
    }
}

async fn obtenir_db() -> Result<Pool<Sqlite>, Erreur> {
    match SqlitePool::connect(&obtenir_db_url()?).await {
        Ok(db) => Ok(db),
        Err(erreur) => ErreurAccesDB { erreur }.as_err(),
    }
}

pub async fn creer_db() -> Result<(), Erreur> {
    let db_url = obtenir_db_url()?;

    if db_existe().await? {
        return Ok(());
    }

    println!("INFO: Création de la base de données {}.", db_url);

    match Sqlite::create_database(&db_url).await {
        Ok(_) => (),
        Err(erreur) => return ErreurCreationDB { erreur }.as_err(),
    }

    let db = obtenir_db().await?;

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS collections (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            updated_at INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table collections"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS franchises (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            updated_at INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table franchises"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS categories_jeu (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(50) NOT NULL,
            name_traduit VARCHAR(50)
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table categories_jeu"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS couvertures (
            id INTEGER PRIMARY KEY NOT NULL,
            url TEXT,

            width INTEGER,
            height INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table couvertures"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            storyline TEXT,
            storyline_traduit TEXT,
            summary TEXT,
            summary_traduit TEXT,

            first_release_date INTEGER,

            collection INTEGER,
            franchise INTEGER,
            category INTEGER,

            rating FLOAT,
            rating_count INTEGER,

            cover INTEGER,

            updated_at INTEGER
        );" /*
            FOREIGN KEY (collection) REFERENCES collections (id),
            FOREIGN KEY (franchise) REFERENCES franchises (id),
            FOREIGN KEY (category) REFERENCES categories_jeu (id),
            FOREIGN KEY (cover) REFERENCES couvertures (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS genres (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            updated_at INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table genres"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS themes (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            updated_at INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table themes"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS mots_cles (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            updated_at INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table mots_cles"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS illustrations (
            id INTEGER PRIMARY KEY NOT NULL,
            url TEXT,

            width INTEGER,
            height INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table illustrations"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS captures_ecran (
            id INTEGER PRIMARY KEY NOT NULL,
            url TEXT,

            width INTEGER,
            height INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table captures_ecran"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS videos (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),

            video_id VARCHAR(500)
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table videos"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_remakes (
            jeu INTEGER,
            remake INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (remake) REFERENCES jeux (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table remakes"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_remasters (
            jeu INTEGER,
            remaster INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (remaster) REFERENCES jeux (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table remasters"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_similaires (
            jeu INTEGER,
            jeu_similaire INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (jeu_similaire) REFERENCES jeux (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_similaires"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_genres (
            jeu INTEGER,
            genre INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (genre) REFERENCES genres (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_genres"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_themes (
            jeu INTEGER,
            theme INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (theme) REFERENCES themes (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_themes"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_mots_cles (
            jeu INTEGER,
            mot_cle INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (mot_cle) REFERENCES mots_cles (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_mots_cles"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_illustrations (
            jeu INTEGER,
            illustration INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (illustration) REFERENCES illustrations (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_illustrations"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_captures_ecran (
            jeu INTEGER,
            capture_ecran INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (capture_ecran) REFERENCES captures_ecran (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_captures_ecran"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_videos (
            jeu INTEGER,
            video INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (video) REFERENCES videos (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_videos"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS categories_plateforme (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(50) NOT NULL,
            name_traduit VARCHAR(50)
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table categories_plateforme"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS logos_plateforme (
            id INTEGER PRIMARY KEY NOT NULL,
            url TEXT,

            width INTEGER,
            height INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table logos_plateforme"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS plateformes (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            summary TEXT,
            summary_traduit TEXT,

            category INTEGER,

            platform_logo INTEGER,

            updated_at INTEGER
        );"/*
            FOREIGN KEY (category) REFERENCES categories_plateforme (id),
            FOREIGN KEY (platform_logo) REFERENCES logos_plateforme (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table plateformes"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_plateformes (
            jeu INTEGER,
            plateforme INTEGER
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (plateforme) REFERENCES plateformes (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_plateformes"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS logos_entreprise (
            id INTEGER PRIMARY KEY NOT NULL,
            url TEXT,

            width INTEGER,
            height INTEGER
        );"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table logos_entreprise"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS entreprises (
            id INTEGER PRIMARY KEY NOT NULL,
            name VARCHAR(250) NOT NULL,
            name_traduit VARCHAR(250),
            slug VARCHAR(250) NOT NULL UNIQUE,

            description TEXT,
            description_traduit TEXT,

            parent INTEGER,

            logo INTEGER,

            start_date INTEGER,

            updated_at INTEGER
        );"/*
            FOREIGN KEY (parent) REFERENCES entreprises (id),
            FOREIGN KEY (logo) REFERENCES logos_entreprise (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table entreprises"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS jeux_entreprises (
            jeu INTEGER,
            entreprise INTEGER,

            developed BOOLEAN,
            published BOOLEAN
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id),
            FOREIGN KEY (entreprise) REFERENCES entreprises (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table jeux_entreprises"}.as_err(),
    };

    let _ = match sqlx::query(
        "INSERT INTO \"categories_jeu\" (\"id\", \"name\", \"name_traduit\") VALUES
            (0, 'Main game', 'Jeu principal'),
            (1, 'DLC addon', 'DLC'),
            (2, 'Expansion', 'Extension'),
            (3, 'Bundle', 'Offre groupée'),
            (4, 'Standalone expansion', 'Extension indépendante'),
            (5, 'Mod', 'Mod'),
            (6, 'Episode', 'Épisode'),
            (7, 'Season', 'Saison'),
            (8, 'Remake', 'Remake'),
            (9, 'Remaster', 'Remaster'),
            (10, 'Expanded game', 'Jeu étendu'),
            (11, 'Port', 'Portage'),
            (12, 'Fork', 'Fourche'),
            (13, 'Pack', 'Paquet'),
            (14, 'Update', 'Mise à jour');"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "l'insertion de categories_jeu"}.as_err(),
    };

    let _ = match sqlx::query(
        "INSERT INTO \"categories_plateforme\" (\"id\", \"name\", \"name_traduit\") VALUES
            (1, 'Console', 'Console'),
            (2, 'Arcade', 'Arcade'),
            (3, 'Platform', 'Plateforme'),
            (4, 'Portable console', 'Console portable'),
            (5, 'Computer', 'Ordinateur');"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "l'insertion de categories_plateforme"}.as_err(),
    };

    let _ = match sqlx::query(
        "CREATE TABLE IF NOT EXISTS catalogue (
            jeu INTEGER,
            chemin TEXT,
            nom VARCHAR(100),
            langue VARCHAR(3)
        );"/*
            FOREIGN KEY (jeu) REFERENCES jeux (id)
        );"*/
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "la création de la table catalogue"}.as_err(),
    };

    let _ = match sqlx::query(
        "INSERT INTO \"jeux\" (\"id\", \"name\", \"slug\") VALUES
            (0, 'Unknown', 'unknown');"
    ).execute(&db).await {
        Ok(valeur) => valeur,
        Err(erreur) => return ErreurSQL { erreur, desc: "l'insertion de jeux"}.as_err(),
    };

    Ok(())
}
