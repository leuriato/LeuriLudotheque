use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, ScrolledWindow, Box};

pub mod jeu;

use crate::donnees::igdb::creer_db;
use crate::outils::{dl::telecharger_couvertures, scan::scanner};
use crate::donnees::igdb::extra::obtenir_jeux_async;
use crate::gui::jeu::construire_categorie;
use crate::interne::erreurs::TraitErreur;

const APP_ID: &str = "org.leuriato.ludotheque";

pub fn lancer_application() -> glib::ExitCode {
    let application = Application::builder()
        .application_id(APP_ID)
        .build();

    application.connect_activate(construire_ui);

    application.run()
}

pub fn construire_ui(application: &Application) {
    let (sender, receiver) = glib::MainContext::channel(glib::Priority::default());

    let deroulante = ScrolledWindow::builder()
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .build();

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

    let surboite = std::sync::Arc::new(boite);

    receiver.attach(
        None,
        glib::clone!(@weak surboite => @default-return Continue(false),
            move |widget| {
                println!("Reception");
                surboite.append(&widget);
                surboite.queue_draw();
                Continue(true)
            }
        )
    );

    fenetre.connect_show( move |_| {
        let contexte_principal = glib::MainContext::default();

        contexte_principal.spawn_local(glib::clone!(@strong sender => async move {
            charger_ui(sender).await;
        }));
    });

    fenetre.present();
}

async fn charger_ui(sender: glib::Sender<Box>) {
    //match creer_db().await {
    //    Ok(_) => (),
    //    Err(erreur) => return erreur.afficher_erreur(),
    //}

    //scanner().await;

    //telecharger_couvertures().await;

    println!("Création des catégories");
    let categorie1 = construire_categorie(
        "Pokémon",
        obtenir_jeux_async("name LIKE '%Pok_mon%' ORDER BY rating DESC").await,
        &sender,
    );
    let categorie2 = construire_categorie(
        "The Legend of Zelda",
        obtenir_jeux_async("collection = 106 ORDER BY rating DESC").await,
        &sender,
    );
    let categorie3 = construire_categorie(
        "Professeur Layton",
        obtenir_jeux_async("collection = 297 ORDER BY rating DESC").await,
        &sender,
    );
    println!("Début");
    categorie1.await;
    categorie2.await;
    categorie3.await;
    println!("Fin");
}
