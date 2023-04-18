use crate::SettingsPage;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::*;

pub trait SettingsPageImpl: BoxImpl {}
unsafe impl<T: SettingsPageImpl> IsSubclassable<T> for SettingsPage {}
