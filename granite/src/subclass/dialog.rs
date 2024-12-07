use crate::Dialog;
use glib::subclass::prelude::*;
use gtk::subclass::prelude::*;

pub trait GraniteDialogImpl: DialogImpl {}

unsafe impl<T: GraniteDialogImpl> IsSubclassable<T> for Dialog {}

#[cfg(feature = "relm4")]
mod relm4 {
    use crate::Dialog;
    use gtk::prelude::{BoxExt, DialogExt};
    use relm4::{ContainerChild, RelmContainerExt};

    impl ContainerChild for Dialog {
        type Child = gtk::Widget;
    }

    impl RelmContainerExt for Dialog {
        fn container_add(&self, widget: &impl AsRef<Self::Child>) {
            self.content_area().append(widget.as_ref());
        }
    }
}
