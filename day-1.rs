use std::io;
use std::string::String;
fn main(){
// how to print in rust
  println!("Hello world !");
// loop in rust
  for i in 0..50{
    println!("hii humanshu !");}
  println!("Please enter the number : ");
// how to take input from users
  let mut num = String::new();
  io::stdin().read_line(&mut num).expect("failed to read the number!");

  let num: u32 = 
  num.trim().parse().expect("Please enter a number!");
    
  println!("you enter {} ", num);
}
