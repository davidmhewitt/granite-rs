mod welcome_view;

use glib::clone;
use granite::traits::SettingsExt;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "io.github.davidmhewitt.granite-rs.example1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let placeholder = welcome_view::WelcomeView::new();

    let main_stack = gtk::Stack::new();
    main_stack.add_titled(&placeholder, Some("placeholder"), "Placeholder");

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

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Granite Rust Demo")
        .default_width(900)
        .default_height(600)
        .width_request(750)
        .height_request(500)
        .child(&paned)
        .build();

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
