use granite::subclass::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use granite::traits::SettingsPageExt;

    use super::*;

    #[derive(Debug, Default)]
    pub struct SimpleSettingsPage;

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleSettingsPage {
        const NAME: &'static str = "SimpleSettingsPage";
        type Type = super::SimpleSettingsPage;
        type ParentType = granite::SimpleSettingsPage;
    }

    impl ObjectImpl for SimpleSettingsPage {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let icon_label = gtk::Label::builder()
                .label("Icon Name:")
                .xalign(1.0)
                .build();

            let icon_entry = gtk::Entry::builder()
                .hexpand(true)
                .placeholder_text("This page's icon name")
                .text(obj.icon_name().expect("Unable to get icon name"))
                .build();

            let title_label = gtk::Label::builder()
                .label("Title:")
                .xalign(1.0)
                .build();
        }
    }
    impl WidgetImpl for SimpleSettingsPage {}
    impl BoxImpl for SimpleSettingsPage {}
    impl SettingsPageImpl for SimpleSettingsPage {}
    impl SimpleSettingsPageImpl for SimpleSettingsPage {}
}

glib::wrapper! {
    pub struct SimpleSettingsPage(ObjectSubclass<imp::SimpleSettingsPage>)
        @extends granite::SimpleSettingsPage, granite::SettingsPage, gtk::Box, gtk::Widget, gtk::Grid;
}

impl SimpleSettingsPage {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("activatable", true)
            .property(
                "description",
                "This is a <b>demo</b> of Granite's <i>SimpleSettingsPage</i>",
            )
            .property("header", "Simple Pages")
            .property("icon-name", "preferences-system")
            .property("title", "First Test Page")
            .build()
    }
}
