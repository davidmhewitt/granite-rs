use granite::{prelude::*, subclass::prelude::*};
use gtk::prelude::*;

mod imp {
    use glib::clone;

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

            let title_label = gtk::Label::builder().label("Title:").xalign(1.0).build();

            let title_entry = gtk::Entry::builder()
                .hexpand(true)
                .placeholder_text("This page's title")
                .build();

            let description_label = gtk::Label::builder()
                .xalign(1.0)
                .label("Description:")
                .build();

            let description_entry = gtk::Entry::builder()
                .hexpand(true)
                .placeholder_text("This page's description")
                .build();

            let content_area = obj.content_area().expect("Couldn't get content area");
            content_area.attach(&icon_label, 0, 0, 1, 1);
            content_area.attach(&icon_entry, 1, 0, 1, 1);
            content_area.attach(&title_label, 0, 1, 1, 1);
            content_area.attach(&title_entry, 1, 1, 1, 1);
            content_area.attach(&description_label, 0, 2, 1, 1);
            content_area.attach(&description_entry, 1, 2, 1, 1);

            let button = gtk::Button::with_label("Test Button");
            let action_area = obj.action_area().expect("Couldn't get action area");
            action_area.append(&button);

            obj.update_status();

            description_entry.connect_changed(
                clone!(@weak obj as settings_page => move |desc_entry| {
                    settings_page.set_description(&desc_entry.text());
                }),
            );

            icon_entry.connect_changed(clone!(@weak obj as settings_page => move |icon_entry| {
                settings_page.set_icon_name(Some(&icon_entry.text()));
            }));

            title_entry.connect_changed(clone!(@weak obj as settings_page => move |title_entry| {
                settings_page.set_title(&title_entry.text());
            }));

            obj.status_switch()
                .expect("Couldn't get status switch")
                .connect_notify_local(
                    Some("active"),
                    clone!(@weak obj as settings_page => move |_, _| {
                        settings_page.update_status();
                    }),
                );
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

    pub fn update_status(&self) {
        let status_switch = self.status_switch().expect("Couldn't get status switch");
        if status_switch.is_active() {
            self.set_status_type(granite::SettingsPageStatusType::Success);
            self.set_status("Enabled");
        } else {
            self.set_status_type(granite::SettingsPageStatusType::Offline);
            self.set_status("Disabled");
        }
    }
}
