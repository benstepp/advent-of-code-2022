pub struct Elf {
    pub section_start: i32,
    pub section_end: i32,
}

impl Elf {
    pub fn new(input: &str) -> Elf {
        let parts: Vec<&str> = input.split('-').collect::<Vec<&str>>();

        if parts.len() != 2 {
            panic!("invalid elf input")
        }

        let section_start = parts[0].parse::<i32>().unwrap();
        let section_end = parts[1].parse::<i32>().unwrap();

        Elf {
            section_start,
            section_end,
        }
    }
}
