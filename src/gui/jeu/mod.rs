use std::process::Command;

use gtk::prelude::*;
use gtk::{Box, Image, Label, ScrolledWindow};
use gtk::Adjustment;

use crate::chemin::chemins;
use crate::donnees::config::obtenir_config;
use crate::donnees::objet::Jeu;

fn obtenir_commande(chemin: String) -> String {
    let config = obtenir_config();

    for emulateur in &config.emulateurs {
        for extension in &emulateur.extensions {
            if chemin.ends_with(format!(".{}", extension).as_str()) {
                return emulateur.commande.clone();
            }
        }
    }

    String::from("echo Aucun émulateur trouvé.")
}

fn creer_commande(chemin: String) -> String {
    let mut commande = obtenir_commande(chemin.clone());
    let chemin_str = chemin.replace("'", "\\'");

    commande = commande.replace("{{chemin}}", "{{/!\\-*-tmp-*-/!\\}}");
    commande = commande.replace("{chemin}", format!("'{chemin_str}'").as_str());
    commande = commande.replace("{{/!\\-*-tmp-*-/!\\}}", "{{chemin}}");

    commande
}

fn construire_miniature(jeu: Jeu) -> gtk::Widget {
    let miniature = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .width_request(264)
        .height_request(300)
        .margin_top(0)
        .margin_bottom(0)
        .margin_start(0)
        .margin_end(0)
        .build();

    let image = Image::builder()
        //.width_request(264)
        //.height_request(352)
        .vexpand(true)
        .hexpand(true)
        .file(
            chemins::trouver_chemin(
                format!("{}.jpg", jeu.jeu.unwrap_or(0)),
                chemins::XDG::CACHE,
            ).unwrap().as_path().as_os_str().to_str().unwrap_or("")
        )
        .build();

    let titre = Label::builder()
        .label(
            format!(
                "{} {}",
                jeu.nom,
                jeu.langue
            ).trim()
        )
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::Word)
        .hexpand(true)
        .lines(2)
        .justify(gtk::Justification::Center)
        .build();

    let chemin = std::sync::Arc::new(jeu.chemin.clone());

    let evenements = gtk::GestureClick::new();
    evenements.connect_released(move |_, _, _, _| {
        Command::new("sh")
            .arg("-c")
            .arg(creer_commande(chemin.to_string()))
            .spawn()
            .expect("Le lancement du jeu a échoué");
    });

    miniature.add_controller(evenements);

    miniature.prepend(&titre);
    miniature.prepend(&image);

    miniature.upcast()
}

pub fn construire_categorie(nom: &str, jeux: Vec<Jeu>, parent: &ScrolledWindow) -> gtk::Widget {
    let categorie = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(0)
        .margin_bottom(0)
        .margin_start(0)
        .margin_end(0)
        .build();

    let titre = Label::builder()
        .label(format!("<span font=\"24\">{nom}</span>"))
        .hexpand(true)
        .justify(gtk::Justification::Left)
        .use_markup(true)
        .build();

    let fenetre = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .vadjustment(&parent.vadjustment())
        .build();

    let boite = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    for i in 1..=jeux.len() {
        let miniature = construire_miniature(jeux[jeux.len()-i].clone());
        boite.prepend(&miniature);
    }
    fenetre.set_child(Some(&boite));

    categorie.prepend(&fenetre);
    categorie.prepend(&titre);

    categorie.upcast()
}

