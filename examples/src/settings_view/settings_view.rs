use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use crate::settings_view;

    use super::*;

    #[derive(Debug, Default)]
    pub struct SettingsView;

    #[glib::object_subclass]
    impl ObjectSubclass for SettingsView {
        const NAME: &'static str = "SettingsView";
        type Type = super::SettingsView;
        type ParentType = gtk::Box;
    }
    impl ObjectImpl for SettingsView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let settings_page = settings_view::SimpleSettingsPage::new();
        }
    }
    impl WidgetImpl for SettingsView {}
    impl BoxImpl for SettingsView {}
}

glib::wrapper! {
    pub struct SettingsView(ObjectSubclass<imp::SettingsView>)
        @extends gtk::Widget, gtk::Grid;
}

impl SettingsView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for SettingsView {
    fn default() -> Self {
        Self::new()
    }
}
