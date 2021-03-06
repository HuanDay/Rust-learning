/// Patterns are a special syntax in Rust for matching 
/// against the structure of types, both complex and simple. 
/// Using patterns in conjunction with match expressions 
/// and other constructs gives you more control over a 
/// program’s control flow. A pattern consists of 
/// some combination of the following:

/// *Literals
/// *Destructured arrays, enums, structs, or tuples
/// *Variables
/// *Wildcards
/// *Placeholders

/// These components describe the shape of the data we’re 
/// working with, which we then match against values to 
/// determine whether our program has the correct data 
/// to continue running a particular piece of code. To use a pattern, 
/// we compare it to some value. If the pattern matches the 
/// value, we use the value parts in our code.

/// 1. match arms
/// One requirement for match expressions is that they need 
/// to be exhaustive in the sense that all possibilities for 
/// the value in the match expression must be accounted for

// match VAULE {
//     PATTERN => expression,
//     PATTERN => expression,
//     PATTERN => expression,
//     _ => expression
// }

// 2. conditional `if let` expression
fn if_let_cases() {
    let favorite_color: Option<&str> = Some("yellow");
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as bg color", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the bg color");
    }
}

// 3. `while let` conditional loops
/// match a tuple in a function’s arguments to the pattern.
fn while_let() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// 4. for loops
/// In a for loop, the pattern is the value that 
/// directly follows the keyword for, so in for x 
/// in y the x is the pattern.
fn for_loops_pattern() {
    let v = vec![1, 2, 3];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// 5. let statments
/// `let PATTERN = EXPRESSION;`

// 6. function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location ({},{})", x, y);
}

// 7. Refutability
/// Patterns come in two forms: refutable and irrefutable. 
/// Patterns that will match for any possible value passed are irrefutable.
/// Function parameters, let statements, and for loops can only accept 
/// irrefutable patterns, because the program cannot do anything meaningful 
/// when values don’t match. The if let and while let expressions only accept 
/// refutable patterns, because by definition they’re intended to handle 
/// possible failure: the functionality of a conditional is in its ability 
/// to perform differently depending on success or failure.

// 8. All matching cases
extern crate rand;
use rand::{thread_rng, Rng};

struct Point {
    x: i32,
    y: i32
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Message {
    Quit,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(Color)
}

fn match_cases() {
    // 1. matching literals
    let mut rng = thread_rng();
    let x = rng.gen_range(0, 10);
    
    match x {
        1 => println!("one"),
        // Ranges are only allowed with numeric values or char values, 
        2...5 => println!("two to five"),
        6 | 7 => println!("six to seven"),
        _ => println!("large than 5")
    }

    let y = 'c';
    match y {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 2. use patterns to destructure structs,
    // enums, tuples, and references to use different parts of these values
    // We can also destructure with literal values as part of the 
    // struct pattern rather than creating variables for all the fields. 
    // Doing so allows us to test some of the fields for particular values 
    // while creating variables to destructure the other fields.
    let p = Point { x: 4, y: 5 };
    match p {
        Point { x: 4, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on either axis ({} {})", x, y)
    }
    println!("\n");

    // 3. destructure enums
    let msg_rgb_color = Message::ChangeColor(Color::Rgb(233, 33, 255));
    let msg_hsv_color = Message::ChangeColor(Color::Hsv(10, 33, 255));
    let msg_quit = Message::Quit;
    let msg_move = Message::Move { x: 4, y: 7 };
    let msg_write = Message::Write(String::from("nothing"));

    let message_content = |msg| {
        match msg {
            Message::Quit => println!("Quit variant has not data to use"),
            Message::Move { x, y } => println!("Move to ({} {})", x, y),
            Message::Write(text) => println!("Write {}", text),
            // destucting nested enums
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!("RGB Color red {}, green {}, blue {}", r, g, b),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!("HSV Color red {}, green {}, blue {}", h, s, v)
        }
    };

    message_content(msg_hsv_color);
    message_content(msg_rgb_color);
    message_content(msg_write);
    message_content(msg_move);
    message_content(msg_quit);

    // 4. destucting reference
    let points = vec! [
        Point { x: 0, y: 0},
        Point { x: 10, y: 10},
        Point { x: 8, y: 23}
    ];

    let sum_of_squares = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum::<i32>();
    println!("sum of squares: {}", sum_of_squares);

    // 5. destructuring structs and tuples
    // Destructuring with patterns is a convenient way to use pieces of values
    let ((feet, inches), Point { x, y }) = ((2, 12), Point { x: 4, y: 5 });
    println!("feet: {}, inches: {}, Point: ({}, {})", feet, inches, x, y);

    // 6. ignoring values in a pattern
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // use underscores in multiple places within one pattern to ignore particular values
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // ignoring an Unused Variable by Starting Its Name with `_`
    let _x = 5;
    let s = Some(String::from("Next"));
    if let Some(_) = s {
        println!("found a string");
    }

    // Ignoring Remaining Parts of a Value with `..`
    // The .. pattern ignores any parts of a value that we 
    // haven’t explicitly matched in the rest of the pattern.
    let point = Point { x: 32, y: 44};
    match point {
        Point { x, .. } => println!("x is {}", x)
        // Point { x, .., z } => println!("x is {}", x)
    }
}

// 7. Extra Conditionals with Match Guards
/// A match guard is an additional if condition 
/// specified after the pattern in a match arm 
/// that must also match, along with the pattern 
/// matching, for that arm to be chosen. Match 
/// guards are useful for expressing more complex 
/// ideas than a pattern alone allows.
fn match_guard() {
    let x = Some(5);
    let y = 5;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("matched, n = {:?}", n),
        _ => println!("Default case")
    }
}

// 8. binding
/// The at operator (@) lets us create a variable that holds a value at 
/// the same time we’re testing that value to see whether it matches a pattern. 
enum Msg {
    Hello { id: i32 }
}

fn binding_case() {
    let msg = Msg::Hello { id: 5 };
    match msg {
        Msg::Hello { id: id_var @ 3...8 } => {
            println!("Found an id in range: {}", id_var);
        },
        Msg::Hello { id: 10...13 } => {
            println!("Nothing");
        },
        Msg::Hello { id } => {
            println!("id: {}", id);
        }
    }
}

// We’ve used the underscore (_) as a wildcard pattern 
// that will match any  value but not bind to the value
fn foo(_: i32, y: i32) {
    // gnoring a function parameter can be especially useful 
    // in some cases, for example, when implementing a trait 
    // when you need a certain type signature but the function
    // body in your implementation doesn’t need one of the parameters. 
    // The compiler will then not warn about unused function parameters, 
    // as it would if you used a name instead.
    println!("this code only uses the y parameter: {}", y);
}

fn main() {
    if_let_cases();
    while_let();
    for_loops_pattern();

    let point = (3, 5);
    print_coordinates(&point);
    match_cases();
    foo(3, 4);
    match_guard();
    binding_case();
}
