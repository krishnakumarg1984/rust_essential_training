use std::env;
use std::fs;

fn main() {
    if env::args().len() <= 2 {
        println!("Two input arguments must be supplied to run this program. Exiting...");
        return;
    }

    let moonwalkers_file = env::args().nth(1).unwrap();
    let name_to_check = env::args().nth(2).unwrap();

    let moonwalkers_filecontents = fs::read_to_string(moonwalkers_file).unwrap();

    for moonwalker in moonwalkers_filecontents.lines() {
        if name_to_check == moonwalker {
            println!("Found {name_to_check}!");
            return;
        }
    }
    println!("Did not find {name_to_check}!");
}
