use std::env;

pub fn set_fix_rule(fix_option: &Option<String>) -> Result<FixRule, Box<dyn std::error::Error>> {
    let enviornment = get_os_enviornment(fix_option)?;
    let mut fix_rule = FixRule {
        enviornment: enviornment,
        files: Vec::new(),
    };
    return Ok(fix_rule);
}

pub struct FixRule {
    pub enviornment: Enviornment,
    pub files: Vec<FixRuleOfFile>,
}

pub struct FixRuleOfFile {
    pub extension: String,
    pub remove_bom: bool,
}

pub enum Enviornment {
    Windows,
    Unix,
}

fn get_os_enviornment(
    fix_option: &Option<String>,
) -> Result<Enviornment, Box<dyn std::error::Error>> {
    let mut enviornment = Enviornment::Windows;

    if (fix_option.is_none()) {
        let os_type = std::env::consts::OS;
        if (os_type == "windows") {
            enviornment = Enviornment::Windows;
        } else if (os_type == "linux" || os_type == "macos") {
            enviornment = Enviornment::Unix;
        } else {
            return Err("Unknown OS type.".into());
        }
    } else {
        let option = fix_option.as_ref().unwrap();
        if (option == "windows") {
            enviornment = Enviornment::Windows;
        } else if (option == "unix") {
            enviornment = Enviornment::Unix;
        } else {
            return Err("Unknown option.".into());
        }
    }
    Ok(enviornment)
}
