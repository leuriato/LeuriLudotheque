use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct Miniature;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Miniature {
    const NAME: &'static str = "MiniatureLeuriLudotheque";
    type Type = super::Miniature;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for Miniature {}

// Trait shared by all widgets
impl WidgetImpl for Miniature {}

// Trait shared by all boxes
impl BoxImpl for Miniature {}
