use std::io;

fn all(a:u32 ,b: u32){
    println!("{} + {} = {}\n", a,b,a+b );
    println!("{} - {} = {}\n",
a,b,a - b);
    println!("{} * {} = {}\n", a,b,a*b);
    println!("{} / {} = {}\n",a,b,a/b);
}


fn main(){
  println!("Enter the bigger no. : ");
  
  let mut a = String::new();
  io::stdin().read_line(&mut a).expect("Failed to read line!");
 let a : u32 =
  a.trim().parse().expect("please enter the number !");

  println!("Enter the smaller no. : ");
let mut b = String::new();
io::stdin().read_line(&mut b).expect("Failed to read line!");
let b : u32 = 
b.trim().parse().expect("please enter a number : ");
  
  println!("Here the results are
  :- ");
  all(a,b);
}