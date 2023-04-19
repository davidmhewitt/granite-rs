use glib::clone;
use granite::subclass::prelude::*;
use granite::traits::SettingsPageExt;
use gtk::traits::{BoxExt, EditableExt};

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

            let title_label = gtk::Label::builder().xalign(1.0).label("Title:").build();

            let title_entry = gtk::Entry::builder()
                .hexpand(true)
                .placeholder_text("This page's title")
                .build();

            let content_area = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .margin_bottom(12)
                .margin_end(12)
                .margin_start(12)
                .margin_top(12)
                .valign(gtk::Align::Start)
                .build();

            content_area.append(&title_label);
            content_area.append(&title_entry);

            obj.set_child(&content_area);

            title_entry.connect_changed(clone!(@weak obj as settings_page => move |title_entry| {
                settings_page.set_title(&title_entry.text());
            }));
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
        display_widget.start();

        glib::Object::builder()
            .property("display-widget", display_widget)
            .property("status", "Spinning")
            .property("header", "Manual Pages")
            .property("title", "Custom Display Widget Page")
            .build()
    }
}
