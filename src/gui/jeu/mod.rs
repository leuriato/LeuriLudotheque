use std::process::Command;

use gtk::glib;
use gtk::prelude::*;
use gtk::{Box, Label, ScrolledWindow};

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

async fn construire_miniature(jeu: Jeu) -> gtk::Widget {
    let miniature = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        //.width_request(200)
        //.height_request(400)
        .valign(gtk::Align::Start)
        .halign(gtk::Align::Fill)
        .spacing(5)
        .margin_bottom(30)
        .build();

    let image = gtk::Picture::for_filename(
        chemins::determiner_chemin(
            format!("{}.png", jeu.jeu.unwrap_or(0)),
            chemins::XDG::CACHE,
        ).unwrap().as_path().as_os_str().to_str().unwrap_or("")
    );
    image.set_can_shrink(true);
    image.set_content_fit(gtk::ContentFit::Cover);
    image.set_size_request(180, 240);

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
        //.height_request(30)
        .valign(gtk::Align::Start)
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

pub async fn construire_categorie(nom: &str, jeux: Vec<Jeu>, sender: &glib::Sender<Box>) {
    let categorie = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(20)
        .margin_end(20)
        .homogeneous(false)
        .halign(gtk::Align::Fill)
        .hexpand(true)
        .build();

    let titre = Label::builder()
        .label(format!("<span font=\"24\">{nom}</span>"))
        .hexpand(true)
        .justify(gtk::Justification::Left)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .use_markup(true)
        .build();
    categorie.append(&titre);

    let fenetre = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Never)
        .hexpand(true)
        .build();
    categorie.append(&fenetre);

    println!("Envoi");
    sender.send(categorie).unwrap();

    let boite = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(false)
        .hexpand(false)
        .halign(gtk::Align::Start)
        .spacing(20)
        .build();

    for jeu in jeux {
        let miniature = construire_miniature(jeu).await;
        boite.append(&miniature);
        boite.queue_draw();
        std::thread::sleep(std::time::Duration::from_millis(250))
    }

    fenetre.set_child(Some(&boite));
}

