pub trait InputBackend {
    fn inject_relative(&mut self, dx: i32, dy: i32) -> anyhow::Result<()>;
    fn inject_click(&mut self) -> anyhow::Result<()>;
    fn start_global_capture(&mut self) -> anyhow::Result<()>;
    fn stop_global_capture(&mut self) -> anyhow::Result<()>;
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::InputBackend;
    use anyhow::Result;

    pub struct UinputBackend;
    impl InputBackend for UinputBackend {
        fn inject_relative(&mut self, _dx: i32, _dy: i32) -> Result<()> { Ok(()) }
        fn inject_click(&mut self) -> Result<()> { Ok(()) }
        fn start_global_capture(&mut self) -> Result<()> {
            // portal/compositor/provider path planned
            anyhow::bail!("global capture not implemented yet")
        }
        fn stop_global_capture(&mut self) -> Result<()> { Ok(()) }
    }
}

pub fn backend() -> Box<dyn InputBackend> {
    #[cfg(target_os = "linux")]
    { Box::new(linux::UinputBackend) }
    #[cfg(not(target_os = "linux"))]
    { panic!("unsupported platform") }
}