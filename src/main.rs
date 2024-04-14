use colored::*;
use std::io;
use passwords::PasswordGenerator;

fn main() {
    //print info message
    println!("{}", "Welcome to Nevix's Password Generator!".magenta());
    println!("{}", "------------------------------------------------------------------".magenta());
    println!("Enter the amount of passwords you want to generate:");

    //amount of passwords
    let amount_of_passwords: u32;

    //as long as we have no valid amount
    loop{
        //var to store amount of passes input
        let mut amount_of_passwords_input = String::new();

        //get the input
        io::stdin()
            .read_line(&mut amount_of_passwords_input)
            .expect("Must be a num");

        //check
        match amount_of_passwords_input.trim().parse() {
            Ok(number) => {
                amount_of_passwords = number;
                break;
            },
            Err(_) => println!("Please enter a valid number")
        };
        
    }

    //generate passwords
    let passwords: Vec<String> = generate_passwords(amount_of_passwords);

    println!("Here are your {} passwords:", amount_of_passwords);

    //print passwords
    for pass in &passwords {
        println!("{}", pass);
    }

    println!("{}", "Thank you for using Nevix's Password Generator! Cya!".magenta());
}

fn generate_passwords(amount: u32) -> Vec<String> {

    //create the pg
    let pass = PasswordGenerator{
        length: 16,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true
    };

    //generate the amount of passwords
    let passwords_result = pass.generate(amount as usize).unwrap();

    //return result
    passwords_result
}