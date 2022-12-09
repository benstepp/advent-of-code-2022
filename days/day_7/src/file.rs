#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: i32,
}

impl File {
    pub fn new(input: &str) -> File {
        let parts = input.split(' ').collect::<Vec<&str>>();

        if parts.len() != 2 {
            panic!("invalid file {}", input);
        }

        let size = parts[0].parse::<i32>().unwrap();
        let name = String::from(parts[1]);

        File { name, size }
    }
}
