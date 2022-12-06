#[derive(Debug)]
pub struct ParserResult {
    pub stacks: String,
    pub index: String,
    pub instructions: String,
}

pub struct Parser {}

impl Parser {
    pub fn parse_input(input: &str) -> ParserResult {
        let mut lines: Vec<&str> = input.split('\n').collect::<Vec<&str>>();
        remove_last_empty_string(&mut lines);
        let index: String = remove_spacing_lines(&mut lines);
        let mut stacks: Vec<&str> = Vec::new();
        let mut instructions: Vec<&str> = Vec::new();

        for line in lines {
            if line.starts_with("move") {
                instructions.push(line);
            } else {
                stacks.push(line);
            }
        }

        let stacks = stacks.join("\n");
        let instructions = instructions.join("\n");

        ParserResult {
            stacks,
            instructions,
            index,
        }
    }
}

fn remove_last_empty_string(lines: &mut Vec<&str>) {
    let last_index: usize = lines.len() - 1;
    if lines[last_index].is_empty() {
        lines.pop();
    }
}

fn remove_spacing_lines(lines: &mut Vec<&str>) -> String {
    let index: usize = lines.iter().position(|&l| l.is_empty()).unwrap();
    lines.remove(index);
    String::from(lines.remove(index - 1))
}
