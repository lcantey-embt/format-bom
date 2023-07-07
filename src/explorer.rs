use std::path::PathBuf;

/// Get all files in a directory.
pub fn get_file_list(path: &PathBuf) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = Vec::new();
    let mut dir_list: Vec<PathBuf> = Vec::new();
    dir_list.push(path.clone());
    while 0 < dir_list.len() {
        let dir = dir_list.pop().unwrap();
        let paths = std::fs::read_dir(dir).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_dir() {
                dir_list.push(path);
            } else {
                file_list.push(path);
            }
        }
    }
    file_list
}
