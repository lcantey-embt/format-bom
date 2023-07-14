use std::{error::Error, path::PathBuf};

use crate::{
    fix_rule::{FixMode, FixRule},
    fixer,
};

pub fn format_bom(files: &Vec<PathBuf>, fix_rule: &FixRule) -> Result<(), Box<dyn Error>> {
    let mut formatter = BomFormatter::new(fix_rule);
    let files_to_add_bom: Vec<&PathBuf> = files
        .iter()
        .filter(|&file| fix_rule.add.contains(&get_extension(file)))
        .collect();
    formatter.register_add_bom(files_to_add_bom);

    let files_to_remove_bom: Vec<&PathBuf> = files
        .iter()
        .filter(|&file| fix_rule.remove.contains(&get_extension(file)))
        .collect();
    formatter.register_remove_bom(files_to_remove_bom);

    let files_etc: Vec<&PathBuf> = files
        .iter()
        .filter(|&file| {
            !fix_rule.add.contains(&get_extension(file))
                && !fix_rule.remove.contains(&get_extension(file))
        })
        .collect();

    match fix_rule.mode {
        FixMode::Add => formatter.register_add_bom(files_etc),
        FixMode::Remove => formatter.register_remove_bom(files_etc),
    }

    formatter.format()?;

    Ok(())
}

struct BomFormatter<'a> {
    fix_rule: &'a FixRule,
    files_to_add_bom: Vec<&'a PathBuf>,
    files_to_remove_bom: Vec<&'a PathBuf>,
}

impl<'a> BomFormatter<'a> {
    fn new(fix_rule: &'a FixRule) -> Self {
        Self {
            fix_rule,
            files_to_add_bom: Vec::new(),
            files_to_remove_bom: Vec::new(),
        }
    }

    fn register_add_bom(&mut self, file: Vec<&'a PathBuf>) {
        self.files_to_add_bom.extend(file);
    }

    fn register_remove_bom(&mut self, file: Vec<&'a PathBuf>) {
        self.files_to_remove_bom.extend(file);
    }

    fn format(&self) -> Result<(), Box<dyn Error>> {
        for file in &self.files_to_add_bom {
            fixer::add_bom(file)?;
            println!("Add BOM to {}", file.display());
        }

        for file in &self.files_to_remove_bom {
            fixer::remove_bom(file)?;
            println!("Remove BOM from {}", file.display());
        }

        Ok(())
    }
}

fn get_extension(path: &PathBuf) -> String {
    path.extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string()
}
