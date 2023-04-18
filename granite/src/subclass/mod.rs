pub mod dialog;
pub mod settings_page;
pub mod simple_settings_page;

pub mod prelude {
    pub use super::dialog::GraniteDialogImpl;
    pub use super::settings_page::SettingsPageImpl;
    pub use super::simple_settings_page::SimpleSettingsPageImpl;

    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
    pub use gtk::subclass::prelude::*;
}