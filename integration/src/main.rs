use std::collections::HashMap;
use std::io::stdin;
use std::env::args;

#[tokio::main]
async fn main() {
    let mut user_id = String::new();
    let mut url = String::new();
    let args = args().enumerate();
    if args.len() != 3 {
        panic!("args too short");
    }
    for (idx, arg) in args{
        match idx {
            1 => user_id = arg,
            2 => url = arg,
            _ => (),
        }
    }

    let mut input: String = String::new();

    println!("Add a todo? (y/n)");
    stdin().read_line(&mut input).unwrap();

    let mut todos: Vec<String> = Vec::new();

    while input.trim().eq("y") {
        // need to trim newline
        input.clear();

        println!("What todo?");
        stdin().read_line(&mut input).unwrap();
        todos.push(beautify_todo(&mut input));

        input.clear();
        println!("Do you want to add another todo? (y/n)");
        stdin().read_line(&mut input).unwrap();
    }

    poke_discord(&url, &user_id, &todos).await.unwrap();
}

fn beautify_todo(input: &mut str) -> String {
    format!("TODO: {input}")
}

async fn poke_discord(url: &String, user_id: &String, todos: &Vec<String>) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut output = String::new();
    for todo in todos.into_iter(){
        output += todo;
    }
    let msg = String::from("Hello <@")
        + user_id
        + &String::from(">, you have some todos from rust\n")
        + &output;
    match client.post(url)
            .header("content-type", "application/json")
            .json(&HashMap::from([
                ("avatar_url", "https://w7.pngwing.com/pngs/114/914/png-transparent-rust-programming-language-logo-machine-learning-haskell-crab-animals-cartoon-crab.png"),
                ("content", &msg),
            ]))
            .send()
            .await?
            .text()
            .await {
        Ok(_) => (),
        Err(fuck) => panic!("{fuck}"),
    };

    Ok(())
}

#[test]
fn test_beautify_todo() {
    let mut input = String::from("test");
    assert_eq!(beautify_todo(&mut input), "TODO: test");
}
