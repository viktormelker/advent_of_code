use std::fs;

struct Declaration {
    answers: Vec<char>
}

impl Declaration {
    fn from_group_input(group_input: Vec<String>) -> Declaration {
        let mut chars: Vec<char> = vec![];
        for word in group_input.iter() {
            chars.extend(word.chars());
        }
        chars.sort();
        chars.dedup();

        Declaration {
            answers: chars
        }
    }

    fn size(&self) -> u64 {
        self.answers.len() as u64
    }
}

fn main() {
    let declaration_input = read_file("./data/input.txt");
    let declarations = parse_declarations(&declaration_input);

    let total_answers: u64 = declarations.iter().fold(0, |acc, declaration| acc + declaration.size());
    println!("Found a total of {} answers", total_answers);
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}


fn parse_declarations(declaration_input: &str) -> Vec<Declaration> {
    let mut current_declaration: Vec<String> = vec![];
    let mut declarations: Vec<Declaration> = vec![];

    for line in declaration_input.lines() {
        if line.is_empty() {
            declarations.push(Declaration::from_group_input(current_declaration));
            current_declaration = vec![];
        }
        else {
            current_declaration.push(line.to_owned());
        }
    }
    declarations.push(Declaration::from_group_input(current_declaration));
    declarations
}
