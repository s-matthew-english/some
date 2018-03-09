
fn main() {
  let numbers = vec![1, 2, 3, 4, 5, 6];

  let mut iter = numbers.iter();

  //assert_eq!(Some(&1), iter.next());
  println!("{:?}", iter.next().unwrap());
  println!("{:?}", Some(&1).unwrap());

  //assert_eq!(Some(&6), iter.next_back());
  println!("{:?}", Some(&6).unwrap());
  println!("{:?}", iter.next_back().unwrap());

  assert_eq!(Some(&5), iter.next_back());
  assert_eq!(Some(&2), iter.next());
  assert_eq!(Some(&3), iter.next());
  assert_eq!(Some(&4), iter.next());
  assert_eq!(None, iter.next());
  assert_eq!(None, iter.next_back());
}
