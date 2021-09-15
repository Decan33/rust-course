// Project 1: Interactive bill manager
use std::io;
use std::collections::HashMap;
//
// User stories:
// * L1: I want to add bills, including the name and amount owed. ----> DONE
// * L1: I want to view existing bills. ----> DONE
// * L2: I want to remove bills. ----> DONE
// * L3: I want to edit existing bills. ----> DONE
// * L3: I want to go back if I change my mind. ----> DONE?
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

#[derive(Debug, Clone)]
struct Bill {
    name : String,
    owed_amount : f64,
}

struct Bills {
    bill_map : HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            bill_map : HashMap::new(),
        }
    }


    fn add(&mut self) {
        println!("Type a name:");
        let name = match get_input() {
            Ok(data) => data,
            Err(_) => return,
        };
        let owed_amount = match get_correct_amount() {
            Some(data) => data,
            None => return,
        };

        let bill = Bill {
            name,
            owed_amount,
        };
        self.bill_map.insert(bill.name.clone(), bill);
    }

    fn print_bills(&self) {
        for bill in self.bill_map.values() {
            println!("{:?}", bill);
        }
    }

    fn del_bill(&mut self) {
        println!("Type a name for deletion:");
        let name = match get_input() {
            Ok(data) => data,
            Err(_) => String::new(),
        };
    
        match self.bill_map.remove(name.as_str()) {
            Some(_data) => (),
            None => println!("There is no such billing"),
        };
    }

    fn edit_bill(&mut self) {
        self.print_bills();
        println!("Enter a name of the bill to edit:");
        let name = match get_input() {
            Ok(data) => data,
            Err(_e) => String::new(),
        };
        println!("Enter an amount:");
        let amount = match get_correct_amount() {
            Some(data) => data,
            None => return,
        };
        match self.bill_map.get_mut(name.as_str()) {
            Some(data) => data.owed_amount = amount,
            None => (),
        }
    }
}

fn get_correct_amount() -> Option<f64> {
    loop{
        println!("Please provide a correct amount:");
        let input = match get_input(){
            Ok(some) => some,
            Err(_) => return None,
        };
        if &input == "" {
            return None;
        }

        let parse = input.parse();

        match parse {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please try again"),
        };
    }
}

fn get_input() -> io::Result<String> {
    let mut buff = String::new();

    io::stdin().read_line(&mut buff)?;

    Ok(buff.trim().to_string())
}

fn print_menu() {
    println!("");
    println!("BILLING APP");
    println!("=============");
    println!("Please select one of the following options:");
    println!("1. Add to billing schedule");
    println!("2. Print all of the billings");
    println!("3. Delete a billing");
    println!("4. Edit a billing");
    println!("");
}

fn main() {
    let mut bills = Bills::new();
    loop{
        print_menu();
        match get_input() {
            Ok(some) => {
                match some.as_str() {
                    "1" => bills.add(),
                    "2" => bills.print_bills(),
                    "3" => bills.del_bill(),
                    "4" => bills.edit_bill(),
                    _ => break,
                }
            },
            Err(e) => println!("Error occurred: {:?}", e),
        }
    }

}
