/* Library for the Wordle App */

use reqwest::*;
use chrono::{Datelike};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Green,
    Yellow,
    Gray
}

pub async fn get_daily_word() -> std::result::Result<String, reqwest::Error> {

    let today_date = chrono::Utc::now();

    let fetch_word = get(format!("https://www.nytimes.com/svc/wordle/v2/{:04}-{:02}-{:02}.json",
            today_date.year(),
            today_date.month(),
            today_date.day())
        )
        .await?
        .text()
        .await?;


    // It's better to remove the expects and use if lets because
    // we don't want our function to crash and just return an error
    // if it happens, and then handle it in the main program
    let word = fetch_word.split(",")
        .find(|x| x.contains("solution"))
        .expect("Couldn't find solution")
        .split_once(":")
        .expect("Split failed")
        .1
        [1..6]
        .to_owned();

    Ok(word)
}


pub fn check_word(input : &str, word : &str) -> Vec<Color>
{
    input.trim().chars().zip(word.chars())
        .map(|(i, w)| {
            if i == w {
                return Color::Green
            } else if word.contains(i) {
                return Color::Yellow
            } else {
                return Color::Gray
            }
        })
        .collect()
}
