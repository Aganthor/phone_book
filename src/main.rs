//
// Phone book application.
//

use std::collections::HashMap;
use std::io;

struct PhoneEntry {
    name : String,
    family_name : String,
    phone_number : String
}

struct PhoneBook {
    phone_book : HashMap<String, PhoneEntry>
}

impl PhoneBook {
    // fn new(self) -> PhoneBook {
    //     self.phone_book = HashMap<String, PhoneEntry>::new()
    // }
    fn add_contact(&mut self) {
        let mut name = String::new();
        let mut family_name = String::new();
        let mut phone_number = String::new();

        println!("Please enter the name:");
        io::stdin().read_line(&mut name).unwrap();
        println!();

        println!("Please enter the family name:");
        io::stdin().read_line(&mut family_name).unwrap();
        println!();
        
        println!("Please enter the phone number:");
        io::stdin().read_line(&mut phone_number).unwrap();        
        println!();

        let key = name.clone();
        let entry = PhoneEntry {
            name,
            family_name,
            phone_number,
        };
        self.phone_book.insert(key, entry);
    }
}

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

    let menu_choice: i32 = match input.trim().parse() {
        Ok(i) => return i,
        Err(_) => {
            println!("Invalid input. Please select a number between 1 and 6.");
            return -1
        }
    };
}


fn main() {
    let mut pb = PhoneBook {phone_book : HashMap::new()};
    let mut menu_choice = 0;

    loop {
        menu_choice = show_menu();
        match menu_choice {
            1 => pb.add_contact(),
            2 => println!("Removing contact..."),
            3 => println!("Searching contact..."),
            4 => println!("Search V2 contact..."),
            5 => println!("Print contact..."),
            6 => {
                println!("Good bye!");
                break;
            },
            _ => {}
        }
    }

}
