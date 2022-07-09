use std::io;
use std::ops::Add;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;

#[derive(Debug)]
enum GuessResult<> {
    Success,
    Failure,
    Error,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //set mutable win and loss status booleans
    let mut won = false;
    let mut lost = false;

    //send a message to show program started and is working for user
    println!("Welcome to Hangman!");

    //implement a word list and randomization to finish this 
    //
    //read word list into file var
    let file = File::open("wordlist.txt")   
        .expect("File not found!");

    //use BufReader to create file reader
    let reader = BufReader::new(file);
    let mut rng = rand::thread_rng();
    
    let rand_num = rng.gen_range(0..58109);

    //pick a random word ('noguess' is a temp fill in)
    let r_lines = reader.lines().enumerate();
    let mut hang_word = String::new();
    for (index, line) in r_lines.enumerate() {
        if rand_num == index {
            match line.1 {
                Ok(x) => {
                    hang_word = x;
                }
                _some_error => {
                    return Ok(())
                }
            }
        }
    }
    

    //record length of word for later use
    let hang_word_length: u8 = hang_word.chars().count() as u8;

    //initialize a way to test for alphabetic characters
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    //set number of lives/limbs/misses
    let mut lives = 6;

    //make an array to keep track of guesses
    let mut letter_guesses = [false; 26];

    //loop until lose or win
    while &won == &false && &lost == &false{
        //Give the empty spaces with any known letters filled in to the player
        println!("Known word so far: {}",generate_known_word_string(&hang_word,&hang_word_length,&letter_guesses,alphabet));

        //get a guess,  and give the letter or limb
        let guess_result = guess_loop(&hang_word,&mut letter_guesses, alphabet);

        //check the guess result type
        match guess_result {
            GuessResult::Success => {
                //update won state if conditions met
                println!("SUCCESS: {} more misses left!", lives);
                if check_if_won(&hang_word, &letter_guesses, &String::from(alphabet)) {
                    won = true;
                    println!("Congratulations! You succeeded! The word was: {}", &hang_word);
                }
            }
            GuessResult::Failure => {
                //update lost state if conditions met
                println!("FAILURE: {} more misses left!", lives);
                lives = lives - 1;
                if lives == 0 {
                    lost = true;
                    println!("Better luck next time! You lost! The word was: {}", hang_word);
                }
            }
            GuessResult::Error => {
                println!("ERROR: {} more misses left!", lives);
            }
        }
    }
    Ok(())
    }
    


fn generate_known_word_string(word: &String, word_length: &u8, guesses: &[bool; 26], alphabet: &str) -> String {
    let mut known_word_string: String = String::new();
    for i in 0..*word_length {
        let letter_option = word.chars().nth(i as usize);
        match letter_option {
            Some(letter) => {
                if check_if_letter_was_guessed(&letter, guesses, alphabet) {
                    known_word_string = known_word_string.add(&String::from(letter));
                }
                else {
                    known_word_string = known_word_string.add(&String::from("_"));
                }
                
            }
            None => {
                //nada
            }
        }
    }
    return known_word_string
}//generate_known_word_string(word, word_length, guesses, alphabet);

fn check_if_letter_was_guessed(letter: &char, guesses: &[bool; 26], alphabet: &str) -> bool {
    let char_num = get_alphabet_index_for_letter(letter, alphabet);
    let mut counter = 0;
    for guess in guesses {
        if counter == char_num {
            return *guess;
        }
        counter = counter + 1;
    }
    return false
}

fn get_alphabet_index_for_letter(letter: &char, alphabet: &str) -> usize {
    let mut count: usize = 0;
    for a_letter in alphabet.chars().into_iter() {
        if &a_letter == letter {
            return count
        }
        count = count + 1;
    }
    return 50
}


fn guess_loop(hang_word: &str, letter_guesses: &mut [bool; 26], alphabet: &str) -> GuessResult {
    //prompt user for their guess
    println!("Enter a guess of one letter: ");
    //get an input from the user for their guess
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();

    //check if the letter is alphabetical and that only one letter was inputted
    let result = check(guess, alphabet);

    //if right format input check the word for matches
    if result.1 {
        //check against each
        for letter in hang_word.chars().into_iter() {
            let result_char_option = &result.0.chars().nth(0);
            match result_char_option {
                Some(result_char) => {
                    if &letter == result_char {
                        println!("Your guess of '{}' was correct!", result_char);
                        letter_guesses[get_alphabet_index_for_letter(result_char, &alphabet)] = true; 
                        return GuessResult::Success;
                    }
                }
                None => {
                    //will never reach
                }
            }
        }
    }
    else {
        println!("{}","The input must be in the format of a single letter. \n");
        return GuessResult::Error
    }
    
    

    return GuessResult::Failure;
}

fn check(word: String, alphabet: &str) -> (String, bool) {
    if word.chars().count() == 2 as usize || word.chars().count() > 3 {
        println!("Please enter exactly one alphabetical character!");
        println!("You entered {} characters. Your input was {}.", word.chars().count(), word);
        return (word, false)
    }
    for guess_letter in word.chars().into_iter() {
        for alphabet_letter in alphabet.chars().into_iter() {
            if &alphabet_letter == &guess_letter {
                return (String::from(guess_letter), true)
            }
        }
    }

    println!("Please enter exactly one alphabetical character! \n");
    return (word, false)
}

fn check_if_won(word: &String, guessed: &[bool; 26], alphabet: &String) -> bool {
    let mut missed_one = false;
    for letter in word.chars().into_iter() {
        let access_point = get_alphabet_index_for_letter(&letter, alphabet);
        if guessed[access_point] == false {
            missed_one = true;
        }
    }
    return !missed_one
}