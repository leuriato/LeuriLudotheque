use std::process::Command;

use crate::{donnees::{igdb::{extra::obtenir_catalogue, interface::CompatibleSQL}, objet::*}, chemin::chemins};

pub async fn telecharger_couvertures() {
    //println!("Obtention du catalogue");
    let jeux = obtenir_catalogue().await;

    for jeu in jeux {
        //println!("Jeu {}", jeu.chemin);
        if let Some(id) = jeu.jeu {
            //println!("Id: {}", id);
            let jeu_igdb = match JeuIGDB::charger(id).await {
                Ok(Some(valeur)) => valeur,
                _ => continue,
            };
            //println!("JeuIGDB trouvé:\n{:?}", &jeu_igdb.rating);

            let couverture = match jeu_igdb.cover {
                Some(valeur) => valeur,
                None => continue,
            };
            //println!("CouvertureIGDB trouvée");

            let chemin = chemins::determiner_chemin(
                format!("{}.jpg", id),
                chemins::XDG::CACHE
            ).unwrap();

            if chemin.exists() {
                continue;
            }

            let lien = couverture.url.replace("t_thumb", "t_cover_big");
            let commande = format!("curl -o {:?} https:{lien}", chemin);

            println!("Commande: {}", &commande);

            Command::new("sh")
                .arg("-c")
                .arg(commande)
                .spawn()
                .expect("Impossible de télécharger la couverture.");

            std::thread::sleep(std::time::Duration::from_millis(2500));
        }
    }

    let chemin = chemins::determiner_chemin(String::from("0.jpg"), chemins::XDG::CACHE).unwrap();
    if !chemin.exists() {
        Command::new("sh")
            .arg("-c")
            .arg(format!("curl -o {:?} https://images.igdb.com/igdb/image/upload/t_cover_big/nocover.jpg", chemin))
            .spawn()
            .expect("Impossible de télécharger la couverture.");
    }
}
