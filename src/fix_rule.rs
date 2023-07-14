use std::{collections::HashSet, error::Error};

use crate::Args;

pub fn parse_fix_rule(argument: &Args) -> Result<FixRule, Box<dyn std::error::Error>> {
    get_fix_mode(argument).and_then(|fix_mode_args| get_fix_rule(argument, &fix_mode_args))
}

pub struct FixRule {
    pub mode: FixMode,
    pub add: HashSet<String>,
    pub remove: HashSet<String>,
}

#[derive(PartialEq, Debug)]
pub enum FixMode {
    Add,
    Remove,
}

#[derive(PartialEq, Debug)]
enum FixModeArgs {
    Add,
    Remove,
    AddStrict,
}

fn get_fix_mode(fix_option: &Args) -> Result<FixModeArgs, Box<dyn std::error::Error>> {
    let mode: [bool; 3] = [fix_option.add, fix_option.remove, fix_option.add_strict];

    if mode.into_iter().filter(|&x| x).count() > 1 {
        Err("You can not set multiple fix mode.".into())
    } else if fix_option.add {
        return Ok(FixModeArgs::Add);
    } else if fix_option.remove {
        return Ok(FixModeArgs::Remove);
    } else if fix_option.add_strict {
        return Ok(FixModeArgs::AddStrict);
    } else {
        return Ok(FixModeArgs::Remove);
    }
}

fn get_fix_rule(argument: &Args, fix_mode_args: &FixModeArgs) -> Result<FixRule, Box<dyn Error>> {
    let mut fix_rule = FixRule {
        mode: FixMode::Remove,
        add: HashSet::new(),
        remove: HashSet::new(),
    };

    let result_set = match fix_mode_args {
        FixModeArgs::Add => fix_rule.set_fix_rule_for_add(argument),
        FixModeArgs::Remove => fix_rule.set_fix_rule_for_remove(argument),
        FixModeArgs::AddStrict => fix_rule.set_fix_rule_for_add_strict(argument),
    };

    match result_set {
        Ok(_) => Ok(fix_rule),
        Err(err) => Err(err),
    }
}

