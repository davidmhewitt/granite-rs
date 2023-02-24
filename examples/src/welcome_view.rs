use granite::traits::PlaceholderExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct WelcomeView;

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomeView {
        const NAME: &'static str = "WelcomeView";
        type Type = super::WelcomeView;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for WelcomeView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let welcome = granite::Placeholder::builder()
                .title("Granite Demo")
                .description("This is a demo of the Granite library")
                .build();

            let vala_button = welcome
                .append_button(
                    &gio::ThemedIcon::new("text-x-vala"),
                    "Visit Valadoc",
                    "The canonical source for Vala API references",
                )
                .unwrap();

            let source_button = welcome
                .append_button(
                    &gio::ThemedIcon::new("text-x-source"),
                    "Get Granite Source",
                    "Granite's source code is hosted on GitHub",
                )
                .unwrap();

            let alert = granite::Placeholder::builder()
                .title("Panic! At the button")
                .description("Maybe you can <b>do something</b> to hide it but <i>otherwise</i> it will stay here")
                .icon(&gio::ThemedIcon::new("dialog-warning"))
                .build();
            alert.add_css_class(granite::STYLE_CLASS_WARNING);

            let alert_action = alert
                .append_button(
                    &gio::ThemedIcon::new("edit-delete"),
                    "Hide This Button",
                    "Click here to hide this",
                )
                .unwrap();

            let search_placeholder = granite::Placeholder::builder()
                .title("No Apps Found")
                .description("Try changing search terms. You can also sideload Flatpak apps e.g. from <a href='https://flathub.org'>Flathub</a>")
                .icon(&gio::ThemedIcon::new("edit-find-symbolic"))
                .build();

            let stack = gtk::Stack::builder().vexpand(true).build();

            stack.add_titled(&welcome, Some("Welcome"), "Welcome");
            stack.add_titled(&alert, Some("Alert"), "Alert");
            stack.add_titled(&search_placeholder, Some("Search"), "Search");

            let stack_switcher = gtk::StackSwitcher::builder()
                .margin_top(24)
                .margin_end(24)
                .margin_start(24)
                .stack(&stack)
                .build();

            vala_button.connect_clicked(|_| {
                gtk::show_uri(
                    gtk::Window::NONE,
                    "https://valadoc.org/granite/Granite.html",
                    gtk::gdk::CURRENT_TIME,
                );
            });

            source_button.connect_clicked(|_| {
                gtk::show_uri(
                    gtk::Window::NONE,
                    "https://github.com/davidmhewitt/granite-rs",
                    gtk::gdk::CURRENT_TIME,
                );
            });

            alert_action.connect_clicked(|alert_action| {
                alert_action.hide();
            });

            obj.append(&stack_switcher);
            obj.append(&stack);
        }
    }
    impl WidgetImpl for WelcomeView {}
    impl BoxImpl for WelcomeView {}
}

glib::wrapper! {
    pub struct WelcomeView(ObjectSubclass<imp::WelcomeView>)
        @extends gtk::Widget, gtk::Box;
}

impl WelcomeView {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("orientation", gtk::Orientation::Vertical)
            .property("spacing", 0)
            .build()
    }
}
