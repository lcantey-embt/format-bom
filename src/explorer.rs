use regex::Regex;
use std::path::PathBuf;

/// Get all files in a directory.
pub fn get_file_list(path: &PathBuf) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = Vec::new();
    let mut dir_list: Vec<PathBuf> = Vec::new();
    let gitignore_pattern = generate_gitignore_regex_patterns(&PathBuf::from(".gitignore"));

    dir_list.push(path.clone());
    while 0 < dir_list.len() {
        let dir = dir_list.pop().unwrap();
        let paths = std::fs::read_dir(dir).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if is_ignored_by_gitignore(&path, &gitignore_pattern) {
                continue;
            }
            if path.is_dir() {
                dir_list.push(path);
            } else {
                file_list.push(path);
            }
        }
    }
    file_list
}

/// Filter files by gitignore.
fn filter_by_gitignore(file_list: Vec<PathBuf>) -> Vec<PathBuf> {
    let gitignore_pattern = generate_gitignore_regex_patterns(&PathBuf::from(".gitignore"));
    file_list
        .into_iter()
        .filter(|file| !is_ignored_by_gitignore(file, &gitignore_pattern))
        .collect()
}

/// Check if a file is ignored by gitignore.
fn is_ignored_by_gitignore(file: &PathBuf, gitignore_pattern: &Vec<Regex>) -> bool {
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
    let gitignore = gitignore.split("\n");
    for line in gitignore {
        if line == "" {
            continue;
        }
        let line = line.replace(".", "\\.");
        let line = line.replace("*", ".*");
        let line = line.replace("?", ".");
        let line = format!("^{}$", line);
        let re = Regex::new(&line).unwrap();
        patterns.push(re);
    }
    patterns
}
