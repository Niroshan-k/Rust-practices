const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // valid for the entire program
// always const are immutable

pub fn my_variable() {

    println!("-------------From variable.rs-------------");
    let x = 5;
    println!("{x}");
    // x = 6; variables are immutable by default
    let mut y = 5; //mutable variable
    println!("{y}");
    y = 4;
    println!("{y}");

    println!("const :{THREE_HOURS_IN_SECONDS}");

    //shadowing
    // without using "mut"
    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("inner scope: {a}");
    }
    println!("{a}");

    let spaces = "  "; // string
    println!("{spaces}");
    let spaces = spaces.len(); // int
    println!("{spaces}");
    
    //but if we use mut it will give error
    //let mut spaces_ = "  "
    //spaces = spaces.len();
    println!("----------------------------------------");
}