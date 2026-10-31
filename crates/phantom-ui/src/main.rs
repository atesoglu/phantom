use gtk4::prelude::*;
use libadwaita::prelude::*;
use phantom_common::{ReplayStep, socket_path};
use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::rc::Rc;

fn main() {
    let app = libadwaita::Application::builder()
        .application_id("dev.atesoglu.phantom")
        .build();

    app.connect_activate(|app| {
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Phantom debug")
            .default_width(700)
            .default_height(500)
            .build();

        let v = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(8).build();
        let header = libadwaita::HeaderBar::new();
        let info = gtk4::Label::new(Some("Move mouse inside box; Start/Stop, then Replay"));
        let area = gtk4::DrawingArea::builder().height_request(300).build();
        let last = Rc::new(RefCell::new(None::<(f64, f64)>));
        let steps = Rc::new(RefCell::new(Vec::<ReplayStep>::new()));
        let recording = Rc::new(RefCell::new(false));

        // Motion capture
        {
            let last = last.clone();
            let steps = steps.clone();
            let recording = recording.clone();
            let motion = gtk4::EventControllerMotion::new();
            motion.connect_motion(move |_, x, y| {
                if *recording.borrow() {
                    if let Some((lx, ly)) = *last.borrow() {
                        let dx = (x - lx) as i32;
                        let dy = (y - ly) as i32;
                        if dx != 0 || dy != 0 {
                            steps.borrow_mut().push(ReplayStep { dx, dy, click: false });
                        }
                    }
                    *last.borrow_mut() = Some((x, y));
                }
            });
            area.add_controller(motion);
        }

        // Click capture
        {
            let steps = steps.clone();
            let recording = recording.clone();
            let g = gtk4::GestureClick::new();
            g.connect_pressed(move |_, _, _, _| {
                if *recording.borrow() {
                    steps.borrow_mut().push(ReplayStep { dx: 0, dy: 0, click: true });
                }
            });
            area.add_controller(g);
        }

        let status = gtk4::Label::new(Some("idle"));
        let btn_start = gtk4::Button::with_label("Start");
        let btn_stop = gtk4::Button::with_label("Stop");
        let btn_replay = gtk4::Button::with_label("Replay");
        let hbox = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(8).build();
        hbox.append(&btn_start);
        hbox.append(&btn_stop);
        hbox.append(&btn_replay);

        {
            let recording = recording.clone();
            let last = last.clone();
            let steps = steps.clone();
            let status = status.clone();
            btn_start.connect_clicked(move |_| {
                steps.borrow_mut().clear();
                *last.borrow_mut() = None;
                *recording.borrow_mut() = true;
                status.set_text("recording");
            });
        }
        {
            let recording = recording.clone();
            let status = status.clone();
            btn_stop.connect_clicked(move |_| {
                *recording.borrow_mut() = false;
                status.set_text("stopped");
            });
        }
        {
            let steps = steps.clone();
            let status = status.clone();
            btn_replay.connect_clicked(move |_| {
                let payload = serde_json::json!({"kind":"debug_replay","steps": *steps.borrow()});
                let resp = (|| -> anyhow::Result<String> {
                    let mut stream = UnixStream::connect(socket_path())?;
                    writeln!(stream, "{}", payload)?;
                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();
                    reader.read_line(&mut line)?;
                    Ok(line)
                })();
                match resp {
                    Ok(line) => status.set_text(&format!("daemon: {}", line.trim())),
                    Err(e) => status.set_text(&format!("error: {}", e)),
                }
            });
        }

        v.append(&header);
        v.append(&info);
        v.append(&area);
        v.append(&hbox);
        v.append(&status);
        window.set_content(Some(&v));
        window.present();
    });

    app.run();
}