use glib::clone;
use granite::prelude::*;
use gtk::prelude::DialogExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::sample_dialog::SampleDialog;

mod imp {
    use std::cell::RefCell;

    use glib::clone;
    use glib_macros::Properties;
    use once_cell::sync::OnceCell;

    use super::*;

    #[derive(Properties, Debug, Default)]
    #[properties(wrapper_type = super::DialogsView)]
    pub struct DialogsView {
        #[property(get, set)]
        window: RefCell<gtk::Window>,
        pub toast: OnceCell<granite::Toast>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DialogsView {
        const NAME: &'static str = "DialogsView";
        type Type = super::DialogsView;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for DialogsView {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let overlay = gtk::Overlay::new();
            obj.append(&overlay);

            let dialog_button = gtk::Button::with_label("Show Dialog");
            let toast = granite::Toast::new("Did something");

            let grid = gtk::Grid::builder()
                .hexpand(true)
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .row_spacing(12)
                .build();

            grid.attach(&dialog_button, 0, 1, 1, 1);

            overlay.set_child(Some(&grid));
            overlay.add_overlay(&toast);

            obj.imp().toast.set(toast).expect("Unable to set toast");

            dialog_button.connect_clicked(clone!(
                #[weak]
                obj,
                move |_| obj.show_dialog()
            ));
        }
    }
    impl WidgetImpl for DialogsView {}
    impl BoxImpl for DialogsView {}
}

glib::wrapper! {
    pub struct DialogsView(ObjectSubclass<imp::DialogsView>)
        @extends gtk::Widget, gtk::Box;
}

impl DialogsView {
    pub fn new(window: gtk::Window) -> Self {
        glib::Object::builder().property("window", window).build()
    }

    fn toast(&self) -> &granite::Toast {
        self.imp().toast.get().expect("Unable to get toast")
    }

    fn show_dialog(&self) {
        let dialog = SampleDialog::new();
        dialog.set_transient_for(Some(&self.window()));
        dialog.connect_response(clone!(
            #[strong]
            dialog,
            #[weak(rename_to = view)]
            self,
            move |_, resp| {
                if resp == gtk::ResponseType::Accept {
                    view.toast().send_notification();
                }

                dialog.close();
            }
        ));

        dialog.show();
    }
}