impl FixRule {
    fn set_fix_rule_for_add(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
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

    fn set_fix_rule_for_remove(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
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

    fn set_fix_rule_for_add_strict(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
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
        self.insert("html".to_string());
        self.insert("css".to_string());
        self.insert("js".to_string());
        self.insert("ts".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_mode() {
        let args = Args {
            path: None,
            add: false,
            remove: false,
            add_strict: false,
            add_bom: None,
            remove_bom: None,
        };

        let fix_rule = parse_fix_rule(&args).unwrap();

        assert_eq!(fix_rule.mode, FixMode::Remove);
    }

    #[test]
    fn test_set_mode_as_add() {
        let args = Args {
            path: None,
            add: true,
            remove: false,
            add_strict: false,
            add_bom: None,
            remove_bom: None,
        };
        let expected_remove = vec!["html", "css", "js", "ts"]
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let fix_rule = parse_fix_rule(&args).unwrap();

        assert_eq!(fix_rule.mode, FixMode::Add);
        assert_eq!(fix_rule.remove, expected_remove);
    }

    #[test]
    fn test_set_mode_as_remove() {
        let args = Args {
            path: None,
            add: false,
            remove: true,
            add_strict: false,
            add_bom: None,
            remove_bom: None,
        };

        let fix_rule = parse_fix_rule(&args).unwrap();

        assert_eq!(fix_rule.mode, FixMode::Remove);
    }

    #[test]
    fn test_set_mode_as_add_strict() {
        let args = Args {
            path: None,
            add: false,
            remove: false,
            add_strict: true,
            add_bom: None,
            remove_bom: None,
        };

        let fix_rule = parse_fix_rule(&args).unwrap();

        assert_eq!(fix_rule.mode, FixMode::Add);
        assert_eq!(fix_rule.remove, HashSet::new());
    }

    #[test]
    fn test_set_mode_as_add_with_add_bom() {
        let args = Args {
            path: None,
            add: true,
            remove: false,
            add_strict: false,
            add_bom: Some(vec!["html".to_string()]),
            remove_bom: None,
        };

        let fix_rule_result = parse_fix_rule(&args);
        assert_eq!(fix_rule_result.is_err(), true);
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set add_bom with add mode."
        );
    }

    #[test]
    fn test_set_mode_as_remove_with_add_bom() {
        let args = Args {
            path: None,
            add: false,
            remove: true,
            add_strict: false,
            add_bom: Some(vec!["cs".to_string()]),
            remove_bom: None,
        };
        let expected_add = vec!["cs"]
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let fix_rule = parse_fix_rule(&args).unwrap();
        assert_eq!(fix_rule.mode, FixMode::Remove);
        assert_eq!(fix_rule.add, expected_add);
    }

    #[test]
    fn test_set_mode_as_add_strict_with_add_bom() {
        let args = Args {
            path: None,
            add: false,
            remove: false,
            add_strict: true,
            add_bom: Some(vec!["cs".to_string()]),
            remove_bom: None,
        };

        let fix_rule_result = parse_fix_rule(&args);
        assert!(fix_rule_result.is_err());
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set add_bom with add-strict mode."
        );
    }

    #[test]
    fn test_set_mode_as_add_with_remove_bom() {
        let args = Args {
            path: None,
            add: true,
            remove: false,
            add_strict: false,
            add_bom: None,
            remove_bom: Some(vec!["cs".to_string()]),
        };
        let expected_remove = vec!["html", "css", "js", "ts", "cs"]
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let fix_rule = parse_fix_rule(&args).unwrap();
        assert_eq!(fix_rule.mode, FixMode::Add);
        assert_eq!(fix_rule.remove, expected_remove);
    }

    #[test]
    fn test_set_mode_as_remove_with_remove_bom() {
        let args = Args {
            path: None,
            add: false,
            remove: true,
            add_strict: false,
            add_bom: None,
            remove_bom: Some(vec!["cs".to_string()]),
        };

        let result_fix_rule = parse_fix_rule(&args);
        assert!(result_fix_rule.is_err());
        assert_eq!(
            result_fix_rule.err().unwrap().to_string(),
            "You can not set remove_bom with remove mode."
        );
    }

    #[test]
    fn test_set_mode_as_add_strict_with_remove_bom() {
        let args = Args {
            path: None,
            add: false,
            remove: false,
            add_strict: true,
            add_bom: None,
            remove_bom: Some(vec!["html".to_string()]),
        };
        let expected_remove = vec!["html"]
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        let fix_rule = parse_fix_rule(&args).unwrap();
        assert_eq!(fix_rule.mode, FixMode::Add);
        assert_eq!(fix_rule.remove, expected_remove);
    }

    #[test]
    fn test_set_mode_as_add_with_add_bom_and_remove_bom() {
        let args = Args {
            path: None,
            add: true,
            remove: false,
            add_strict: false,
            add_bom: Some(vec!["html".to_string()]),
            remove_bom: Some(vec!["html".to_string()]),
        };

        let fix_rule_result = parse_fix_rule(&args);
        assert!(fix_rule_result.is_err());
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set add_bom with add mode."
        );
    }

    #[test]
    fn test_set_mode_as_remove_with_add_bom_and_remove_bom() {
        let args = Args {
            path: None,
            add: false,
            remove: true,
            add_strict: false,
            add_bom: Some(vec!["html".to_string()]),
            remove_bom: Some(vec!["html".to_string()]),
        };

        let fix_rule_result = parse_fix_rule(&args);
        assert!(fix_rule_result.is_err());
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set remove_bom with remove mode."
        );
    }

    #[test]
    fn test_set_mode_as_add_strict_with_add_bom_and_remove_bom() {
        let args = Args {
            path: None,
            add: false,
            remove: false,
            add_strict: true,
            add_bom: Some(vec!["html".to_string()]),
            remove_bom: Some(vec!["html".to_string()]),
        };

        let fix_rule_result = parse_fix_rule(&args);
        assert!(fix_rule_result.is_err());
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set add_bom with add-strict mode."
        );
    }
}
