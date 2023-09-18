mod accel_label_view;
mod css_view;
mod date_time_picker_view;
mod dialogs_view;
mod sample_dialog;
mod settings_view;
mod welcome_view;

use glib::clone;
use granite::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "io.github.davidmhewitt.granite-rs.example1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Granite Rust Demo")
        .default_width(900)
        .default_height(600)
        .width_request(750)
        .height_request(500)
        .build();

    let accel_label_view = accel_label_view::AccelLabelView::new();
    let css_view = css_view::CssView::new(window.clone().into());
    let date_time_picker_view = date_time_picker_view::DateTimePickerView::new();
    let settings_view = settings_view::SettingsView::new();
    let dialogs_view = dialogs_view::DialogsView::new(window.clone().into());
    let placeholder = welcome_view::WelcomeView::new();

    let main_stack = gtk::Stack::new();
    main_stack.add_titled(&placeholder, Some("placeholder"), "Placeholder");
    main_stack.add_titled(&accel_label_view, Some("accel_label"), "AccelLabel");
    main_stack.add_titled(&date_time_picker_view, Some("pickers"), "Date & Time");
    main_stack.add_titled(&css_view, Some("css"), "Style Classes");
    main_stack.add_titled(&settings_view, Some("settings"), "SettingsSidebar");
    main_stack.add_titled(&dialogs_view, Some("dialogs"), "Dialogs");

    let stack_sidebar = gtk::StackSidebar::builder().stack(&main_stack).build();

    let paned = gtk::Paned::builder()
        .start_child(&stack_sidebar)
        .end_child(&main_stack)
        .resize_start_child(false)
        .shrink_start_child(false)
        .shrink_end_child(false)
        .build();

    let gtk_settings = gtk::Settings::default().unwrap();

    let mode_switch = granite::ModeSwitch::builder()
        .primary_icon_name("display-brightness-symbolic")
        .secondary_icon_name("weather-clear-night-symbolic")
        .primary_icon_tooltip_text("Light Background")
        .secondary_icon_tooltip_text("Dark Background")
        .build();

    mode_switch.set_valign(gtk::Align::Center);
    mode_switch
        .bind_property("active", &gtk_settings, "gtk-application-prefer-dark-theme")
        .bidirectional()
        .build();

    let granite_settings = granite::Settings::default().unwrap();
    gtk_settings.set_gtk_application_prefer_dark_theme(
        granite_settings.prefers_color_scheme() == granite::SettingsColorScheme::Dark,
    );

    let headerbar = gtk::HeaderBar::builder().show_title_buttons(true).build();

    headerbar.style_context().add_class("default-decoration");
    headerbar.pack_end(&mode_switch);

    window.set_child(Some(&paned));
    window.set_titlebar(Some(&headerbar));

    window.present();

    granite_settings.connect_prefers_color_scheme_notify(
        clone!(@weak gtk_settings => move |granite_settings| {
            gtk_settings.set_gtk_application_prefer_dark_theme(
                granite_settings.prefers_color_scheme() == granite::SettingsColorScheme::Dark,
            );
        }),
    );
}
