
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {

  use std::io::{stdin,stdout,Write};
  let mut s=String::new();
  print!("Please enter some text: ");
  let _=stdout().flush();
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }
  println!("You typed: {}",s);


  // The return value of the function is an option
  let result = divide(2.0, 0.0);

  // Pattern match to retrieve the value
  match result {
      // The division was valid
      Some(x) => println!("Result: {}", x),
      // The division was invalid
      None    => println!("Cannot divide by 0"),
  }

}
