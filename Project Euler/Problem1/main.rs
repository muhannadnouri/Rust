fn main() {
  let mut multiples: Vec<i32> = Vec::new();

  for x in 0..1000 {
    if x % 3 == 0 || x % 5 == 0 {
      multiples.push(x);
    }
  }
  
  let sum: i32 = multiples.iter().sum();
  println!("Sum: {}", sum);
}
