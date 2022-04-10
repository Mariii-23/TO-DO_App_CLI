extern crate serde_json;

pub mod todo_list {
    use serde::{Deserialize, Serialize};

    use std::{
        collections::HashMap,
        fs::write,
        io::{BufReader, Read},
    };

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TodoItem {
        id: u32,
        description: String,
        done: bool,
    }

    impl TodoItem {
        /// We will consider we pass false as value
        pub fn build(next_id: u32, description: String) -> TodoItem {
            TodoItem {
                id: next_id,
                description,
                done: false,
            }
        }

        pub fn is_done(&self) -> bool {
            self.done
        }

        /// Update a TodoItem
        pub fn update(&mut self) {
            self.done = !self.done;
        }

        /// Header off a TodoItem to a line of a csv
        pub fn header_of_csv() -> &'static str {
            "Id,Description,Done"
        }

        /// Convert a TodoItem to a line of a csv
        pub fn elem_in_csv(&mut self) -> String {
            format!("{},{},{}", self.id, self.description, self.done)
        }

        /// Clone
        pub fn clone(&self) -> TodoItem {
            TodoItem {
                id: self.id,
                description: String::from(&self.description),
                done: self.done,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TodoList {
        list: HashMap<String, TodoItem>,
        next_id: u32,
    }

    impl TodoList {
        /// Build a empty TodoList
        /// The id is always start with 0
        pub fn build() -> TodoList {
            TodoList {
                list: HashMap::new(),
                next_id: 0,
            }
        }

        /// Get todo item by description
        pub fn get_item_by_description(&self, todo_description: String) -> Option<TodoItem> {
            match self.list.get(&todo_description) {
                Some(value) => Some(value.clone()),
                None => None,
            }
        }

        /// Get todo item by id
        pub fn get_item_by_id(&self, todo_id: u32) -> Option<TodoItem> {
            for elem in self.list.values() {
                if elem.id == todo_id {
                    return Some(elem.clone());
                }
            }
            None
        }

        /// Update one todo item according the given description
        pub fn update_todo_item_description(&mut self, todo_description: String) -> Option<bool> {
            match self.list.get_mut(&todo_description.to_ascii_lowercase()) {
                Some(v) => {
                    v.update();
                    Some(v.is_done())
                }
                None => None,
            }
        }

        /// Update one todo item according the given id
        pub fn update_todo_item_id(&mut self, id: u32) -> Option<bool> {
            for elem in self.list.values_mut() {
                if elem.id == id {
                    elem.update();
                    return Some(elem.is_done());
                }
            }
            None
        }

        /// Insert a new item into our Todo_list.
        /// We will consider we pass false as value
        pub fn insert(&mut self, todo_description: String) -> bool {
            let todo_item = TodoItem::build(self.next_id, todo_description.to_ascii_lowercase());
            if self
                .list
                .get(&todo_description.to_ascii_lowercase())
                .is_none()
            {
                self.next_id += 1;
                self.list
                    .insert(todo_description.to_ascii_lowercase(), todo_item);
                return true;
            }
            false
        }

        /// Remove a item from our Todo_list by description
        pub fn remove_by_description(&mut self, todo_description: String) -> Option<TodoItem> {
            self.list.remove(&todo_description.to_ascii_lowercase())
        }

        /// Remove a item from our Todo_list by id
        pub fn remove_by_id(&mut self, id: u32) -> Option<TodoItem> {
            let mut index: Option<String> = None;
            for (description, elem) in self.list.iter_mut() {
                if elem.id == id {
                    index = Some(String::from(description))
                }
            }

            if index.is_some() {
                return self.list.remove(&index.unwrap());
            }
            None
        }

        /// Return all the struct in json  pretty
        pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
            serde_json::to_string_pretty(&self)
        }

        /// Return all the struct in json
        pub fn to_json(&self) -> Result<String, serde_json::Error> {
            serde_json::to_string(&self)
        }

        /// Read the default file, and return the all struct
        /// If the file don't exist we will create one
        /// In this case the file is JSON
        fn read_json(filename: &str) -> Result<TodoList, std::io::Error> {
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open(format!("{}.json", &filename))?;

            let buf_reader = BufReader::new(f);

            let todo_list: TodoList = match serde_json::from_reader(buf_reader) {
                Ok(todo_list) => todo_list,
                Err(err) => {
                    println!("\nError reading json file {}.json :\n {}", filename, err);
                    TodoList::build()
                }
            };

            Ok(todo_list)
        }

        /// Save all the struct in a json file
        pub fn save_json(&mut self, filename: &str) -> Result<(), std::io::Error> {
            let path = format!("{}.json", filename);
            let todo_list_json = serde_json::to_string_pretty(&self).unwrap();
            write(path, &todo_list_json)
        }

        /// Save all the struct in a csv file
        pub fn save_csv(&mut self, filename: &str) -> Result<(), std::io::Error> {
            let mut content = String::new();

            // TODO Add header to file
            // content.push_str(&format!("{}\n", TodoItem::header_of_csv()));
            for value in self.list.values_mut() {
                let record = format!("{}\n", value.elem_in_csv());
                content.push_str(&record);
            }
            std::fs::write(filename, content)
        }

        /// Read the default file, and return the all struct
        /// If the file don't exist we will create one
        /// In this case the file is CSV
        pub fn read_csv(filename: &str) -> Result<TodoList, std::io::Error> {
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .read(true)
                .open(format!("{}.csv", &filename))?;

            let mut id_max = 0;
            let mut content = String::new();

            f.read_to_string(&mut content)?;
            //TODO Remove first line (header) from file
            // content.remove(format!("{}\n", TodoItem::header_of_csv()));
            let map: HashMap<String, TodoItem> = content
                .lines()
                .map(|line| line.splitn(3, ',').collect::<Vec<&str>>())
                .map(|v| (v[0], v[1], v[2]))
                .map(|(id, description, done)| {
                    let number_id = id.trim().parse().unwrap();
                    if id_max < number_id {
                        id_max = number_id
                    }

                    (
                        String::from(description),
                        TodoItem {
                            id: number_id,
                            description: String::from(description),
                            done: String::from(done) == "true",
                        },
                    )
                })
                .collect();
            Ok(TodoList {
                list: map,
                next_id: id_max + 1,
            })
        }
    }

    /// Module responsible for executing actions and returning input to the user
    pub mod actions {
        use crate::todo_list::TodoList;
        use std::process::exit;

        ///Action responsible for adding an item
        pub fn add(todo: &mut TodoList, item: String) {
            let b = todo.insert(item);
            if b {
                println!("Todo item saved!")
            } else {
                println!("Todo item already exist!")
            }
        }

        ///Action responsible for removing an item according to an description
        pub fn remove(todo: &mut TodoList, item: String) {
            use std::num::ParseIntError;
            let number_id: Result<u32, ParseIntError> = String::from(&item).trim().parse();
            match number_id {
                Ok(id) => {
                    let result = todo.remove_by_id(id);
                    match result {
                        Some(value) => {
                            println!(
                                "Todo item deleted with success! -> {} : {}",
                                id, value.description
                            )
                        }
                        None => println!("There is no item with the given id: {} !", id),
                    }
                }
                Err(_) => {
                    let result = todo.remove_by_description(String::from(&item));
                    match result {
                        Some(value) => {
                            println!(
                                "Todo item deleted with success! -> {} : {}",
                                value.id, value.description
                            )
                        }
                        None => println!("There is no item with the given description: {} !", item),
                    }
                }
            }
        }

        ///Action responsible for update an item according to an id or a description
        pub fn update(todo: &mut TodoList, item: String) {
            use std::num::ParseIntError;
            let number_id: Result<u32, ParseIntError> = String::from(&item).trim().parse();
            match number_id {
                Ok(id) => {
                    let result = todo.update_todo_item_id(id);
                    match result {
                        Some(value) => {
                            println!("Todo item update with success! -> {} : {}", id, value)
                        }
                        None => println!("There is no item with the given id: {} !", id),
                    }
                }
                Err(_) => {
                    let result = todo.update_todo_item_description(String::from(&item));
                    match result {
                        Some(value) => {
                            println!("Todo item update with success! -> {} : {}", &item, value)
                        }
                        None => println!("There is no item with the given description: {} !", item),
                    }
                }
            }
        }

        ///Action responsible to save the TodoList to a file
        pub fn save(todo: &mut TodoList, filename: &str) {
            match todo.save_json(filename) {
                Ok(_) => {}
                Err(why) => println!("An error occurred: {}", why),
            }
        }

        ///Action responsible to read the TodoList to a file
        pub fn read(filename: &str) -> TodoList {
            TodoList::read_json(filename).expect("Initialisation of db failed")
        }

        ///Action responsible to given all the TodoList
        pub fn print_json_pretty(todo: &TodoList) {
            println!(
                "{}",
                &todo.to_json_pretty().unwrap_or("Nothing".to_string())
            )
        }

        ///Action responsible to given all the TodoList
        pub fn print_json(todo: &TodoList) {
            println!("{}", &todo.to_json().unwrap_or("Nothing".to_string()))
        }

        pub fn render_cli(filename: &str) {
            let argv = std::env::args().len();

            if argv < 2 {
                println!("Please specify an action");
                exit(0);
            }

            let action = std::env::args().nth(1).expect("Please specify an action");
            let mut item = "".to_string();
            // actions that only need 2 args
            let actions_only_2 = ["help", "show"];

            if !actions_only_2.contains(&action.as_str()) && argv < 3 {
                println!("Please specify an item");
                exit(0);
            } else if !actions_only_2.contains(&action.as_str()) {
                item = std::env::args().nth(2).expect("Please specify an item");
            }
            // println!("{:?}, {:?}", action, item);

            let mut todo = read(filename);
            let mut changes = true;

            if action == "add" {
                add(&mut todo, item);
            } else if action == "remove" {
                remove(&mut todo, String::from(&item));
            } else if action == "update" {
                update(&mut todo, String::from(&item));
            } else if action == "show" {
                print_json_pretty(&todo)
            } else {
                changes = false;
                println!("The given command: {} is invalid!", action);
            }

            if changes {
                save(&mut todo, filename)
            }
        }
    }
}

static FILENAME: &'static str = "todo_list";

fn main() {
    todo_list::actions::render_cli(FILENAME);
}
