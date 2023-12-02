use regex::Regex;

fn main() {
    // Create your regex
    let re: Regex = Regex::new("[a-zA-Z]+").unwrap();

    // Print your match
    println!("{}", re.find("up 5").unwrap().as_str());
}