use anyhow::Result;
use phantom_common::{Macro, config_dir};
use std::fs;
use std::path::PathBuf;

pub struct MacroStore {
    dir: PathBuf,
}

impl MacroStore {
    pub fn new(dir: Option<PathBuf>) -> Self {
        Self { dir: dir.unwrap_or_else(config_dir) }
    }
    pub fn ensure(&self) -> Result<()> {
        fs::create_dir_all(&self.dir)?;
        Ok(())
    }
    fn path(&self, id: &str) -> PathBuf { self.dir.join(format!("{}.toml", id)) }

    pub fn load_all(&self) -> Result<Vec<Macro>> {
        self.ensure()?;
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.dir)? {
            let entry = entry?;
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) == Some("toml") {
                let txt = fs::read_to_string(&p)?;
                let m: Macro = toml::from_str(&txt)?;
                out.push(m);
            }
        }
        out.sort_by(|a,b| a.id.cmp(&b.id));
        Ok(out)
    }

    pub fn save(&self, m: &Macro) -> Result<()> {
        self.ensure()?;
        let txt = toml::to_string_pretty(m)?;
        fs::write(self.path(&m.id), txt)?;
        Ok(())
    }

    pub fn remove(&self, id: &str) -> Result<()> {
        let p = self.path(id);
        if p.exists() { fs::remove_file(p)?; }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn roundtrip_store() {
        let base = temp_dir().join(format!("phantom-test-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
        let store = MacroStore::new(Some(base.clone()));
        let m = Macro {
            id: "hello".into(),
            name: "Hello".into(),
            enabled: true,
            trigger: phantom_common::Trigger::Hotkey { keys: "ctrl+shift+h".into() },
            actions: vec![phantom_common::Action::Keystroke { text: "hi".into() }],
        };
        store.save(&m).unwrap();
        let all = store.load_all().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, "hello");
        store.remove("hello").unwrap();
        assert!(store.load_all().unwrap().is_empty());
    }
}