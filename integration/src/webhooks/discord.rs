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
