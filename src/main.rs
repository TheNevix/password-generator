use std::io::{self, Error};
use rand::{seq::IteratorRandom, *};
use rand::seq::SliceRandom;

fn main() {
    //print info message
    println!("Welcome to Nevix's Password Generator!");
    println!("------------------------------------------------------------------");
    println!("Enter the amount of passwords you want to generate:");

    //amount of passwords
    let amount_of_passwords: u32;
    let amount_of_chars: u32;

    //ask for amount of pass
    loop {
        //get the amount
        let result = ask_input("How many passwords do you want to generate?");

        //check if okay or error
        match result {
            Ok(num ) => {
                amount_of_passwords = num;
                break;
            },
            Err(_) => println!("Please enter a valid number!")
        }
    }

    //ask for amount of chars
    loop {
        //get the amount
        let result = ask_input("How many chars should each password contain?");

        //check if okay or error
        match result {
            Ok(num ) => {
                amount_of_chars = num;
                break;
            },
            Err(_) => println!("Please enter a valid number!")
        }
    }

    let mut passwords: Vec<String> = Vec::new();

    while (passwords.len() as u32) < amount_of_passwords {
        let password = generate_password(amount_of_chars);
        passwords.push(password);
    }

    println!("Here are your {} passwords:", amount_of_passwords);

    //print passwords
    for pass in &passwords {
        println!("{}", pass);
    }

    println!("Thank you for using Nevix's Password Generator! Cya!");

    //exit
    println!("Press Enter to exit...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).expect("Failed to read input");
}

//ask for user input
fn ask_input(question: &str) -> Result<u32, Error> {

    //print the line of text
    println!("{}", question);

    //create input var
    let mut result = String::new();

    //read the input
    // Read the input
    io::stdin()
        .read_line(&mut result)
        .map(|_| {
            result.trim().parse::<u32>().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
        })
        .and_then(|res| res)
}

fn generate_password(mut char_amount: u32) -> String{
    //you always want capital letters, normal letters, numbers and special chars
    let divided_by_four: u32 =  char_amount / 4;
    let mut password = String::new();
    let mut rng = rand::thread_rng();

    //generate capital letters
    {
        let amount_of_capital_letters: u32 = rng.gen_range(1..=divided_by_four);
        let capital_letters: String = (0..amount_of_capital_letters)
        .map(|_| {
            let letter = (rng.gen_range(b'A'..=b'Z') as char).to_string();
            letter
        })
        .collect();
        char_amount = char_amount - amount_of_capital_letters;
        password.push_str(&capital_letters)
    };

    //generate lowercase letters
    {
        let amount_of_capital_letters: u32 = rng.gen_range(1..=divided_by_four);
        let capital_letters: String = (0..amount_of_capital_letters)
        .map(|_| {
            let letter = (rng.gen_range(b'a'..=b'z') as char).to_string();
            letter
        })
        .collect();
        char_amount = char_amount - amount_of_capital_letters;
        password.push_str(&capital_letters)
    }

    //generate numbers
    {
        let amount_of_capital_letters: u32 = rng.gen_range(1..=divided_by_four);
        let capital_letters: String = (0..amount_of_capital_letters)
        .map(|_| {
            let letter = (rng.gen_range(b'0'..=b'9') as char).to_string();
            letter
        })
        .collect();
        char_amount = char_amount - amount_of_capital_letters;
        password.push_str(&capital_letters)
    }

    //special chars
    {
        let special_chars = "!@#$%^&*()-_=+[{]}\\|;:'\",<.>/?";
        let amount_of_special_chars: u32 = char_amount;
        let mut special_chars_string = String::new();
        while (special_chars_string.len() as u32) < amount_of_special_chars {
            let random_char = special_chars.chars().choose(&mut rand::thread_rng()).unwrap();
            special_chars_string.push(random_char);
        }
        password.push_str(&special_chars_string)
    }

    let shuffled_password = shuffle_string(&password);
    shuffled_password
}

//shuffle generated string
fn shuffle_string(input: &String) -> String {
 let mut chars: Vec<char> = input.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}