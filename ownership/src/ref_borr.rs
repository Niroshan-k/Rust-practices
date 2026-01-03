// references and borrowing

pub fn ref_borrow() {
 let s1 = String::from("hello");
 let len = calculate_length(&s1);
 println!("The length of '{s1} is {len}.");
 
 let mut s = String::from("hello");
 change(&mut s); //mutable reference

 // If you have a mutable reference to a value, 
 //you can have no other references to that value.
 let r1 = &mut s;
 //let r2 = &mut s; //give error // rust prevent data races
 println!("{r1}");

 //but we can use {}
 {
  let r2 = &mut s;
  println!("{r2}");
 }

 let s1 = &s;
 let s2 = &s;
 // combine mutable and immutable
 //let r3 = &mut s; // we cannot have mutable ref while we have an immutable

 println!("{s1}{s2}");

 // ref scope start from where it is introduced
 let q1 = &s;
 let q2 = &s;
 println!("{q1} and {q2}"); // Variables q1 and q2 will not be used after this point.
 let q3 = &mut s; //no errors
 println!("{q3}");

 //let reference_to_nothing = dangle();
 let string = no_dangle();

 println!("{string}");

}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
//     // give error because we cant change the borrow value
// }

//mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    
}

//dangling pointer—a pointer that references a location in memory that 
//may have been given to someone else—by 
//freeing some memory while preserving a pointer to that memory.

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped, so its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s //Ownership is moved out, and nothing is deallocated.
}