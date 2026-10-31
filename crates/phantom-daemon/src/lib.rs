use anyhow::Result;
use phantom_common::{ReplayStep, Request, Response, socket_path};
use phantom_core::MacroStore;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tokio::time::Duration;

pub async fn run() -> Result<()> {
    let store = Arc::new(MacroStore::new(None));
    store.ensure()?;
    let sock = socket_path();
    let _ = std::fs::remove_file(&sock);
    let listener = UnixListener::bind(&sock)?;
    tracing::info!("phantom daemon listening on {:?}", sock);
    loop {
        let (stream, _) = listener.accept().await?;
        let store = store.clone();
        tokio::spawn(async move {
            let (r, mut w) = stream.into_split();
            let mut reader = BufReader::new(r);
            let mut line = String::new();
            if reader.read_line(&mut line).await.is_ok() {
                let req: Request = match serde_json::from_str(&line) {
                    Ok(r) => r,
                    Err(e) => { let _ = w.write_all(format!("{}\n", serde_json::to_string(&Response::Error { message: e.to_string() }).unwrap()).as_bytes()).await; return; }
                };
                let resp = handle(req, &store).await;
                let _ = w.write_all(format!("{}\n", serde_json::to_string(&resp).unwrap()).as_bytes()).await;
            }
        });
    }
}

async fn handle(req: Request, store: &MacroStore) -> Response {
    match req {
        Request::ListMacros => match store.load_all() {
            Ok(items) => Response::Macros { items },
            Err(e) => Response::Error { message: e.to_string() },
        },
        Request::ToggleMacro { id, enabled } => {
            match store.load_all() {
                Ok(mut items) => {
                    if let Some(m) = items.iter_mut().find(|m| m.id == id) {
                        m.enabled = enabled;
                        if let Err(e) = store.save(m) { return Response::Error { message: e.to_string() }; }
                        Response::Ok
                    } else { Response::Error { message: "not found".into() } }
                }
                Err(e) => Response::Error { message: e.to_string() },
            }
        }
        Request::AddMacro { macro_def } => match store.save(&macro_def) {
            Ok(_) => Response::Ok,
            Err(e) => Response::Error { message: e.to_string() },
        },
        Request::RemoveMacro { id } => match store.remove(&id) {
            Ok(_) => Response::Ok,
            Err(e) => Response::Error { message: e.to_string() },
        },
        Request::DaemonStatus => {
            let count = store.load_all().map(|v| v.len()).unwrap_or(0);
            Response::Status { running: true, macros: count }
        }
        Request::DebugReplay { steps } => {
            if let Err(e) = replay_steps(&steps).await {
                Response::Error { message: e.to_string() }
            } else { Response::Ok }
        }
    }
}

#[cfg(target_os = "linux")]
async fn replay_steps(steps: &[ReplayStep]) -> Result<()> {
    use evdev::{Device, InputEvent, KeyCode, RelativeAxisType, RelativeAxisCode};
    // Build virtual device
    let mut device = Device::builder()
        .name("phantom-debug")
        .with_rel_axes(&[RelativeAxisType::REL_X, RelativeAxisType::REL_Y])?
        .with_keys(&[KeyCode::BTN_LEFT])?
        .build()?;
    for s in steps {
        if s.dx != 0 || s.dy != 0 {
            device.emit(&[
                InputEvent::new(evdev::EventType::RELATIVE, RelativeAxisType::REL_X.0, s.dx),
                InputEvent::new(evdev::EventType::RELATIVE, RelativeAxisType::REL_Y.0, s.dy),
                InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
            ])?;
        }
        if s.click {
            device.emit(&[
                InputEvent::new(evdev::EventType::KEY, KeyCode::BTN_LEFT.0, 1),
                InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
            ])?;
            device.emit(&[
                InputEvent::new(evdev::EventType::KEY, KeyCode::BTN_LEFT.0, 0),
                InputEvent::new(evdev::EventType::SYNCHRONIZATION, 0, 0),
            ])?;
        }
        tokio::time::sleep(Duration::from_millis(8)).await;
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
async fn replay_steps(_steps: &[ReplayStep]) -> Result<()> {
    anyhow::bail!("not supported on this platform")
}