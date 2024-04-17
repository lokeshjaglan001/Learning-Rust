use std::io;
use std::string::String;
fn main(){
  // here is the program in which user input the meters and the program will return the kilometres
  println!("Enter Meters here :");
  let mut input = String::new();
  io::stdin().read_line(& mut input).expect("Failed to read line!");
  let meters : u32 = input.trim().parse().expect("Please enter the number");
  fn first(m: u32){
  let km = m as f32 / 1000.0; 
  println!("{} meters are equivalent to {} KM" , m,km)
  }
  
  first(meters);

  // here is the program in which user will input the minutes an then the program will return the hours and the remaining minutes
  println!("Enter the minutes here : ");
  let mut inp = String::new();
  io::stdin().read_line(& mut inp).expect("Failed to read the number");
  let minutes : u32 = inp.trim().parse().expect("Please enter the number !");
  fn second(n : u32){
  println!("{} minutes are equivalent to {} hours and {} minutes", n, n/60 , n%60);
  }
  
  second(minutes);
}