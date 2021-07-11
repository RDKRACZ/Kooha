mod application;
mod backend;
mod config;
mod widgets;

use application::KhaApplication;
use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use gettextrs::*;
use gtk::gio;

fn main() {
    pretty_env_logger::init();

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    gtk::glib::set_application_name("Kooha");

    gst::init().expect("Unable to start gstreamer");
    gtk::init().expect("Unable to start GTK4");
    adw::init();

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = KhaApplication::new();
    app.run();
}
