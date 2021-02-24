use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("TODO saved!"),
            Err(e) => println!("An error occurred: {}", e),
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
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
        // Open the db.txt file
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        // Read the contents of db.txt into a String variable
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        // Allocate an empty HashMap
        let mut map = HashMap::new();

        // Iterate over every single line in the file db.txt
        for entries in content.lines() {
            let mut values = entries.split("\t");
            let key = values.next().expect("No key!");
            let val = values.next().expect("No value!");

            // Insert them into the HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        }

        // Return OK
        Ok(Todo { map })
    }
}
