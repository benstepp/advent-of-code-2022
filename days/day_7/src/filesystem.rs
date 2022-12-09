use crate::file::File;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Filesystem {
    directories: HashMap<String, Vec<File>>,
}

impl Filesystem {
    pub fn new(input: &str) -> Filesystem {
        let directories: HashMap<String, Vec<File>> = make_directories(input);

        Filesystem { directories }
    }

    pub fn sum_of_directories_under(&self, size: i32) -> i32 {
        let mut total: i32 = 0;

        for files in self.directories.values() {
            let mut sum: i32 = 0;
            for file in files {
                sum += file.size;
            }

            if sum < size {
                total += sum;
            }
        }

        total
    }

    pub fn make_free_space(&self, size: i32) -> i32 {
        let to_free: i32 = self.current_used_space() - (70_000_000 - size);
        self.smallest_directory_under(to_free)
    }

    fn smallest_directory_under(&self, size: i32) -> i32 {
        let mut sizes: Vec<i32> = Vec::new();

        for files in self.directories.values() {
            let mut sum: i32 = 0;
            for file in files {
                sum += file.size;
            }

            if sum >= size {
                sizes.push(sum);
            }
        }

        let mut min: i32 = sizes[0];

        for size in sizes {
            if size < min {
                min = size;
            }
        }

        min
    }

    fn current_used_space(&self) -> i32 {
        let mut sum = 0;
        for file in self.directories.get("/").unwrap() {
            sum += file.size;
        }
        sum
    }
}

fn make_directories(input: &str) -> HashMap<String, Vec<File>> {
    let lines: Vec<&str> = input.split('\n').collect::<Vec<&str>>();
    let mut current_path: Vec<String> = Vec::new();
    let mut directories: HashMap<String, Vec<File>> = HashMap::new();

    for line in &lines {
        if line.starts_with("$ cd") {
            move_path(&mut current_path, line);
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            continue;
        } else {
            add_file_to_directories(&mut directories, &current_path, line);
        }
    }

    directories
}

fn move_path(current_path: &mut Vec<String>, command_line: &str) {
    let path: &str = command_line
        .split(' ')
        .collect::<Vec<&str>>()
        .last()
        .unwrap();
    if path == "/" {
        current_path.clear();
        current_path.push(String::from("/"));
    } else if path == ".." {
        current_path.pop();
    } else {
        current_path.push(String::from(path));
    }
}

fn add_file_to_directories(
    directories: &mut HashMap<String, Vec<File>>,
    current_path: &[String],
    input: &str,
) {
    if input.is_empty() {
        return;
    }

    let file: File = File::new(input);

    for (index, _dir) in current_path.iter().enumerate() {
        let parts = &current_path[0..=index];
        let key = parts.join("");

        directories
            .entry(key)
            .and_modify(|files| files.push(file.clone()))
            .or_insert_with(|| vec![file.clone()]);
    }
}
