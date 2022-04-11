//! Module responsible for executing actions and returning input to the user
use std::process::exit;
use todo_list::TodoList;

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
                        id,
                        value.description()
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
                        value.id(),
                        value.description()
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
