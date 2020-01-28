//
// Phone book application.
//

use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::OpenOptions;
use std::fmt;

struct PhoneEntry {
    name : String,
    family_name : String,
    phone_number : String
}

impl fmt::Display for PhoneEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} telephone # is {}", self.name, self.family_name, self.phone_number)?;
        Ok(())
    }
}

struct PhoneBook {
    phone_book : HashMap<String, PhoneEntry>
}

impl PhoneBook {
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

        let key = family_name.clone();
        let entry = PhoneEntry {
            name: name.trim().to_string(),
            family_name: family_name.trim().to_string(),
            phone_number: phone_number.trim().to_string(),
        };
        self.phone_book.insert(key.trim().to_string(), entry);
    }

    fn print_phone_book(&mut self) {
        if self.phone_book.len() == 0 {
            println!("Phone book is empty...");
            return;
        }
        for (_key, val) in self.phone_book.iter() {
            println!("{}", val);
        }
    }

    fn remove_contact(&mut self) {
        let mut name_input = String::new();

        println!("Please enter the contact name to remove :");
        io::stdin().read_line(&mut name_input).unwrap();

        let name = name_input.trim().to_string();

        if self.phone_book.contains_key(&name) {
            self.phone_book.remove(&name);
        } else {
            println!("Name {} not found in the phone book.", name);
        }
    }

    fn search_phone_by_name(&mut self) {
        let mut name_input = String::new();

        println!("Please enter the contact name to search :");
        io::stdin().read_line(&mut name_input).unwrap();

        let name = name_input.trim().to_string();

        if self.phone_book.contains_key(&name) {
            let result: Option<&PhoneEntry> = self.phone_book.get(&name);
            match result {
                Some(contact) => println!("Found contact \"{}\". Phone number is {}.", contact.family_name, contact.phone_number),
                None => println!("Error getting key value."),
            }
        } else {
            println!("Name {} not found in the phone book.", name);
        }        
    }

    fn search_contact_by_phone(&mut self) {
        let mut phone_input = String::new();

        println!("Please enter the phone number to search :");
        io::stdin().read_line(&mut phone_input).unwrap();

        let phone = phone_input.trim().to_string();
        let mut found = false;

        for (_key, contact) in self.phone_book.iter() {
            if contact.phone_number == phone {
                println!("The phone number ({}) is associated with {} {}.", phone, contact.name, contact.family_name);
                found = true;
                break;
            }
        }

        if !found {
            println!("Can't find any contact with this phone number ({})!", phone);
        }
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

    let _menu_choice: i32 = match input.trim().parse() {
        Ok(i) => return i,
        Err(_) => {
            println!("Invalid input. Please select a number between 1 and 6.");
            return -1
        }
    };
}

fn save_phone_book(pb : &PhoneBook) {
    let _file = match OpenOptions::new().create(true).write(true)
        .truncate(true)
        .open("phone_book.txt") {
            Ok(file) => {
                for (_key, val) in pb.phone_book.iter() {
                    writeln!(&file, "{};{};{}", val.name, val.family_name, val.phone_number).expect("Failed to write to file...");
                }
            },
            Err(e) => {
                println!("Error in saving phone book to file ({}).", e);
            },
        };
}

fn load_phone_book(pb: &mut PhoneBook) {
    let _file = match OpenOptions::new().read(true).open("phone_book.txt") {
        Ok(file) => {
            let file = BufReader::new(file);
            for line in file.lines().filter_map(|result| result.ok()) {
                let contact_info: Vec<&str> = line.split(';').collect();
                let entry = PhoneEntry {
                    name: contact_info[0].trim().to_string(),
                    family_name: contact_info[1].trim().to_string(),
                    phone_number: contact_info[2].trim().to_string(),
                };
                pb.phone_book.insert(contact_info[1].trim().to_string(), entry);
            }
        }
        Err(e) => println!("Error reading phone book content ({}).", e),
    };
}

fn main() {
    let mut pb = PhoneBook {phone_book : HashMap::new()};
    let mut menu_choice : i32;

    load_phone_book(&mut pb);

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

    save_phone_book(&pb);
}
