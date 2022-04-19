use serde::{Deserialize, Serialize};

use std::{
    collections::{hash_map::Entry, HashMap},
    fs::write,
    io::{BufReader, ErrorKind, Read},
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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
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
    pub fn get_item_by_description(&self, todo_description: String) -> Option<&TodoItem> {
        match self.list.get(&todo_description) {
            Some(value) => Some(&value),
            None => None,
        }
    }

    /// Get todo item by id
    pub fn get_item_by_id(&self, todo_id: u32) -> Option<&TodoItem> {
        for elem in self.list.values() {
            if elem.id == todo_id {
                return Some(&elem);
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
        match self.list.entry(todo_description.to_ascii_lowercase()) {
            Entry::Vacant(elem) => {
                let todo_item =
                    TodoItem::build(self.next_id, todo_description.to_ascii_lowercase());
                elem.insert(todo_item);
                self.next_id += 1;
                true
            }
            Entry::Occupied(_) => false,
        }
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
                index = Some(String::from(description));
                break;
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
    pub fn read_json(filename: &str) -> Result<TodoList, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(format!("{}.json", &filename));

        if f.is_err() {
            return Err(f.err().unwrap());
        }

        let f = f.unwrap();

        let buf_reader = BufReader::new(f);

        let result = serde_json::from_reader(buf_reader);

        if result.is_err() {
            let phrase = format!(
                "Error reading / opening file ::: {}",
                result.err().unwrap().to_string()
            );
            let error = std::io::Error::new(ErrorKind::Other, phrase);
            Err(error)
        } else {
            Ok(result.unwrap())
        }
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

        content.push_str(&format!("{}\n", TodoItem::header_of_csv()));
        for value in self.list.values_mut() {
            let record = format!("{}\n", value.elem_in_csv());
            content.push_str(&record);
        }
        std::fs::write(format!("{}.csv", filename), content)
    }

    /// Read the default file, and return the all struct
    /// If the file don't exist we will create one
    /// In this case the file is CSV
    pub fn read_csv(filename: &str) -> Result<TodoList, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(format!("{}.csv", filename))?;

        let mut id_max = 0;
        let mut content = String::new();

        f.read_to_string(&mut content)?;
        let map: HashMap<String, TodoItem> = content
            .lines()
            .skip(1)
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
