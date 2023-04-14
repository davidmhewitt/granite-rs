use crate::Dialog;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::*;

pub trait GraniteDialogImpl: DialogImpl {}

unsafe impl<T: GraniteDialogImpl> IsSubclassable<T> for Dialog {}