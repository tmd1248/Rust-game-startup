//Imports to make the code easier to read down below
// hash maps are the data structure being used, and io is input/output 
use std::collections::HashMap;
use std::io;
// a custom data type known as value encapsulates integers, strings, and booleans so they can all be placed in the same hashmap
#[derive(Debug)]
enum Value {
    Str(&'static str),
    Int(i32),
    Bool(bool),
}


fn main() {
// create an empty hash map for both characters 
    let mut char1 = HashMap::new();
    let mut char2 = HashMap::new();
// on the first turn, each player will choose a class from the three provided with different strengths and weaknesses 
    println!("Player 1 choose your class: Fighter, Thief, Knight");
// setting up declarations for the loop 
    let mut has_chosen= true;
    let mut input_string = String::new();
    let error_message = "We didn't recognize that. the classes are Fighter, Thief, and Knight";
// infinite loop is here used for decision making by the user 
    loop {
    // make sure that the input string is empty instead of appending each time 
        input_string.clear();
    // take input from the user, ensure it actually takes the input, and place it into the input_string variable 
        io::stdin().read_line(&mut input_string).unwrap();
    // ensure that the has_chosen variable is assigned to true before each loop
        has_chosen = true;
    // make sure that whitespace is removed from the input_string. without trim it would never match any condition
         match input_string.trim() {
    // assign each class to player 1's character if they picked it 
            "Fighter" => {
                char1.insert("class", &&Value::Str("Fighter"));
                char1.insert("health", &&Value::Int(10));
                char1.insert("stamina", &&Value::Int(12));
                char1.insert("poisoned", &&Value::Bool(false));
                char1.insert("incapacitated",&&Value::Bool(false));
            }, 
            "Thief" => {
                char1.insert("class", &&Value::Str("thief"));
                char1.insert("health", &&Value::Int(8));
                char1.insert("stamina", &&Value::Int(12));
                char1.insert("poisoned", &&Value::Bool(false));
               char1.insert("incapacitated",&&Value::Bool(false));
            },
            "Knight" => {
                char1.insert("class", &&Value::Str("Knight"));
                char1.insert("health", &&Value::Int(12));
                char1.insert("stamina", &&Value::Int(8));
                char1.insert("poisoned", &&Value::Bool(false));
                char1.insert("incapacitated",&&Value::Bool(false));
            },
        // in the default case, set has_chosen to false as the system didn't populate the hashmap 
            _ => has_chosen = false
        };
        // if the checker reads true, move to the next loop 
        if has_chosen == true {
            break;
        }
        // otherwise, tell the user the issue and go again
        println!("{}", error_message);
    }
    // same as last loop 
    println!("Player 2 Choose your class: Fighter, Thief, Knight");
    loop{
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        has_chosen = true;
        input_string = input_string.to_string();
         match input_string.trim() {
            "Fighter" => {
                char2.insert("class", &&Value::Str("Fighter"));
                char2.insert("health", &&Value::Int(10));
                char2.insert("stamina", &&Value::Int(12));
                char2.insert("poisoned", &&Value::Bool(false));
                char2.insert("incapacitated",&&Value::Bool(false));
            }, 
            "Thief" => {
                char2.insert("class", &&Value::Str("thief"));
                char2.insert("health", &&Value::Int(8));
                char2.insert("stamina", &&Value::Int(12));
                char2.insert("poisoned", &&Value::Bool(false));
                char2.insert("incapacitated",&&Value::Bool(false));
            },
            "Knight" => {
               char2.insert("class", &&Value::Str("Knight"));
                char2.insert("health", &&Value::Int(12));
                char2.insert("stamina", &&Value::Int(8));
                char2.insert("poisoned", &&Value::Bool(false));
                char2.insert("incapacitated",&&Value::Bool(false));
            },
            _ => has_chosen = false
        };
        if has_chosen == true {
            break;
        };
        println!("{}", error_message);
    }
    // for demonstration purposes only. in a full game, the statistics would be manipulated against each other instead of read out. 
    for (key, value) in &char1 {
        println!("{}: {:?}", key, value);
    }
    for (key, value) in &char2 {
        println!("{}: {:?}", key, value);
    }
}

