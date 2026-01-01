pub fn my_function() {

 println!("-------------function.rs-------------");

 let y = {
  let x = 3;
  x + 1 // an expression return value.. x + 1; is a statement
 };
 println!("{y}");

 let five = five();
 println!("{five}");

 let pluvalue = plus_one(4);
 println!("{pluvalue}");

 let condition = false;
 let number = if condition {5} else {6}; // if is an expression
 let number = if condition {5} else {"six"}; // compiler error
 println!("{number}");

 println!("-------------------------------------");
}

fn five() -> i32 {
 5
}

fn plus_one(x: i32) -> i32 {
 x + 1 // not x + 1; because its returning a value
}