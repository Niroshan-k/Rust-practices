// Rusts ownership enables Rust to make 
// memory safety garantees without needing a grabage collector

pub fn ownership() {
    println!("Hello, world!");

    variable_scope();
    string_type(); 
    var_and_data_interacting_with_move();
    scope_assignment();
    var_and_data_interactwith_clone();

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("{s}"); give error
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{s1}{s3}");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn variable_scope() {
    {                      // s is not valid here, since it's not yet declared
        let s = "hello world";   // s is valid from this point forward
        println!("{s}");
        // do stuff with s
    }                     // this scope is now over, and s is no longer valid
    // rust call a function call drop
}

fn string_type() {
    let mut s = String::from("hello");
    s.push_str(", world!"); //push_str() appends a literal to a String
    println!("{s}");
}

fn var_and_data_interacting_with_move() {
    let x = 5;
    let y = x; //both values are 5. fixed size
    println!("{y}");

    // this is different even though it seems same
    let s1 = String::from("hello"); // ask permission from memory
    let s2 = s1; // so we copying the pointer,length,capacity
    // string contain 3parts -> 
    //ptr - {the location to the memory} , len - {how much memory in bytes} , 
    //capacity - {total amount of memory in bytes} ... more in rust pdf

    // in outof scope rust will clean the data: we lose the both values {double free error}
    // then rust considering s1 is gone
    //println!("{s1}"); // this give a error
    println!("{s2}"); // move
}

fn scope_assignment() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // rust will call drop and free the original "hello"
    println!("{s}, world!");
}

fn var_and_data_interactwith_clone() {
    // if we do want to copy the heap data of the string not just the stack
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

// rust let us return multiple values using a tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}