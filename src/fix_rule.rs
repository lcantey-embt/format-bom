use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::Args;

pub fn set_fix_rule(argument: &Args) -> Result<FixRule, Box<dyn std::error::Error>> {
    let fix_mode_args = match (get_fix_mode(argument)) {
        Ok(fix_mode_args) => fix_mode_args,
        Err(err) => return Err(err),
    };
    let fix_rule = match (get_fix_rule(argument, &fix_mode_args)) {
        Ok(fix_rule) => fix_rule,
        Err(err) => return Err(err),
    };
    return Ok(fix_rule);
}

pub struct FixRule {
    pub mode: FixMode,
    pub add: HashSet<String>,
    pub remove: HashSet<String>,
}

impl FixRule {
    fn set_fix_rule_for_add(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
        if argument.add_bom.is_some() {
            return Err("You can not set add_bom with add mode.".into());
        }

        if let Some(remove_bom) = &argument.remove_bom {
            let mut remove_bom_hashset = remove_bom
                .iter()
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            self.remove.union(&remove_bom_hashset);
        }

        self.remove.insert("html".to_string());
        self.remove.insert("css".to_string());
        self.remove.insert("js".to_string());
        self.remove.insert("ts".to_string());

        self.mode = FixMode::Add;
        return Ok(());
    }

    fn set_fix_rule_for_remove(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
        if argument.remove_bom.is_some() {
            return Err("You can not set add_bom with add mode.".into());
        }

        if let Some(add_bom) = &argument.add_bom {
            let mut add_bom_hashset = add_bom
                .iter()
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            self.add.union(&add_bom_hashset);
        }

        self.mode = FixMode::Remove;
        return Ok(());
    }

    fn set_fix_rule_for_add_strict(&mut self, argument: &Args) -> Result<(), Box<dyn Error>> {
        if argument.add_bom.is_some() {
            return Err("You can not set add_bom with add mode.".into());
        }

        if let Some(remove_bom) = &argument.remove_bom {
            let mut remove_bom_hashset = remove_bom
                .iter()
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            self.remove.union(&remove_bom_hashset);
        }

        self.mode = FixMode::Add;
        return Ok(());
    }
}

pub struct FixRuleOfFile {
    pub extension: String,
    pub remove_bom: bool,
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

fn get_fix_rule(argument: &Args, fix_mode_args: &FixModeArgs) -> Result<FixRule, Box<dyn Error>> {
    let mut fix_rule = FixRule {
        mode: FixMode::Remove,
        add: HashSet::new(),
        remove: HashSet::new(),
    };

    let result_set = match (fix_mode_args) {
        FixModeArgs::Add => fix_rule.set_fix_rule_for_add(argument),
        FixModeArgs::Remove => fix_rule.set_fix_rule_for_remove(argument),
        FixModeArgs::AddStrict => fix_rule.set_fix_rule_for_add_strict(argument),
    };

    return match result_set {
        Ok(_) => Ok(fix_rule),
        Err(err) => Err(err),
    };
}

fn get_fix_rule_remove(argument: &Args) -> Result<FixRule, Box<dyn Error>> {
    if argument.remove_bom.is_some() {
        return Err("You can not set add_bom with add mode.".into());
    }

    let mut add: HashSet<String> = HashSet::new();
    let mut remove: HashSet<String> = HashSet::new();

    if let Some(remove_bom) = &argument.remove_bom {
        let mut remove_bom_hashset = remove_bom
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        remove.union(&remove_bom_hashset);
    }

    remove.insert("html".to_string());
    remove.insert("css".to_string());
    remove.insert("js".to_string());
    remove.insert("ts".to_string());

    let fix_rule = FixRule {
        mode: FixMode::Add,
        add,
        remove,
    };
    return Ok(fix_rule);
}

fn get_fix_mode(fix_option: &Args) -> Result<FixModeArgs, Box<dyn std::error::Error>> {
    let mode: [bool; 3] = [fix_option.add, fix_option.remove, fix_option.add_strict];

    if mode.into_iter().filter(|&x| x).count() > 1 {
        return Err("You can not set multiple fix mode.".into());
    } else if (fix_option.add) {
        fn get_fix_rule(
            argument: &Args,
            fix_mode_args: &FixModeArgs,
        ) -> Result<FixRule, Box<dyn Error>> {
            let mut fix_rule = FixRule {
                mode: FixMode::Remove,
                add: HashSet::new(),
                remove: HashSet::new(),
            };

            let result_set = match (fix_mode_args) {
                FixModeArgs::Add => fix_rule.set_fix_rule_for_add(argument),
                FixModeArgs::Remove => fix_rule.set_fix_rule_for_remove(argument),
                FixModeArgs::AddStrict => fix_rule.set_fix_rule_for_add_strict(argument),
            };

            return match result_set {
                Ok(_) => Ok(fix_rule),
                Err(err) => Err(err),
            };
        }

        fn get_fix_rule_remove(argument: &Args) -> Result<FixRule, Box<dyn Error>> {
            if argument.remove_bom.is_some() {
                return Err("You can not set add_bom with add mode.".into());
            }

            let mut add: HashSet<String> = HashSet::new();
            let mut remove: HashSet<String> = HashSet::new();

            if let Some(remove_bom) = &argument.remove_bom {
                let mut remove_bom_hashset = remove_bom
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<HashSet<String>>();
                remove.union(&remove_bom_hashset);
            }

            remove.insert("html".to_string());
            remove.insert("css".to_string());
            remove.insert("js".to_string());
            remove.insert("ts".to_string());

            let fix_rule = FixRule {
                mode: FixMode::Add,
                add,
                remove,
            };
            return Ok(fix_rule);
        }
        return Ok(FixModeArgs::Add);
    } else if (fix_option.remove) {
        return Ok(FixModeArgs::Remove);
    } else if (fix_option.add_strict) {
        return Ok(FixModeArgs::AddStrict);
    } else {
        return Ok(FixModeArgs::Remove);
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

        let fix_rule = set_fix_rule(&args).unwrap();

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

        let fix_rule = set_fix_rule(&args).unwrap();

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

        let fix_rule = set_fix_rule(&args).unwrap();

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

        let fix_rule = set_fix_rule(&args).unwrap();

        assert_eq!(fix_rule.mode, FixMode::Add);
        assert_eq!(fix_rule.remove, HashSet::new());
    }

    #[test]
    fn test_can_not_set_mode_as_add_with_add_bom() {
        let args = Args {
            path: None,
            add: true,
            remove: false,
            add_strict: false,
            add_bom: Some(vec!["html".to_string()]),
            remove_bom: None,
        };

        let fix_rule_result = set_fix_rule(&args);
        assert_eq!(fix_rule_result.is_err(), true);
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set add_bom with add mode."
        );
    }

    // #[test]
    // fn test_set_mode_as_remove_with_add_bom() {
    //     let args = Args {
    //         path: None,
    //         add: false,
    //         remove: true,
    //         add_strict: false,
    //         add_bom: Some(vec!["cs".to_string()]),
    //         remove_bom: None,
    //     };
    //     let expected_add = vec!["cs"]
    //         .iter()
    //         .map(|x| x.to_string())
    //         .collect::<HashSet<String>>();

    //     let fix_rule = set_fix_rule(&args).unwrap();
    //     assert_eq!(fix_rule.mode, FixMode::Remove);
    //     assert_eq!(fix_rule.add, expected_add);
    // }
}
