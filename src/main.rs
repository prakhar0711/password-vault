mod password_entry;

use crate::password_entry::prompt;
use crate::password_entry::read_passwords_from_file;
use crate::password_entry::ServiceInfo;
fn clr() {
    // print!("{}[2J", 27 as char);
    print!("\x1B[H\x1B[2J");
}

fn main() {
    clr();
    let ascii = r#"
  _____                                           _  __      __            _  _
 |  __ \                                         | | \ \    / /           | || |
 | |__) |__ _  ___  ___ __      __ ___   _ __  __| |  \ \  / /__ _  _   _ | || |_
 |  ___// _` |/ __|/ __|\ \ /\ / // _ \ | '__|/ _` |   \ \/ // _` || | | || || __|
 | |   | (_| |\__ \\__ \ \ V  V /| (_) || |  | (_| |    \  /| (_| || |_| || || |_
 |_|    \__,_||___/|___/  \_/\_/  \___/ |_|   \__,_|     \/  \__,_| \__,_||_| \__|
    "#;
    println!("{ascii}");
    loop {
        println!("Password Vault Menu");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service : "),
                    prompt("Username : "),
                    prompt("Password : "),
                );
                println!("Entry Added Successfully!");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords from file: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Services = {}
                        - Username : {}
                        - Password : {}\n",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords from file: {}", err);
                    Vec::new()
                });
                let search = prompt("Search(Enter Service Name) : ");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Services = {}
                            - Username : {}
                            - Password : {}\n",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("See you again!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
        println!("\n");
    }
}
