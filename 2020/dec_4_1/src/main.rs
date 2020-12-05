use std::collections::HashMap;
use std::fs;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn print(&self) {
        println!(
            "Passport(byr={:?}, iyr={:?}, eyr={:?}, hgt={:?}, hcl={:?}, ecl={:?}, pid={:?}, cid={:?})",
            self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid, self.cid
        );
    }
}

fn main() {

    let passport_input = read_file("./data/input.txt");
    let passports = parse_passports(&passport_input);
    let mut valid_passports = 0;


    for passport in passports.iter() {
        passport.print();
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
        pw_map.insert(passport_kv[0].to_owned(), passport_kv[0].to_owned());
    }
    Passport{
        byr: pw_map.remove("byr"),
        iyr: pw_map.remove("iyr"),
        eyr: pw_map.remove("eyr"),
        hgt: pw_map.remove("hgt"),
        hcl: pw_map.remove("hcl"),
        ecl: pw_map.remove("ecl"),
        pid: pw_map.remove("pid"),
        cid: pw_map.remove("cid"),
    }
}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
