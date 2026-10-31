use gtk4::prelude::*;
use libadwaita::prelude::*;
use phantom_common::ReplayStep;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;

fn main() {
    let app = libadwaita::Application::builder()
        .application_id("dev.atesoglu.phantom")
        .build();

    app.connect_activate(|app| {
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Phantom")
            .default_width(500)
            .default_height(400)
            .build();

        let v = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(8).build();
        let header = libadwaita::HeaderBar::new();
        let info = gtk4::Label::new(Some("Global mouse capture via evdev (requires /dev/input access)"));
        let status = gtk4::Label::new(Some("idle"));
        let steps = Rc::new(RefCell::new(Vec::<ReplayStep>::new()));
        let recording = Arc::new(AtomicBool::new(false));
        let step_shared = Arc::new(Mutex::new(Vec::<ReplayStep>::new()));
        let handle = Rc::new(RefCell::new(None::<thread::JoinHandle<()>>));

        let btn_start = gtk4::Button::with_label("Start");
        let btn_stop = gtk4::Button::with_label("Stop");
        let btn_replay = gtk4::Button::with_label("Replay");
        let hbox = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(8).build();
        hbox.append(&btn_start);
        hbox.append(&btn_stop);
        hbox.append(&btn_replay);

        {
            let recording = recording.clone();
            let step_shared = step_shared.clone();
            let steps = steps.clone();
            let status = status.clone();
            let handle = handle.clone();
            btn_start.connect_clicked(move |_| {
                step_shared.lock().unwrap().clear();
                steps.borrow_mut().clear();
                recording.store(true, Ordering::SeqCst);
                let recording_t = recording.clone();
                let step_shared_t = step_shared.clone();
                let h = thread::spawn(move || {
                    use evdev::{Device, EventType, RelativeAxisType, Key};
                    let mut devices: Vec<Device> = Vec::new();
                    if let Ok(entries) = std::fs::read_dir("/dev/input") {
                        for e in entries.flatten() {
                            let path = e.path();
                            if let Ok(dev) = Device::open(&path) {
                                if dev.supported_events().contains(EventType::RELATIVE) {
                                    devices.push(dev);
                                }
                            }
                        }
                    }
                    while recording_t.load(Ordering::SeqCst) {
                        for dev in devices.iter_mut() {
                            if let Ok(events) = dev.fetch_events() {
                                for ev in events {
                                    if !recording_t.load(Ordering::SeqCst) { break; }
                                    let et = ev.event_type();
                                    let code = ev.code();
                                    let val = ev.value();
                                    if et == EventType::RELATIVE {
                                        if code == RelativeAxisType::REL_X.0 {
                                            step_shared_t.lock().unwrap().push(ReplayStep { dx: val, dy: 0, click: false });
                                        } else if code == RelativeAxisType::REL_Y.0 {
                                            step_shared_t.lock().unwrap().push(ReplayStep { dx: 0, dy: val, click: false });
                                        }
                                    } else if et == EventType::KEY && code == Key::BTN_LEFT.code() && val == 1 {
                                        step_shared_t.lock().unwrap().push(ReplayStep { dx: 0, dy: 0, click: true });
                                    }
                                }
                            }
                        }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                });
                *handle.borrow_mut() = Some(h);
                status.set_text("recording (global)");
            });
        }
        {
            let recording = recording.clone();
            let steps = steps.clone();
            let step_shared = step_shared.clone();
            let status = status.clone();
            btn_stop.connect_clicked(move |_| {
                recording.store(false, Ordering::SeqCst);
                if let Ok(v) = step_shared.lock() { *steps.borrow_mut() = v.clone(); }
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
        std::thread::sleep(std::time::Duration::from_millis(8));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn replay_steps(_steps: &[ReplayStep]) -> anyhow::Result<()> {
    anyhow::bail!("not supported on this platform")
}