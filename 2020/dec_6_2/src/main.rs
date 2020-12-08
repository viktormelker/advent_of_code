use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Declaration {
    answers: Vec<char>
}

impl Declaration {
    fn from_group_input(group_input: Vec<&str>) -> Declaration {
        let mut all_group_chars: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

        for input in group_input.iter() {
            let current_chars: HashSet<char> = input.chars().collect();
            // println!("Found chars {:?}", current_chars);
            all_group_chars = all_group_chars.intersection(&current_chars).copied().collect();
        }

        Declaration {
            answers: all_group_chars.into_iter().collect()
        }
    }

    fn size(&self) -> u64 {
        self.answers.len() as u64
    }
}

fn main() {
    let declaration_input = read_file("./data/test_input.txt");
    let declarations = parse_declarations(&declaration_input);

    for declaration in declarations.iter() {
        println!("Found declaration for group {:?}", declaration);
    }

    let total_answers: u64 = declarations.iter().fold(0, |acc, declaration| acc + declaration.size());
    println!("Found a total of {} answers", total_answers);
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}


fn parse_declarations(declaration_input: &str) -> Vec<Declaration> {
    let mut declarations: Vec<Declaration> = vec![];
    let declaration_groups: Vec<&str> = declaration_input.split("\n\n").collect();

    for declaration_group in declaration_groups.iter() {
        declarations.push(Declaration::from_group_input(declaration_group.split("\n").collect()));
    }

    declarations
}
