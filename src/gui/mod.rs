use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, ScrolledWindow, Box};

pub mod jeu;

use crate::donnees::igdb::extra::obtenir_jeux_par;
use crate::gui::jeu::construire_categorie;

const APP_ID: &str = "org.leuriato.ludotheque";

pub fn lancer_application() -> glib::ExitCode {
    let application = Application::builder()
        .application_id(APP_ID)
        .build();

    application.connect_activate(construire_ui);

    application.run()
}

pub fn construire_ui(application: &Application) {
    let deroulante = ScrolledWindow::builder()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

    deroulante.connect_scroll_child( |c, h, w| {
        println!("{}\t{}", h, w);
        true
    });

    let fenetre = ApplicationWindow::builder()
        .application(application)
        .title("LeuriLudothèque")
        .width_request(1280)
        .height_request(720)
        .child(&deroulante)
        .build();

    let boite = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    deroulante.set_child(Some(&boite));

    let categorie1 = construire_categorie(
        "Pokémon",
        obtenir_jeux_par("name LIKE '%Pok_mon%' ORDER BY rating DESC"),
        &deroulante,
    );
    let categorie2 = construire_categorie(
        "The Legend of Zelda",
        obtenir_jeux_par("collection = 106 ORDER BY rating DESC"),
        &deroulante,
    );
    let categorie3 = construire_categorie(
        "Professeur Layton",
        obtenir_jeux_par("collection = 297 ORDER BY rating DESC"),
        &deroulante,
    );

    boite.prepend(&categorie3);
    boite.prepend(&categorie2);
    boite.prepend(&categorie1);

    deroulante.set_vscrollbar_policy(gtk::PolicyType::Automatic);

    fenetre.present();
}

