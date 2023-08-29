// Sub Materi 1: Introduction to Pattern Matching
fn intro_to_pattern_matching() {
    let value = 42;

    match value {
        0 => println!("Value is zero"),
        1..=10 => println!("Value is between 1 and 10"),
        _ => println!("Value is something else"),
    }
}

// Sub Materi 2: Matching Literals and Ranges
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Stormy,
}

fn match_literals_and_ranges(weather: Weather) {
    match weather {
        Weather::Sunny => println!("It's a sunny day!"),
        Weather::Cloudy => println!("It's a cloudy day."),
        Weather::Rainy | Weather::Stormy => println!("It's raining or stormy."),
    }
}

// Sub Materi 3: Matching Structs and Enums
struct Person {
    name: String,
    age: u32,
}

enum Message {
    Greeting(String),
    Farewell,
}

fn match_structs_and_enums(person: Person, message: Message) {
    match person {
        Person { name, age } => {
            println!("Hello, my name is {} and I'm {} years old.", name, age);
        }
    }

    match message {
        Message::Greeting(msg) => {
            println!("Received a greeting: {}", msg);
        }
        Message::Farewell => {
            println!("Received a farewell message.");
        }
    }
}

// Sub Materi 4: Destructuring with Patterns
fn destructure_with_patterns() {
    let tuple = (1, "hello");

    match tuple {
        (1, text) => {
            println!("Found number 1 and text: {}", text);
        }
        _ => {
            println!("Didn't find the expected pattern.");
        }
    }
}

// Sub Materi 5: Combining Patterns
fn combining_patterns(value: i32) {
    match value {
        1 | 2 | 3 => println!("Value is 1, 2, or 3."),
        4..=10 => println!("Value is between 4 and 10."),
        _ => println!("Value is something else."),
    }
}

// Sub Materi 6: Pattern Matching in Control Flow
fn pattern_matching_in_control_flow() {
    let result: Result<i32, &str> = Ok(42);

    if let Ok(value) = result {
        println!("Operation succeeded with value: {}", value);
    } else {
        println!("Operation failed.");
    }
}

// Sub Materi 7: Advanced Pattern Matching
fn advanced_pattern_matching() {
    let value = 10;

    match value {
        x if x < 0 => println!("Value is negative"),
        x if x > 0 => println!("Value is positive"),
        _ => println!("Value is zero"),
    }
}

// Sub Materi 8: Real-World Examples
fn real_world_examples() {
    // Parsing URLs
    let url = "http://www.example.com/page";
    let url_parts: Vec<&str> = url.split('/').collect();

    match url_parts.as_slice() {
        ["http:", "", host, path] => {
            println!("Protocol: http");
            println!("Host: {}", host);
            println!("Path: {}", path);
        }
        _ => {
            println!("Invalid URL format");
        }
    }

    // Handling Errors
    let result = perform_complex_operation();

    match result {
        Ok(value) => {
            println!("Operation succeeded with value: {}", value);
        }
        Err(error) => {
            println!("Operation failed with error: {:?}", error);
        }
    }

    // Handling State Machine
    let light = TrafficLight::Green;

    match light {
        TrafficLight::Red => {
            println!("Stop!");
        }
        TrafficLight::Yellow => {
            println!("Prepare to stop");
        }
        TrafficLight::Green => {
            println!("Go!");
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn perform_complex_operation() -> Result<i32, String> {
    let success = true;

    if success {
        Ok(42)
    } else {
        Err(String::from("An error occurred"))
    }
}

fn main() {
    intro_to_pattern_matching();
    
    let weather = Weather::Rainy;
    match_literals_and_ranges(weather);
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let message = Message::Greeting(String::from("Hello there!"));
    match_structs_and_enums(person, message);
    
    destructure_with_patterns();
    
    combining_patterns(7);
    
    pattern_matching_in_control_flow();
    
    advanced_pattern_matching();
    
    real_world_examples();
}
