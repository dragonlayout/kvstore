use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key was not here");
    let value= args.next().unwrap();
    println!("The key is '{}', and the value is '{}'", key, value);

    let mut database = Database::new().expect("Create db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    match database.flush() {
        Ok(()) => println!("YAY!"),
        Err(error) => println!("OH NOS! Error: {}", error),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    // new is not a method, but a associate function
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map,
            flush: false,
        })
    }

    // insert is a method, just as a special function
    fn insert(&mut self, key: String, value: String) {
        // insert will take the ownership of the argument
        self.map.insert(key, value);
    }

    fn flush(&self) -> std::io::Result<()> {
        println!("flush called");
        let mut contents = String::new();
        // . will take the ownership of map, so add &
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = self.flush();
        }
    }
}