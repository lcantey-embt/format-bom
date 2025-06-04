use ignore::WalkBuilder;
use regex::Regex;
use std::path::{Path, PathBuf};

/// Get all files in a directory.
pub fn get_file_list(path: &PathBuf) -> Vec<PathBuf> {
    let walker = WalkBuilder::new(path).build();
    let mut file_list: Vec<PathBuf> = Vec::new();
    walker.for_each(|result_entry| {
        if let Ok(entry) = result_entry {
            if entry.file_type().unwrap().is_file() {
                file_list.push(entry.into_path());
            }
        }
    });
    file_list
}

#[allow(unused)]
/// Filter files by gitignore.
fn filter_by_gitignore(file_list: Vec<PathBuf>) -> Vec<PathBuf> {
    let gitignore_pattern = generate_gitignore_regex_patterns(&PathBuf::from(".gitignore"));
    file_list
        .into_iter()
        .filter(|file| !is_ignored_by_gitignore(file, &gitignore_pattern))
        .collect()
}

/// Check if a file is ignored by gitignore.
fn is_ignored_by_gitignore(file: &Path, gitignore_pattern: &Vec<Regex>) -> bool {
    for re in gitignore_pattern {
        if re.is_match(file.to_str().unwrap()) {
            return true;
        }
    }
    false
}

fn generate_gitignore_regex_patterns(gitignore_file: &PathBuf) -> Vec<Regex> {
    let mut patterns: Vec<Regex> = Vec::new();
    if !gitignore_file.exists() {
        return patterns;
    }
    let gitignore = std::fs::read_to_string(gitignore_file).unwrap();
    let gitignore = gitignore.split('\n');
    for line in gitignore {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let line = line.replace('.', "\\.");
        let line = line.replace('*', ".*");
        let line = line.replace('?', ".");
        let line = format!("^{}$", line);
        let re = Regex::new(&line).unwrap();
        patterns.push(re);
    }
    patterns
}
