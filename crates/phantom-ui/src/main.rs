use gtk4::prelude::*;
use libadwaita::prelude::*;

fn main() {
    let app = libadwaita::Application::builder()
        .application_id("dev.atesoglu.phantom")
        .build();

    app.connect_activate(|app| {
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Phantom")
            .default_width(800)
            .default_height(600)
            .build();

        let header = libadwaita::HeaderBar::new();
        let box_ = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).build();
        box_.append(&header);

        let label = gtk4::Label::new(Some("Phantom — macro/automation (v0 skeleton)"));
        box_.append(&label);

        window.set_content(Some(&box_));
        window.present();
    });

    app.run();
}