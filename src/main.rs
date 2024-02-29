use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("C:\\");
    search(&path);
}

fn search(path: &Path)
{
    let result = fs::read_dir(&path);
    match result {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let sub_path = entry.path();
                        println!("path: {:?}", sub_path);
                        if sub_path.is_dir() {
                            let sub_path = Path::new(&sub_path);
                            search(sub_path);
                        }
                    },
                    Err(err) => println!("path: {}", err),
                }
            }
        },
        Err(error) => println!("Error: {}", error)
    }
}