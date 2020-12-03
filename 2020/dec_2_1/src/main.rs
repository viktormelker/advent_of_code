use std::fs;

struct PasswordPolicy {
    letter: char,
    min: u32,
    max: u32
}

fn main() {
    let pw_entries = read_file("data/input.txt");
    let mut correct_passwords = 0;

    for pw_entry in pw_entries.lines() {
        let policy = get_policy(pw_entry);
        let password = get_password(pw_entry);

        if validate_password(password, policy) {
            correct_passwords = correct_passwords + 1;
        }
    }

    println!("Found {} correct passwords in the database", correct_passwords);
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents
}


fn get_policy(pw_entry: &str) -> PasswordPolicy {
    let pw_string_vector: Vec<&str> = pw_entry.split(":").collect();

    let policy_string = pw_string_vector[0];
    let policy_sizes_string: Vec<&str> = policy_string.split(" ").collect();
    let letter = policy_sizes_string[1];
    let policy_size_string: Vec<&str> = policy_sizes_string[0].split("-").collect();
    let min_string = policy_size_string[0];
    let max_string = policy_size_string[1];


    PasswordPolicy {
        letter: letter.chars().next().unwrap(),
        min: min_string.parse::<u32>().unwrap(),
        max: max_string.parse::<u32>().unwrap(),
    }
}


fn get_password(pw_entry: &str) -> &str {
    let pw_string_vector: Vec<&str> = pw_entry.split(":").collect();

    pw_string_vector[1]
}

fn validate_password(password: &str, policy: PasswordPolicy) -> bool {
    let num_occurrances = count_occurrances(policy.letter, password);

    num_occurrances >= policy.min && num_occurrances <= policy.max
}

fn count_occurrances(letter: char, text: &str) -> u32 {
   text.matches(letter).count() as u32
}
