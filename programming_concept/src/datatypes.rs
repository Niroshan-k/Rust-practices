// pub public function
pub fn my_datatypes() { 
 println!("-------------data types.rs-------------");
 
 let guess: u32 = "42".parse().expect("Not a number!");
 // let guess = "42".parse().expect("Not a number!"); 
 // we must add a type annotation : u32
 println!("{guess}"); 

 // scalar
 // 1. integer
 //  ------------------------------------------
 // | lenght              |  signed | unsigned |
 //  ------------------------------------------
 // | 8-bit               |  i8     | u8       |
 // | 16-bit              |  i16    | u16      |
 // | 32-bit              |  i32    | u32      |
 // | 64-bit              | i64     | u64      |
 // | 128-bit             | i128    | u128     |
 // | Architecture-depent | isize   | usize    |
 //  ------------------------------------------

 // 2. floating points
 let x = 2.0; // f64
 let y: f32 = 3.0; // f32
 println!("f32-{y} | f64-{x}");

 // addition
 let sum = 5 + 10;
 println!("{sum}");
 // subtraction
 let difference = 95.5 - 4.3;
 println!("{difference}");
 // multiplication
 let product = 4 * 30;
 println!("{product}");
 // division
 let quotient = 56.7 / 32.2;
 let truncated = -5 / 3; // Results in -1
 println!("{quotient}-{truncated}");
 // remainder
 let remainder = 43 % 5;
 println!("{remainder}");

 // 3.bool
 let t = true; // default
 let f: bool = false;
 println!("{t}-{f}");

 // 4.characters
 let c = 'z';
 let z: char = 'ℤ'; // with explicit type annotation
 let heart_eyed_cat = '😻';
 println!("{c}-{z}-{heart_eyed_cat}");

 println!("---------------------------------------");
}