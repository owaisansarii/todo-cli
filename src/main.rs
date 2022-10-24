use std::str::FromStr;
use std::{collections::HashMap, io::Read};
fn main() {
    /*
     * std::env::args() is a function brought in from the env module of the standard libray that returns
     * the arguments that the program was started with.
     * Since it's an iterator we can access the value stored at each position with the nth() function.
     * The Argument at position 0 is the program itself, which is why we start reading from the 1st element.
     */
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?},{:?}", action, item); // placeholder {:?} or {:#?} is used for Debug print.

    let mut todo = Todo::new().expect("Initialisation of db failed"); // expect() is used to handle the error

    /*
     * If the action is add we are adding the item to the todo list.
     * And saving it to db.txt
     * Else if the action is complete we are making the boolean value of the item false and saving it to db.txt
     */
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved!"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("{}, is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurde {}", why),
            },
        }
    } else if action == "remove" {
        match todo.remove(&item) {
            None => println!("{}, is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurde {}", why),
            },
        }
    } else if action == "list" {
        todo.list();
    } else {
        println!("Unknown action");
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true) // open the file in write mode
            .create(true) // create the file if it doesn't exist
            .read(true) // open the file in read mode
            .open("db.txt")?; // open the file db.txt
        let mut content = String::new();
        f.read_to_string(&mut content)?; // ? is used to handle the error
        let map: HashMap<String, bool> = content // content is the string that we read from the file
            .lines() // lines() is used to split the string into lines
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>()) // splitn() is used to split the string into 2 parts
            .map(|v| (v[0], v[1])) // map() is used to map the values to a tuple
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap())) // map() is used to map the values to a tuple
            .collect(); // collect() is used to collect the values into a HashMap
        Ok(Todo { map }) // return the Todo struct
    }

    /*
     *Complete function is used to mark the item as completed.
     */
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }

    /*
     * remove function is used to remove the item from the list.
     */
    fn remove(&mut self, key: &String) -> Option<()> {
        match self.map.remove(key) {
            Some(_) => Some(()),
            None => None,
        }
    }

    /*
     *list function is used to list the items in the list.
     */
    fn list(&self) {
        for (k, v) in &self.map {
            println!("{}: {}", k, v);
        }
    }
}
