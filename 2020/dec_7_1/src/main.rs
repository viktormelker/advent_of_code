use std::fs;

const COLOR_START_POS: usize = 0;
const COLOR_END_POS: usize = 2;
const CONTENT_START_POS: usize= 4;
const CONTENT_STR_QUANTITY_START_POS: usize = 0;
const CONTENT_STR_COLOR_START_POS: usize = 1;
const CONTENT_STR_COLOR_END_POS: usize = 3;


#[derive(Debug)]
struct BagContent {
    quantity: u64,
    bag_color: String
}
impl BagContent {
    fn from_contents_string(content_string: &str) -> Option<BagContent> {
        if content_string == "no other bags." {
            return None
        }
        else {
            let content_words: Vec<&str> = content_string.split_whitespace().collect();
            let quantity: u64 = content_words[CONTENT_STR_QUANTITY_START_POS].parse::<u64>().unwrap();
            let color: String = content_words[CONTENT_STR_COLOR_START_POS..CONTENT_STR_COLOR_END_POS].join(" ");
            Some(BagContent {
                quantity: quantity,
                bag_color: color
            })
        }

    }
}

#[derive(Debug)]
struct Bag {
    color: String,
    contents: Vec<BagContent>
}
impl Bag {
    fn parse_rule(rule: &str) -> Bag {
        let rule_word_vector: Vec<&str>  = rule.split_whitespace().collect::<Vec<&str>>();
        let color: String = rule_word_vector[COLOR_START_POS..COLOR_END_POS].join(" ").to_owned();
        let contents_string: String = rule_word_vector[CONTENT_START_POS..].join(" ").to_owned();

        let contents: Vec<BagContent> = contents_string
            .split(", ")
            .map(|content_string| BagContent::from_contents_string(content_string))
            .filter_map(|bag_content| bag_content)
            .collect();

        Bag {
            color: String::from(color),
            contents: contents
        }
    }

    fn can_contain(&self, bag_color: &str) -> bool {
        self.contents.iter().filter(|bc| bc.bag_color == bag_color).count() > 0
    }
}

fn main() {
    let bag_to_check = String::from("shiny gold");
    let input = read_file("./data/test_input.txt");
    let rules: Vec<&str> = input.split("\n").filter(|rule| *rule != "").collect();
    println!("Found {} rules", rules.len());

    let bags: Vec<Bag> = rules.iter().map(|rule| Bag::parse_rule(rule)).collect();

    let matching_outer_bags: Vec<&Bag> = bags
        .iter()
        .filter(|bag| bag.can_contain(&bag_to_check))
        .collect();

    for bag in bags.iter() {
        println!("Found a bag: {:?}", bag);
    }
    println!("Found {:?} bags that could contain a '{}' bag", matching_outer_bags.len(), bag_to_check);

}

fn read_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
