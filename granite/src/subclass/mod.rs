pub mod dialog;

pub mod prelude {
    pub use super::dialog::GraniteDialogImpl;

    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
    pub use gtk::subclass::prelude::*;
}