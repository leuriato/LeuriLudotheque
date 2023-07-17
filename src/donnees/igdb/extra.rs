use crate::donnees::objet::Jeu;

use super::obtenir_db;

pub async fn obtenir_catalogue() -> Vec<Jeu> {
    let db = obtenir_db().await.unwrap();
    match sqlx::query_as::<_, Jeu>(
        "SELECT * FROM catalogue"
    ).fetch_all(&db).await {
        Ok(liste) => {
            db.close().await;
            liste
        },
        Err(erreur) => {
            db.close().await;
            println!("ATTENTION: {}", erreur);
            vec![]
        },
    }
}

pub async fn obtenir_jeux_async(filtre: &str) -> Vec<Jeu> {
    let db = obtenir_db().await.unwrap();
    match sqlx::query_as::<_, Jeu>(
        &format!(
            "SELECT * FROM catalogue LEFT JOIN jeux ON catalogue.jeu = jeux.id
            WHERE {};",
            filtre,
        ),
    ).fetch_all(&db).await {
        Ok(liste) => {
            db.close().await;
            liste
        },
        Err(erreur) => {
            db.close().await;
            println!("ATTENTION: {}", erreur);
            vec![]
        },
    }
}

pub fn obtenir_jeux_par(filtre: &str) -> Vec<Jeu> {
    async_std::task::block_on(async {
        obtenir_jeux_async(filtre).await
    })
}
