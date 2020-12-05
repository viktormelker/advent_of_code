use std::collections::HashMap;
use std::fs;
use regex::Regex;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_pid()
            && self.validate_hair_color()
            && self.validate_height()
            && self.validate_eye_color()
    }

    fn validate_height(&self) -> bool {
        self.hgt.is_some() && is_valid_height(self.hgt.as_ref().unwrap())
    }

    fn validate_eye_color(&self) -> bool {
        self.ecl.is_some() && is_valid_eye_color(self.ecl.as_ref().unwrap())
    }

    fn validate_hair_color(&self) -> bool {
        self.hcl.is_some() && is_valid_hair_color(self.hcl.as_ref().unwrap())
    }

    fn validate_byr(&self) -> bool {
        self.byr.is_some() && is_digit(self.byr.as_ref().unwrap(), 1920, 2002, 4)
    }

    fn validate_iyr(&self) -> bool {
        self.iyr.is_some() && is_digit(self.iyr.as_ref().unwrap(), 2010, 2020, 4)
    }

    fn validate_eyr(&self) -> bool {
        self.eyr.is_some() && is_digit(self.eyr.as_ref().unwrap(), 2020, 2030, 4)
    }

    fn validate_pid(&self) -> bool {
        self.pid.is_some() && is_digit(self.pid.as_ref().unwrap(), 0, 999999999, 9)
    }


    /*fn print(&self) {
        println!(
            "Passport(byr={:?}, iyr={:?}, eyr={:?}, hgt={:?}, hcl={:?}, ecl={:?}, pid={:?}",
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid
        );
    }*/
}

const VALID_EYE_COLORS: [&str; 7] = [
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth",
];

fn is_valid_eye_color(color: &str) -> bool {
    VALID_EYE_COLORS.contains(&color)
}

fn is_valid_height(height: &String) -> bool {
    let len = height.len();
    match &height[len-2..] {
        "cm" => {
            let cm_height = height[..len-2].parse::<u64>().ok().unwrap();
            cm_height >= 150 && cm_height <= 193
        },
        "in" => {
            let in_height = height[..len-2].parse::<u64>().ok().unwrap();
            in_height >= 59 && in_height <= 76
        },
        _ => false,
    }
}

fn is_valid_hair_color(color: &str) -> bool {
    let hcl_regex: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    hcl_regex.is_match(color)
}

fn is_digit(value: &str, min: u64, max: u64, length: u8) -> bool {
    if value.chars().count() as u8 != length {
        //println!("Found value '{}' of incorrect length, expected {}", value, length);
        return false
    }

    let value = match value.parse::<u64>() {
        Ok(number)  => number,
        Err(_) => return false,
    };

    value >= min && value <= max
}

fn main() {
    let passport_input = read_file("./data/input.txt");
    let passports = parse_passports(&passport_input);
    let mut valid_passports = 0;

    for passport in passports.iter() {
        // passport.print();
        if passport.is_valid() {
            valid_passports += 1;
        }
    }
    println!("Found {} valid passports", valid_passports);
}

fn parse_passports(passport_input: &str) -> Vec<Passport>{
    let mut current_passport: Vec<String> = vec![];
    let mut passports: Vec<Passport> = vec![];
    for line in passport_input.lines() {
        if line.is_empty() {
            passports.push(get_passport(current_passport));
            current_passport = vec![];
        }
        else {
            current_passport.push(line.to_owned());
        }
    }
    passports.push(get_passport(current_passport));
    passports
}

fn get_passport(passport_strings: Vec<String>) -> Passport {
    let mut pw_map: HashMap<String, String> = HashMap::new();
    for passport_entry in passport_strings.join(" ").split_whitespace().into_iter() {
        let passport_kv: Vec<&str> = passport_entry.split(":").collect();
        pw_map.insert(passport_kv[0].to_owned(), passport_kv[1].to_owned());
    }
    Passport{
        byr: pw_map.remove("byr"),
        iyr: pw_map.remove("iyr"),
        eyr: pw_map.remove("eyr"),
        hgt: pw_map.remove("hgt"),
        hcl: pw_map.remove("hcl"),
        ecl: pw_map.remove("ecl"),
        pid: pw_map.remove("pid"),
    }
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
