use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct AccelLabelView;

    #[glib::object_subclass]
    impl ObjectSubclass for AccelLabelView {
        const NAME: &'static str = "AccelLabelView";
        type Type = super::AccelLabelView;
        type ParentType = gtk::Grid;
    }
    impl ObjectImpl for AccelLabelView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let accellabel_label = gtk::Label::builder()
                .label("AccelLabel:")
                .halign(gtk::Align::End)
                .build();

            let copy_label = granite::AccelLabel::new("Copy", Some("<Ctrl>C"));

            let popover_label = gtk::Label::builder()
                .label("In a Popover:")
                .halign(gtk::Align::End)
                .build();

            let lock_button = gtk::Button::new();
            lock_button.set_child(Some(&granite::AccelLabel::new("Lock", Some("<Super>L"))));
            lock_button.add_css_class(granite::STYLE_CLASS_MENUITEM);

            let logout_button = gtk::Button::new();
            logout_button.set_child(Some(&granite::AccelLabel::new(
                "Log Out…",
                Some("<Ctrl><Alt>Delete"),
            )));
            logout_button.add_css_class(granite::STYLE_CLASS_MENUITEM);

            let popover_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(0)
                .margin_top(3)
                .margin_bottom(3)
                .build();
            popover_box.append(&lock_button);
            popover_box.append(&logout_button);

            let popover = gtk::Popover::builder()
                .child(&popover_box)
                .build();

            let popover_button = gtk::MenuButton::builder()
                .popover(&popover)
                .build();

            obj.attach(&accellabel_label, 0, 0, 1, 1);
            obj.attach(&copy_label, 1, 0, 1, 1);
            obj.attach(&popover_label, 0, 1, 1, 1);
            obj.attach(&popover_button, 1, 1, 1, 1);
        }
    }
    impl WidgetImpl for AccelLabelView {}
    impl GridImpl for AccelLabelView {}
}

glib::wrapper! {
    pub struct AccelLabelView(ObjectSubclass<imp::AccelLabelView>)
        @extends gtk::Widget, gtk::Grid;
}

impl AccelLabelView {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("column-spacing", 12)
            .property("row-spacing", 12)
            .property("halign", gtk::Align::Center)
            .property("valign", gtk::Align::Center)
            .build()
    }
}

impl Default for AccelLabelView {
    fn default() -> Self {
        Self::new()
    }
}
