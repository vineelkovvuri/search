use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("C:\\");
    search(&path, "kernel32.dll");
}

fn search(path: &Path, file_name: &str) {
    let result = fs::read_dir(&path);
    if let Ok(entries) = result {
        for entry in entries {
            if let Ok(entry) = entry {
                let sub_path = entry.path();
                if sub_path.is_dir() {
                    let sub_path = Path::new(&sub_path);
                    search(sub_path, file_name);
                } else {
                    if sub_path.ends_with(file_name) {
                        println!("path: {:?}", sub_path);
                    }
                }
            }
        }
    }
}
