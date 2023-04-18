use granite::subclass::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SettingsPage;

    #[glib::object_subclass]
    impl ObjectSubclass for SettingsPage {
        const NAME: &'static str = "SettingsPage";
        type Type = super::SettingsPage;
        type ParentType = granite::SettingsPage;
    }

    impl ObjectImpl for SettingsPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
        }
    }
    impl WidgetImpl for SettingsPage {}
    impl BoxImpl for SettingsPage {}
    impl SettingsPageImpl for SettingsPage {}
}

glib::wrapper! {
    pub struct SettingsPage(ObjectSubclass<imp::SettingsPage>)
        @extends granite::SettingsPage, gtk::Box, gtk::Widget, gtk::Grid;
}

impl SettingsPage {
    pub fn new() -> Self {
        let display_widget = gtk::Spinner::builder().height_request(32).build();

        glib::Object::builder()
            .property("display-widget", display_widget)
            .property("status", "Spinning")
            .property("header", "Manual Pages")
            .property("title", "Custom Display Widget Page")
            .build()
    }
}
