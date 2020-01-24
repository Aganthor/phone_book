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
    fn add_contact(&self) {
        let mut name = String::new();
        let mut family_name = String::new();
        let mut phone_number = String::new();

        print!("Please enter the name:");
        io::stdin().read_line(&mut name).unwrap();

        print!("Please enter the family name:");
        io::stdin().read_line(&mut family_name).unwrap();
        
        print!("Please enter the phone number:");
        io::stdin().read_line(&mut phone_number).unwrap();        

        self.phone_book.insert(name, PhoneEntry{name, family_name, phone_number});
    }
}


fn main() {
    let mut pb : PhoneBook;

    pb.add_contact();
}
