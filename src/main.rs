use gtk::prelude::*;
use gtk::{gio, glib};

const APP_ID: &str = "cz.k8r.sheetleague";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &adw::Application) {
//    let settings = gio::Settings::new(APP_ID);

    let is_switch_enabled = false; // settings.boolean("is-switch-enabled");

    let switch = gtk::Switch::builder()
        .margin_bottom(50)
        .margin_top(50)
        .margin_start(50)
        .margin_end(50)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .state(is_switch_enabled)
        .build();

    switch.set_active(is_switch_enabled);

    // switch.connect_state_set(move |_, is_enabled| {
    //     settings
    //         .set_boolean("is-switch-enabled", is_enabled)
    //         .expect("Could not save state");
    //     glib::Propagation::Proceed
    // });

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Sheet League 3000")
        .child(&switch)
        .build();

    window.present();

    let dialog = gtk::FileDialog::builder()
        .title("Vyber soubor")
        //        .initial_folder(std::path::Path::from("/tmp"))
        .build();

    let alert = gtk::AlertDialog::builder()
        .modal(true)
        .buttons(vec!["OK"])
        .build();

    dialog.open(Some(&window), gio::Cancellable::NONE, glib::clone!(
        #[weak] window,
        move |res| {
        match res {
            Err(s) => {
                alert.set_message(&s.to_string());
                alert.show(Some(&window));
            }
            Ok(file) => {
                println!("{:?}", dbg!(file));
            }
        };
    }));
}
