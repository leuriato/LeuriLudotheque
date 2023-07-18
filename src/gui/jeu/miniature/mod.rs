mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;

use crate::chemin::chemins;
use crate::outils::lanceur::lancer_jeu;
use crate::donnees::objet::Jeu;

glib::wrapper! {
    pub struct Miniature(ObjectSubclass<imp::Miniature>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Miniature {
    pub fn modele() -> Self {
        Miniature::for_jeu(
            Jeu {
                jeu: None,
                chemin: String::new(),
                nom: String::from("Inconnu"),
                langue: String::new(),
            }
        )
    }

    pub fn new(nom: String, image: String, commande: String) -> Self {
        let miniature: Miniature = Object::builder().build();

        let boite = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .width_request(200)
            .height_request(300)
            .spacing(5)
            .margin_top(0)
            .margin_bottom(10)
            .margin_start(0)
            .margin_end(0)
            .halign(gtk::Align::Start)
            .hexpand(false)
            .valign(gtk::Align::Start)
            .vexpand(false)
            .build();

        let image = gtk::Picture::for_filename(image);
        image.set_can_shrink(true);
        image.set_content_fit(gtk::ContentFit::Cover);
        image.set_halign(gtk::Align::Start);
        image.set_hexpand(false);
        image.set_valign(gtk::Align::Start);
        image.set_vexpand(false);

        let titre = gtk::Label::builder()
            .label(nom)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::Word)
            .halign(gtk::Align::Center)
            .hexpand(false)
            .valign(gtk::Align::Start)
            .vexpand(false)
            .lines(2)
            .justify(gtk::Justification::Center)
            .build();

        boite.append(&image);
        boite.append(&titre);

        let controlleur = gtk::GestureClick::new();
        controlleur.connect_released(glib::clone!(@strong commande => move |_, _, _, _| {
            lancer_jeu(commande.clone());
        }));

        miniature.add_controller(controlleur);
        miniature.append(&boite);

        miniature
    }

    pub fn for_jeu(jeu: Jeu) -> Self {
        Miniature::new(
            format!("{} {}", jeu.nom, jeu.langue).trim().to_string(),
            chemins::trouver_chemin(
                format!("{}.jpg", jeu.jeu.unwrap_or(0)),
                chemins::XDG::CACHE,
            ).unwrap().to_str().unwrap().to_string(),
            jeu.chemin,
        )
    }
}
