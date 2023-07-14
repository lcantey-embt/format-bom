use std::{error::Error, path::PathBuf};

use crate::{
    fix_rule::{FixMode, FixRule},
    fixer,
};

pub fn format_bom(files: &Vec<PathBuf>, fix_rule: &FixRule) -> Result<(), Box<dyn Error>> {
    let mut formatter = BomFormatter::new(fix_rule);
    formatter.register_files(files);

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

    fn register_files(&mut self, files: &'a Vec<PathBuf>) {
        self.register_add_bom(&files);
        self.register_remove_bom(&files);

        let files_etc: Vec<&PathBuf> = files
            .iter()
            .filter(|file| {
                !self.fix_rule.add.contains(&get_extension(file))
                    && !self.fix_rule.remove.contains(&get_extension(file))
            })
            .collect();

        match self.fix_rule.mode {
            FixMode::Add => self.files_to_add_bom.extend(files_etc),
            FixMode::Remove => self.files_to_remove_bom.extend(files_etc),
        }
    }

    fn register_add_bom(&mut self, files: &'a Vec<PathBuf>) {
        let files_to_add_bom: Vec<&PathBuf> = files
            .iter()
            .filter(|&file| self.fix_rule.add.contains(&get_extension(file)))
            .collect();
        self.files_to_add_bom.extend(files_to_add_bom);
    }

    fn register_remove_bom(&mut self, files: &'a Vec<PathBuf>) {
        let files_to_remove_bom: Vec<&PathBuf> = files
            .iter()
            .filter(|&file| self.fix_rule.remove.contains(&get_extension(file)))
            .collect();
        self.files_to_remove_bom.extend(files_to_remove_bom);
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
