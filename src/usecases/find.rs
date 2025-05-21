use walkdir::WalkDir;

pub fn find_txt(text: &str, path: &str) -> bool {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let content = std::fs::read_to_string(entry.path()).unwrap();
        if content.contains(text) {
            return true;
        } else {
            return false;
        }
    }
    return false;
}
