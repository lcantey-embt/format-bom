use std::env;

pub fn set_fix_rule(fix_option: &Option<String>) -> Result<FixRule, Box<dyn std::error::Error>> {
    let enviornment = get_enviornment(fix_option)?;
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
    let mut enviornment: Enviornment;

    if (fix_option.is_none()) {
        let get_os_result = get_os();
        match get_os_result {
            Ok(enviornment_ok) => {
                enviornment = enviornment_ok;
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        let option = fix_option.as_ref().unwrap();
        let parse_option_result = parse_option(option);
        match parse_option_result {
            Ok(enviornment_ok) => {
                enviornment = enviornment_ok;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(enviornment)
}

fn get_os() -> Result<Enviornment, Box<dyn std::error::Error>> {
    let mut enviornment: Enviornment;

    let os_type = std::env::consts::OS;
    match (os_type) {
        "windows" => {
            enviornment = Enviornment::Windows;
        }
        "linux" | "macos" => {
            enviornment = Enviornment::Unix;
        }
        _ => {
            return Err("Unknown OS type.".into());
        }
    }

    Ok(enviornment)
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
