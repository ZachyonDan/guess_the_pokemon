use anyhow::Result;
use rand::prelude::*;
use rand::thread_rng;
use std::borrow::Cow;
use std::{thread, time};

use crate::prelude::*;
use crate::{Args, CLOSE_ENOUGH_DURATION_MS};

fn clear_terminal() {
    // Send control character to clear the terminal
    print!("{}[2J", 27 as char);
}

fn sleep_ms(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(
        || s.to_string(),
        |first_char| {
            let capitalized = if first_char.is_ascii_lowercase() {
                first_char.to_ascii_uppercase()
            } else {
                first_char
            };
            capitalized.to_string() + chars.as_str()
        },
    )
}

fn display_asset(asset: &PokemonAsset) -> Result<()> {
    let conf = viuer::Config {
        x: 2,
        y: 1,
        width: None,
        height: None,
        ..Default::default()
    };
    viuer::print_from_file(&asset.path, &conf)?;

    Ok(())
}

fn close_enough_vowel(guess: &str, answer: &str) -> bool {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    // Preallocate the correct size for the String
    let mut okayish_guess = String::with_capacity(guess.len());
    let mut okayish_answer = String::with_capacity(answer.len());

    for character in guess.chars() {
        if vowels.contains(&character) {
            okayish_guess.push(vowels[0]);
        } else {
            okayish_guess.push(character);
        }
    }

    for character in answer.chars() {
        if vowels.contains(&character) {
            okayish_answer.push(vowels[0]);
        } else {
            okayish_answer.push(character);
        }
    }

    okayish_guess == okayish_answer
}

pub fn start_game(pokemon: &[PokemonAsset], args: &Args) -> Result<()> {
    let mut correct_guesses: usize = 0;
    let mut num_skipped: usize = 0;
    let mut num_guessed: usize = 0;
    while num_guessed < args.num_guesses {
        clear_terminal();

        let random_pokemon: &PokemonAsset = if let Some(pokemon) = pokemon.choose(&mut thread_rng())
        {
            pokemon
        } else {
            break;
        };

        let pokemon_name: Box<str> = if args.exact_names {
            random_pokemon.name.clone()
        } else {
            random_pokemon.name_stem()
        };

        display_asset(random_pokemon)?;
        println!("[Pokemon {}/{}]\n", num_guessed + 1, args.num_guesses);

        let mut do_break = false;
        for i in 1..=args.attempts {
            println!("Attempt {i}/{}", args.attempts);
            let guess: String = inquire::Text::new("What pokemon is this?")
                .prompt()?
                .trim()
                .to_lowercase();
            let guess: Cow<str> = {
                if args.exact_names {
                    guess.into()
                } else {
                    // For accidental suffixes if `exact-names` is disabled
                    let split_args: Vec<&str> = guess.split('-').collect();
                    split_args[0].into()
                }
            };

            if guess == pokemon_name.as_ref() {
                println!("Correct!");
                correct_guesses += 1;
                num_guessed += 1;
                do_break = true;
            } else if close_enough_vowel(&guess, pokemon_name.as_ref()) {
                println!("Close enough! Your were off by at least one vowel.");
                println!("The exact answer was {pokemon_name}");
                println!("Your guess: {guess}");
                if args.delay < CLOSE_ENOUGH_DURATION_MS {
                    sleep_ms(CLOSE_ENOUGH_DURATION_MS - args.delay);
                }

                correct_guesses += 1;
                num_guessed += 1;
                do_break = true;
            } else if guess == "skip" || guess == "pass" {
                println!(
                    "Skipping! The correct answer was {}",
                    capitalize_first(&pokemon_name)
                );
                num_skipped += 1;
                do_break = true;
            } else if i == args.attempts {
                println!(
                    "Incorrect! The correct answer was {}",
                    capitalize_first(&pokemon_name)
                );
                num_guessed += 1;
                do_break = true;
            } else {
                println!("Incorrect!");
            }

            if do_break {
                sleep_ms(args.delay);
                break;
            }
        }
    }

    println!("\n--- Details ---");
    println!("{}/{} pokemon correct!", correct_guesses, args.num_guesses);
    if num_guessed - correct_guesses == 0 {
        println!("No pokemon guessed incorrectly!");
    } else {
        print!("{} incorrect guess", num_guessed - correct_guesses);
        // Plural
        if num_guessed - correct_guesses > 1 {
            print!("es");
        }
        println!();
    }
    if num_skipped == 0 {
        println!("Wow! You used 0 skips! Great job!");
    } else if num_skipped == 1 {
        println!("Almost 1 skip used! Maybe you can manage 0 skips next time!");
    } else {
        println!("You used {num_skipped} skips total");
    }

    Ok(())
}
