pub fn print_choice(text: &str, choices: Vec<&str>) -> Result<String, std::io::Error> {
    println!(
        "{} [{}]",
        text,
        choices
            .iter()
            .filter(|c| !c.is_empty())
            .map(|c| *c)
            .collect::<Vec<&str>>()
            .join("/")
    );

    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();

        if choices.iter().any(|c| c.to_lowercase() == input) {
            break;
        }

        println!(
            "Invalid input, please enter any of the following [{}]",
            choices.join("/")
        );
    }

    Ok(input)
}
