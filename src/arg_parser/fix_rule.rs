use crate::Args;

use super::fix_mode::FixMode;
use std::{collections::HashSet, error::Error};

pub struct FixRule {
    pub mode: FixMode,
    pub add: HashSet<String>,
    pub remove: HashSet<String>,
}

impl FixRule {
    pub fn set_fix_rule_for_add(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
        if argument.add_bom.is_some() {
            return Err("You can not set add_bom with add mode.".into());
        }

        if let Some(remove_bom) = &argument.remove_bom {
            for ext in remove_bom {
                self.remove.insert(ext.to_string());
            }
        }
        self.remove.remove_default();
        self.mode = FixMode::Add;
        Ok(())
    }

    pub fn set_fix_rule_for_remove(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
        if argument.remove_bom.is_some() {
            return Err("You can not set remove_bom with remove mode.".into());
        }

        if let Some(add_bom) = &argument.add_bom {
            for ext in add_bom {
                self.add.insert(ext.to_string());
            }
        }

        self.mode = FixMode::Remove;
        Ok(())
    }

    pub fn set_fix_rule_for_add_strict(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
        if argument.add_bom.is_some() {
            return Err("You can not set add_bom with add-strict mode.".into());
        }

        if let Some(remove_bom) = &argument.remove_bom {
            for ext in remove_bom {
                self.remove.insert(ext.to_string());
            }
        }

        self.mode = FixMode::Add;
        Ok(())
    }
}

trait HashSetExt {
    fn remove_default(&mut self);
}

impl HashSetExt for HashSet<String> {
    fn remove_default(&mut self) {
        let defaults = vec![
            "html", "css", "svg", // web development
            "js", "ts", // programming language
            "md", // document
            "json", "toml", "yaml", "csv", "xml", // data
            "ini", "conf", "cfg", // config
            "sh", "bat", "ps1", // other
        ];

        for default in defaults {
            self.insert(default.to_string());
        }
    }
}
