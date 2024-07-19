use granite::prelude::*;
use gtk::prelude::{SettingsExt, *};
use gtk::subclass::prelude::*;

mod imp {
    use glib::WeakRef;

    use super::*;

    #[derive(Debug, Default)]
    pub struct DateTimePickerView {
        pub date_picker: WeakRef<granite::DatePicker>,
        pub time_picker: WeakRef<granite::TimePicker>,
        pub relative_datetime: WeakRef<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DateTimePickerView {
        const NAME: &'static str = "DateTimePickerView";
        type Type = super::DateTimePickerView;
        type ParentType = gtk::Grid;
    }
    impl ObjectImpl for DateTimePickerView {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let pickers_label = gtk::Label::builder()
                .label("Picker Widgets")
                .xalign(0.)
                .build();

            pickers_label
                .style_context()
                .add_class(granite::STYLE_CLASS_H4_LABEL);

            let date_label = gtk::Label::builder()
                .label("DatePicker:")
                .halign(gtk::Align::End)
                .build();

            let date_picker = granite::DatePicker::new();
            self.date_picker.set(Some(&date_picker));

            let time_label = gtk::Label::builder()
                .label("TimePicker:")
                .halign(gtk::Align::End)
                .build();

            let time_picker = granite::TimePicker::new();
            self.time_picker.set(Some(&time_picker));

            let formatting_label = gtk::Label::builder()
                .label("String Formatting")
                .margin_top(6)
                .xalign(0.)
                .build();

            formatting_label
                .style_context()
                .add_class(granite::STYLE_CLASS_H4_LABEL);

            let current_time_label = gtk::Label::builder()
                .label("Localized time:")
                .halign(gtk::Align::End)
                .build();

            let now = glib::DateTime::now_local().expect("Unable to get current time");
            let settings = gio::Settings::new("org.gnome.desktop.interface");
            let time_format = granite::date_time_get_default_time_format(
                settings.enum_("clock-format") == 1,
                false,
            )
            .expect("Unable to get default time format");

            let current_time = gtk::Label::builder()
                .label(
                    now.format(time_format.as_str())
                        .expect("Unable to format time"),
                )
                .tooltip_text(time_format)
                .xalign(0.)
                .build();

            let current_date_label = gtk::Label::builder()
                .label("Localized date:")
                .halign(gtk::Align::End)
                .build();

            let date_format = granite::date_time_get_default_date_format(true, true, true)
                .expect("Unable to get default date format");

            let current_date = gtk::Label::builder()
                .label(
                    now.format(date_format.as_str())
                        .expect("Unable to format date"),
                )
                .tooltip_text(date_format)
                .xalign(0.)
                .build();

            let relative_datetime_label = gtk::Label::builder()
                .label("Relative datetime:")
                .halign(gtk::Align::End)
                .build();

            let relative_datetime = gtk::Label::builder().label("").xalign(0.).build();

            self.relative_datetime.set(Some(&relative_datetime));

            obj.set_selected_datetime();
            date_picker.connect_changed(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.set_selected_datetime();
                }
            ));
            time_picker.connect_changed(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.set_selected_datetime();
                }
            ));

            obj.attach(&pickers_label, 0, 0, 1, 1);
            obj.attach(&date_label, 0, 1, 1, 1);
            obj.attach(&date_picker, 1, 1, 1, 1);
            obj.attach(&time_label, 0, 2, 1, 1);
            obj.attach(&time_picker, 1, 2, 1, 1);
            obj.attach(&formatting_label, 0, 3, 1, 1);
            obj.attach(&current_time_label, 0, 4, 1, 1);
            obj.attach(&current_time, 1, 4, 1, 1);
            obj.attach(&current_date_label, 0, 5, 1, 1);
            obj.attach(&current_date, 1, 5, 1, 1);
            obj.attach(&relative_datetime_label, 0, 6, 1, 1);
            obj.attach(&relative_datetime, 1, 6, 1, 1);
        }
    }
    impl WidgetImpl for DateTimePickerView {}
    impl GridImpl for DateTimePickerView {}
}

glib::wrapper! {
    pub struct DateTimePickerView(ObjectSubclass<imp::DateTimePickerView>)
        @extends gtk::Widget, gtk::Grid;
}

impl DateTimePickerView {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("column-spacing", 12)
            .property("row-spacing", 6)
            .property("halign", gtk::Align::Center)
            .property("valign", gtk::Align::Center)
            .build()
    }

    fn set_selected_datetime(&self) {
        let mut selected_datetime = self
            .imp()
            .date_picker
            .upgrade()
            .expect("Unable to get date picker")
            .date()
            .expect("Unable to get date from date picker");

        selected_datetime = selected_datetime
            .add_hours(
                self.imp()
                    .time_picker
                    .upgrade()
                    .expect("Unable to get time picker")
                    .time()
                    .expect("Unable to get time from time picker")
                    .hour(),
            )
            .expect("Unable to add hours to time");

        selected_datetime = selected_datetime
            .add_minutes(
                self.imp()
                    .time_picker
                    .upgrade()
                    .expect("Unable to get time picker")
                    .time()
                    .expect("Unable to get time from time picker")
                    .minute(),
            )
            .expect("Unable to add minutes to time");

        self.imp()
            .relative_datetime
            .upgrade()
            .expect("Unable to get relative datetime label")
            .set_label(
                granite::date_time_get_relative_datetime(&selected_datetime)
                    .expect("Unable to format relative datetime")
                    .as_str(),
            );
    }
}

impl Default for DateTimePickerView {
    fn default() -> Self {
        Self::new()
    }
}
