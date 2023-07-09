use std::env;

pub fn set_fix_rule(fix_option: &Option<String>) -> Result<FixRule, Box<dyn std::error::Error>> {
    let enviornment_result = get_enviornment(fix_option);
    if (enviornment_result.is_err()) {
        return Err(enviornment_result.err().unwrap());
    }
    let enviornment = enviornment_result.unwrap();

    let mut fix_rule = FixRule {
        enviornment: enviornment,
        rules: Vec::new(),
    };
    return Ok(fix_rule);
}

pub struct FixRule {
    pub enviornment: Enviornment,
    pub rules: Vec<FixRuleOfFile>,
}

pub struct FixRuleOfFile {
    pub extension: String,
    pub remove_bom: bool,
}

#[derive(PartialEq, Debug)]
pub enum Enviornment {
    Windows,
    Unix,
}

fn get_enviornment(fix_option: &Option<String>) -> Result<Enviornment, Box<dyn std::error::Error>> {
    if (fix_option.is_none()) {
        return get_os();
    } else {
        let option = fix_option.as_ref().unwrap();
        return parse_option(option);
    }
}

fn get_os() -> Result<Enviornment, Box<dyn std::error::Error>> {
    let os_type = std::env::consts::OS;
    match (os_type) {
        "windows" => Ok(Enviornment::Windows),
        "linux" | "macos" => Ok(Enviornment::Unix),
        _ => Err("Unknown OS type.".into()),
    }
}

fn parse_option(fix_option: &str) -> Result<Enviornment, Box<dyn std::error::Error>> {
    match (fix_option) {
        "windows" => Ok(Enviornment::Windows),
        "unix" => Ok(Enviornment::Unix),
        _ => Err("Unknown option.".into()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_set_fix_option_as_windows() {
        let fix_option = Some("windows".to_string());

        let fix_rule = super::set_fix_rule(&fix_option);

        assert_eq!(fix_rule.is_ok(), true);
        assert_eq!(fix_rule.unwrap().enviornment, super::Enviornment::Windows);
    }

    #[test]
    fn test_set_fix_option_as_unix() {
        let fix_option = Some("unix".to_string());

        let fix_rule = super::set_fix_rule(&fix_option);

        assert_eq!(fix_rule.is_ok(), true);
        assert_eq!(fix_rule.unwrap().enviornment, super::Enviornment::Unix);
    }

    #[test]
    fn test_can_not_set_fix_option() {
        let fix_option = Some("foo".to_string());

        let fix_rule = super::set_fix_rule(&fix_option);

        assert_eq!(fix_rule.is_err(), true);
        assert_eq!(fix_rule.err().unwrap().to_string(), "Unknown option.");
    }
}
