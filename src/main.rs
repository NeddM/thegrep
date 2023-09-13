use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) {
        println!(" - USAGE - ");
        println!("The first argument is the file name/path");
        println!("The second argument is the pattern that you want to find in the file");
        println!("");
        println!("Example: thegrep example.txt pattern");
    } else {
        grep_document(&args);
    }
}

fn grep_document(args: &Vec<String>) {
    let document = fs::read_to_string(&args[1]).unwrap();

    for line in document.lines() {
        if line.contains(&args[2]) {
            println!("{}", line);
        }
    }
}
