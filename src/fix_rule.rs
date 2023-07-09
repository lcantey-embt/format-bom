use std::collections::HashMap;

use crate::Args;

pub fn set_fix_rule(fix_option: &Args) -> Result<FixRule, Box<dyn std::error::Error>> {
    let fix_mode = match (get_fix_mode(fix_option)) {
        Ok(fix_mode) => fix_mode,
        Err(err) => return Err(err),
    };
    let fix_rule = FixRule {
        fix_mod: fix_mode,
        rules: HashMap::new(),
    };
    return Ok(fix_rule);
}

pub struct FixRule {
    pub fix_mod: FixMod,
    pub rules: HashMap<String, bool>,
}

pub struct FixRuleOfFile {
    pub extension: String,
    pub remove_bom: bool,
}

#[derive(PartialEq, Debug)]
pub enum FixMod {
    Add,
    Remove,
    AddStrict,
}

fn get_fix_mode(fix_option: &Args) -> Result<FixMod, Box<dyn std::error::Error>> {
    let mode: [bool; 3] = [fix_option.add, fix_option.remove, fix_option.add_strict];

    if mode.into_iter().filter(|&x| x).count() > 1 {
        return Err("You can not set multiple fix mode.".into());
    } else if (fix_option.add) {
        return Ok(FixMod::Add);
    } else if (fix_option.remove) {
        return Ok(FixMod::Remove);
    } else if (fix_option.add_strict) {
        return Ok(FixMod::AddStrict);
    } else {
        return Ok(FixMod::Remove);
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

        assert_eq!(fix_rule.fix_mod, FixMod::Remove);
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

        let fix_rule = set_fix_rule(&args).unwrap();

        assert_eq!(fix_rule.fix_mod, FixMod::Add);
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

        assert_eq!(fix_rule.fix_mod, FixMod::Remove);
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

        assert_eq!(fix_rule.fix_mod, FixMod::AddStrict);
    }

    #[test]
    fn test_can_not_set_fix_rule_with_multiple_fix_flags() {
        let args = Args {
            path: None,
            add: true,
            remove: false,
            add_strict: true,
            add_bom: None,
            remove_bom: None,
        };

        let fix_rule_result = set_fix_rule(&args);

        assert_eq!(fix_rule_result.is_err(), true);
        assert_eq!(
            fix_rule_result.err().unwrap().to_string(),
            "You can not set multiple fix mode."
        );
    }
}
