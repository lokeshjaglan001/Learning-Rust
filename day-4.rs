fn main(){
  println!("Hello world!");
  first();
  second();
  third();
}
// Types of Loop

// 1) Simple loop which always continues
fn first(){
  loop{
    println!("Jaat");
  }
}

// 2) For loop which always go to the range which we give to it
fn second(){
  for _x_ in 0..78{
    if _x_ == 10{
      break;
    }
    println!("Humanshu");
  }
}
// 3) While loop which always run until the while condition is true
fn third(){
  let mut i = 0;
  i += 1;
  while i < 9{
    println!("Hello!");
    i += 1;
  }
}