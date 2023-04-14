use gtk::prelude::*;
use gtk::subclass::prelude::*;
use granite::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SampleDialog {}

    #[glib::object_subclass]
    impl ObjectSubclass for SampleDialog {
        const NAME: &'static str = "SampleDialog";
        type Type = super::SampleDialog;
        type ParentType = granite::Dialog;
    }

    impl ObjectImpl for SampleDialog {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let header = granite::HeaderLabel::new("Header");

            let entry = gtk::Entry::new();
            let gtk_switch = gtk::Switch::builder()
                .halign(gtk::Align::Start)
                .build();

            let layout = gtk::Grid::builder()
                .row_spacing(12)
                .vexpand(true)
                .build();

            layout.attach(&header, 0, 1, 1, 1);
            layout.attach(&entry, 0, 2, 1, 1);
            layout.attach(&gtk_switch, 0, 3, 1, 1);

            obj.content_area().append(&layout);
            obj.add_button("Cancel", gtk::ResponseType::Cancel);

            let suggested_action_button = obj.add_button("Suggested Action", gtk::ResponseType::Accept);
            suggested_action_button.style_context().add_class(granite::STYLE_CLASS_SUGGESTED_ACTION);
        }
    }
    impl WidgetImpl for SampleDialog {}
    impl WindowImpl for SampleDialog {}
    impl DialogImpl for SampleDialog {}
    impl GraniteDialogImpl for SampleDialog {}
}

glib::wrapper! {
    pub struct SampleDialog(ObjectSubclass<imp::SampleDialog>)
        @extends gtk::Widget, gtk::Window, gtk::Dialog, granite::Dialog;
}

impl SampleDialog {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}