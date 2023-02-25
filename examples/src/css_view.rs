use glib_macros::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Properties, Debug, Default)]
    #[properties(wrapper_type = super::CssView)]
    pub struct CssView {
        #[property(get, set)]
        window: RefCell<gtk::Window>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CssView {
        const NAME: &'static str = "CssView";
        type Type = super::CssView;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for CssView {
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

            let header1 = gtk::Label::builder()
                .label("\"h1\" Style Class")
                .margin_end(24)
                .margin_start(24)
                .margin_top(12)
                .build();

            header1
                .style_context()
                .add_class(granite::STYLE_CLASS_H1_LABEL);

            let header2 = gtk::Label::new(Some("\"h2\" Style Class"));
            header2
                .style_context()
                .add_class(granite::STYLE_CLASS_H2_LABEL);

            let header3 = gtk::Label::new(Some("\"h3\" Style Class"));
            header3
                .style_context()
                .add_class(granite::STYLE_CLASS_H3_LABEL);

            let header4 = gtk::Label::builder()
                .label("\"h4\" Style Class")
                .margin_bottom(12)
                .build();

            header4
                .style_context()
                .add_class(granite::STYLE_CLASS_H4_LABEL);

            let card_header = granite::HeaderLabel::builder()
                .label("Cards and Headers")
                .secondary_text("\"card\" with \"rounded\" and \"checkerboard\" style classes")
                .build();

            let card = gtk::Box::new(gtk::Orientation::Vertical, 0);
            card.add_css_class(granite::STYLE_CLASS_CARD);
            card.add_css_class(granite::STYLE_CLASS_CHECKERBOARD);
            card.add_css_class(granite::STYLE_CLASS_ROUNDED);

            card.append(&header1);
            card.append(&header2);
            card.append(&header3);
            card.append(&header4);

            let richlist_label = granite::HeaderLabel::builder()
                .label("Lists")
                .secondary_text("\"rich-list\" and \"frame\" style classes")
                .build();

            let rich_listbox = gtk::ListBox::builder().show_separators(true).build();

            rich_listbox.add_css_class(granite::STYLE_CLASS_RICH_LIST);
            rich_listbox.add_css_class(granite::STYLE_CLASS_FRAME);
            rich_listbox.append(&gtk::Label::new(Some("Row 1")));
            rich_listbox.append(&gtk::Label::new(Some("Row 2")));
            rich_listbox.append(&gtk::Label::new(Some("Row 3")));

            let terminal_label = granite::HeaderLabel::new("\"terminal\" style class");

            let terminal = gtk::Label::builder()
                .label("[ 73%] Linking C executable granite-demo\n[100%] Built target granite-demo")
                .selectable(true)
                .wrap(true)
                .xalign(0.0)
                .yalign(0.0)
                .build();

            let terminal_scroll = gtk::ScrolledWindow::builder()
                .min_content_height(70)
                .child(&terminal)
                .build();

            terminal_scroll.add_css_class(granite::STYLE_CLASS_TERMINAL);

            let back_button_label = granite::HeaderLabel::new("\"back-button\" style class");

            let back_button = gtk::Button::builder()
                .label("Back Button")
                .halign(gtk::Align::Start)
                .build();

            back_button
                .style_context()
                .add_class(granite::STYLE_CLASS_BACK_BUTTON);

            let scales_label = granite::HeaderLabel::builder()
                .label("Scales")
                .secondary_text("\"warmth\" and \"temperature\" style classes")
                .build();

            let warmth_scale = gtk::Scale::builder()
                .orientation(gtk::Orientation::Horizontal)
                .adjustment(
                    &gtk::Adjustment::builder()
                        .lower(3500.0)
                        .upper(6000.0)
                        .step_increment(10.0)
                        .build(),
                )
                .draw_value(false)
                .has_origin(false)
                .hexpand(true)
                .inverted(true)
                .build();

            warmth_scale.set_value(6000.0);
            warmth_scale
                .style_context()
                .add_class(granite::STYLE_CLASS_WARMTH);

            let temperature_scale = gtk::Scale::builder()
                .orientation(gtk::Orientation::Horizontal)
                .adjustment(
                    &gtk::Adjustment::builder()
                        .lower(-16.0)
                        .upper(16.0)
                        .step_increment(1.0)
                        .build(),
                )
                .draw_value(false)
                .has_origin(false)
                .hexpand(true)
                .build();

            temperature_scale.set_value(0.0);
            temperature_scale
                .style_context()
                .add_class(granite::STYLE_CLASS_TEMPERATURE);

            let primary_color_label = granite::HeaderLabel::new("Set HeaderBar color");

            let primary_color_button =
                gtk::ColorButton::with_rgba(&gtk::gdk::RGBA::new(222.0, 222.0, 222.0, 255.0));

            let accent_color_label = granite::HeaderLabel::new("Accent colored labels and icons");

            let accent_color_icon = gtk::Image::from_icon_name("emoji-body-symbolic");
            accent_color_icon
                .style_context()
                .add_class(granite::STYLE_CLASS_ACCENT);

            let accent_color_string = gtk::Label::new(Some("Lorem ipsum dolor sit amet"));
            accent_color_string
                .style_context()
                .add_class(granite::STYLE_CLASS_ACCENT);

            let accent_color_grid = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            accent_color_grid.append(&accent_color_icon);
            accent_color_grid.append(&accent_color_string);

            let container_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(12)
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(24)
                .margin_end(24)
                .build();

            container_box.append(&card_header);
            container_box.append(&card);
            container_box.append(&richlist_label);
            container_box.append(&rich_listbox);
            container_box.append(&terminal_label);
            container_box.append(&terminal_scroll);
            container_box.append(&back_button_label);
            container_box.append(&back_button);
            container_box.append(&scales_label);
            container_box.append(&warmth_scale);
            container_box.append(&temperature_scale);
            container_box.append(&primary_color_label);
            container_box.append(&primary_color_button);
            container_box.append(&accent_color_label);
            container_box.append(&accent_color_grid);

            let scrolled = gtk::ScrolledWindow::builder().child(&container_box).build();

            obj.append(&scrolled);

            primary_color_button.connect_color_set(
                glib::clone!(@weak obj => move |primary_color_button| {
                    granite::widgets_utils_set_color_primary(
                        &obj.window(),
                        &mut primary_color_button.rgba(),
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
                            .try_into()
                            .expect("Unable to convert style provider priority"),
                    );
                }),
            );
        }
    }
    impl WidgetImpl for CssView {}
    impl BoxImpl for CssView {}
}

glib::wrapper! {
    pub struct CssView(ObjectSubclass<imp::CssView>)
        @extends gtk::Widget, gtk::Box;
}

impl CssView {
    pub fn new(window: gtk::Window) -> Self {
        glib::Object::builder().property("window", window).build()
    }
}
