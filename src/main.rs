enum GuessState {
    Correct,
    Exists,
    Incorrect,
}

struct Guess {
    character: char,
    state: GuessState,
}

fn main() {
    let length = std::env::args().nth(1).unwrap_or_else(|| "5".to_string());
    let length = length.parse::<u32>().unwrap_or(5);
    let language = std::env::args().nth(2).unwrap_or_else(|| "en".to_string());

    let word = fetch_word(length, &language);
    let mut attempts = 6;

    let mut guesses = Vec::new();
    render(&guesses, false, &word);

    while attempts > 0 {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_ascii_lowercase();

        if guess.len() != length as usize {
            continue;
        }

        let mut guess_to_add = Vec::new();
        for (i, guess_char) in guess.chars().enumerate() {
            if word.chars().nth(i).unwrap() == guess_char {
                guess_to_add.push(Guess {
                    character: guess_char.to_ascii_uppercase(),
                    state: GuessState::Correct,
                });
            } else if word.contains(guess_char) {
                guess_to_add.push(Guess {
                    character: guess_char.to_ascii_uppercase(),
                    state: GuessState::Exists,
                });
            } else {
                guess_to_add.push(Guess {
                    character: guess_char.to_ascii_uppercase(),
                    state: GuessState::Incorrect,
                });
            }
        }

        guesses.push(guess_to_add);
        render(&guesses, guess == word, &word);
        if guess == word {
            break;
        }

        attempts -= 1;
    }
}

fn render(guesses: &[Vec<Guess>], is_win: bool, word: &str) {
    println!("{}[2J", 27 as char);
    println!("┏━━━━━━━━━━━━ WORDLEcli ━━━━━━━━━━━━┓");
    println!("┃                                   ┃");
    println!("┃   ═══ -> Correct                  ┃");
    println!("┃   ━ ━ -> Wrong position           ┃");
    println!("┃   ━━━ -> Incorrect                ┃");
    println!("┃                                   ┃");

    for i in 0..6 {
        if is_win {
            println!("┃                                   ┃");
            if i == 2 {
                println!("┃             YOU WON!!             ┃");
                println!(
                    "┃        THE WORD WAS: {}        ┃",
                    word.to_ascii_uppercase()
                );
            } else {
                println!("┃                                   ┃");
                println!("┃                                   ┃");
            }
        } else {
            println!("┃   {}  ┃", gen_line_top(guesses.get(i)));
            println!("┃   {}  ┃", get_line(guesses.get(i)));
            println!("┃   {}  ┃", gen_line_bottom(guesses.get(i)));
        }
    }

    println!("┃                                   ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");

    if !is_win {
        println!("Guess the word: ");
    }
}

fn gen_line_top(guess: Option<&Vec<Guess>>) -> String {
    if guess.is_none() {
        return String::from("┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓ ");
    }

    let mut result = String::from("");

    for guess in guess.unwrap().iter() {
        match guess.state {
            GuessState::Correct => result += "╔═══╗ ",
            GuessState::Exists => result += "┏ ━ ┓ ",
            GuessState::Incorrect => result += "┏━━━┓ ",
        }
    }
    result
}

fn gen_line_bottom(guess: Option<&Vec<Guess>>) -> String {
    if guess.is_none() {
        return String::from("┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛ ");
    }

    let mut result = String::from("");

    for guess in guess.unwrap().iter() {
        match guess.state {
            GuessState::Correct => result += "╚═══╝ ",
            GuessState::Exists => result += "┗ ━ ┛ ",
            GuessState::Incorrect => result += "┗━━━┛ ",
        }
    }
    result
}

fn get_line(guess: Option<&Vec<Guess>>) -> String {
    if guess.is_none() {
        return String::from("┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃ ");
    }

    let mut result = String::from("");

    for guess in guess.unwrap().iter() {
        match guess.state {
            GuessState::Correct => result += format!("║ {} ║ ", guess.character).as_str(),
            GuessState::Exists => result += format!("  {}   ", guess.character).as_str(),
            GuessState::Incorrect => result += format!("┃ {} ┃ ", guess.character).as_str(),
        }
    }

    result
}

fn fetch_word(length: u32, language: &str) -> String {
    let url = format!(
        "https://random-word-api.herokuapp.com/word?number=1&length={}&lang={}",
        length, language
    );
    let response = reqwest::blocking::get(&url).unwrap();
    let words: Vec<String> = response.json().unwrap();
    if words[0].chars().next().unwrap().is_uppercase() {
        return fetch_word(length, language);
    }
    words[0].to_string().to_ascii_lowercase()
}
