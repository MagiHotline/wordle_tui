use wordle_tui::{check_word, get_daily_word, Color};

#[tokio::main]
async fn main() {

    let word = get_daily_word().await.expect("Couldn't get daily word");

    let mut tries = 6;
    let mut input : String = String::new();

    while tries != 0
    {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        println!("{}", tries);

        if input.trim().len() != 5 { continue; }
        tries -= 1;

        let colors = check_word(&input, &word);

        println!("{:?}", colors);

        if colors.iter().all(|&c| c == Color::Green) {
            println!("You won!");
            break;
        }
    }

    println!("The word was: {}", word);
}
