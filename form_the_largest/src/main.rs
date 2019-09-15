fn max_number(n: u32) -> u32 {
  let mut collected_numbers = Vec::new();
  let mut result = String::new();
  fetch_each_number(n, &mut collected_numbers);
  collected_numbers.sort();
  collected_numbers.reverse();
  for el in collected_numbers{
    result.push_str( &el.to_string() );
  }
  result.parse::<u32>().unwrap()
}

fn fetch_each_number(n: u32, mut collected: &mut Vec<u32>) -> &mut Vec<u32>{
  collected.push(n%10);
  if n>9{
    fetch_each_number(n/10, &mut collected); 
  } 
  collected
}

fn main() {
    println!("{}", max_number(63729));
}

#[test]
fn basic_tests() {
  assert_eq!(max_number(213), 321);
  assert_eq!(max_number(7389), 9873);
  assert_eq!(max_number(63729), 97632);
  assert_eq!(max_number(566797), 977665);
  assert_eq!(max_number(17693284), 98764321);
}