use std::fs::{OpenOptions, self};
use std::io::{Write, Read};

fn main() {
    const ERR: &str = "something went wrong"; // const need to be explicit

    match fs::read("todos") {
        Ok(_) => {},
        Err(_) => {
            println!("Creating new file");
            fs::write("todos", "").expect(&ERR);
        },
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("todos")
        .expect(&ERR);

    let mut arr: Vec<String> = Vec::new();
    let mut input: String = String::new();
    println!("Add a todo?");
    std::io::stdin().read_line(&mut input).expect(&ERR);
    
    while input.trim().eq("y") { // need to trim newline
        input.clear();
        println!("What todo?");
        std::io::stdin().read_line(&mut input).expect(&ERR);
        beautify_todo(&mut input);
        arr.push(input.clone());
        input.clear();
        println!("Do you want to add another todo? (y/n)");
        std::io::stdin().read_line(&mut input).expect(&ERR);
    }

    for each in arr.iter() {
        write!(file, "{}", each).unwrap(); // need to make sure write and append is enabled
    }

    let mut output = String::new();
    match file.read_to_string(&mut output) { // need to make sure read mode is enabled
        Ok(_) => println!("TODO:\n{}", output),
        Err(why) => panic!("fuck {}", why),
    }
}

fn beautify_todo(input: &mut String) -> String {
    *input = String::from("TODO: ") + input; // dereferencing to mutate value
    input.to_string()
}

