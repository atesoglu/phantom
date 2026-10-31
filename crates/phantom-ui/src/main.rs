use gtk4::prelude::*;
use libadwaita::prelude::*;
use phantom_common::ReplayStep;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let app = libadwaita::Application::builder()
        .application_id("dev.atesoglu.phantom")
        .build();

    app.connect_activate(|app| {
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Phantom")
            .default_width(700)
            .default_height(500)
            .build();

        let v = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(8).build();
        let header = libadwaita::HeaderBar::new();
        let info = gtk4::Label::new(Some("Move mouse inside area; Start/Stop, then Replay"));
        let area = gtk4::DrawingArea::builder().height_request(300).build();
        let last = Rc::new(RefCell::new(None::<(f64, f64)>));
        let steps = Rc::new(RefCell::new(Vec::<ReplayStep>::new()));
        let recording = Rc::new(RefCell::new(false));

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
                let to_replay = steps.borrow().clone();
                status.set_text(&format!("replaying {} steps", to_replay.len()));
                let res = replay_steps(&to_replay);
                match res {
                    Ok(_) => status.set_text(&format!("replayed {} steps", to_replay.len())),
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

#[cfg(target_os = "linux")]
fn replay_steps(steps: &[ReplayStep]) -> anyhow::Result<()> {
    use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent, Key, RelativeAxisType};
    let mut rels = AttributeSet::<RelativeAxisType>::new();
    rels.insert(RelativeAxisType::REL_X);
    rels.insert(RelativeAxisType::REL_Y);
    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::BTN_LEFT);
    let mut device = VirtualDeviceBuilder::new()?
        .name("phantom-debug")
        .with_relative_axes(&rels)?
        .with_keys(&keys)?
        .build()?;
    for s in steps {
        if s.dx != 0 || s.dy != 0 {
            device.emit(&[
                InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, s.dx),
                InputEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, s.dy),
                InputEvent::new(EventType::SYNCHRONIZATION, 0, 0),
            ])?;
        }
        if s.click {
            device.emit(&[
                InputEvent::new(EventType::KEY, Key::BTN_LEFT.code(), 1),
                InputEvent::new(EventType::SYNCHRONIZATION, 0, 0),
            ])?;
            device.emit(&[
                InputEvent::new(EventType::KEY, Key::BTN_LEFT.code(), 0),
                InputEvent::new(EventType::SYNCHRONIZATION, 0, 0),
            ])?;
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn replay_steps(_steps: &[ReplayStep]) -> anyhow::Result<()> {
    anyhow::bail!("not supported on this platform")
}