use crate::SimpleSettingsPage;
use glib::subclass::prelude::*;

use super::prelude::SettingsPageImpl;

pub trait SimpleSettingsPageImpl: SettingsPageImpl {}
unsafe impl<T: SettingsPageImpl> IsSubclassable<T> for SimpleSettingsPage {}
