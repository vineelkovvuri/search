use std::fs;
use std::path::Path;

struct ExactFileNameMatcher<'a> {
    file_name: &'a str,
}

trait Matcher {
    fn match_path(&self, file_path: &str) -> bool;
}

impl<'a> Matcher for ExactFileNameMatcher<'a> {
    fn match_path(&self, file_path: &str) -> bool {
        file_path.ends_with(self.file_name)
    }
}

trait Consumer {
    fn consume(&mut self, file_path: &str);
}

struct ResultsCollector {
    results: Vec<String>,
}

impl Consumer for ResultsCollector {
    fn consume(&mut self, file_path: &str) {
        self.results.push(file_path.to_string())
    }
}

fn main() {
    let path = Path::new("C:\\");
    let exact_matcher = ExactFileNameMatcher {
        file_name: "kernel32.dll",
    };
    let mut consumer = ResultsCollector {
        results: Vec::new(),
    };

    search(&path, &exact_matcher, &mut consumer);

    for result in consumer.results {
        println!("{:?}", result);
    }
}

fn search(path: &Path, matcher: &dyn Matcher, consumer: &mut dyn Consumer) {
    let result = fs::read_dir(&path);
    if let Ok(entries) = result {
        for entry in entries {
            if let Ok(entry) = entry {
                let sub_path = entry.path();
                if sub_path.is_dir() {
                    let sub_path = Path::new(&sub_path);
                    search(sub_path, matcher, consumer);
                } else if matcher.match_path(sub_path.to_str().unwrap()) {
                    consumer.consume(sub_path.to_str().unwrap());
                }
            }
        }
    }
}
