//
// Phone book application.
//
use std::io;
use phone_book::{self, PhoneBook};

fn show_menu() -> i32 {
    println!("Welcome to the phone book application version 0.1.");
    println!("\t 1- Add contact.");
    println!("\t 2- Remove a contact.");
    println!("\t 3- Search contact by name.");
    println!("\t 4- Search contact by phone number.");
    println!("\t 5- Print phone book entries.");
    println!("\t 6- Quit.");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input from stdin...");

    let _menu_choice: i32 = match input.trim().parse() {
        Ok(i) => return i,
        Err(_) => {
            println!("Invalid input. Please select a number between 1 and 6.");
            return -1
        }
    };
}

fn main() {
    let mut pb = PhoneBook::new();
    let mut menu_choice : i32;

    phone_book::load_phone_book(&mut pb);

    loop {
        menu_choice = show_menu();
        match menu_choice {
            1 => pb.add_contact(),
            2 => pb.remove_contact(),
            3 => pb.search_phone_by_name(),
            4 => pb.search_contact_by_phone(),
            5 => pb.print_phone_book(),
            6 => {
                println!("Good bye!");
                break;
            },
            _ => {}
        }
    }

    phone_book::save_phone_book(&pb);
}
