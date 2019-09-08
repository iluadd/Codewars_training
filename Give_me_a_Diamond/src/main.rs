use std::iter;

fn print(n: i32) -> Option<String> {
    if n < 1 || n%2 == 0 {
        None
    } else {
        let mut sum = String::new();
        let fwd_numbers: Vec<i32> = (1..(n as i32)+1).collect();
        let bkw_numbers: Vec<i32> = (1..(n as i32)).rev().collect();
        let mut numbers = [&fwd_numbers[..], &bkw_numbers[..]].concat();

        numbers.retain(|&i|i % 2 != 0);

        for i in 0..n {
          sum += &" ".repeat(((n - numbers[i as usize])/2) as usize);
          sum += &iter::repeat("*").take(numbers[i as usize] as usize).collect::<String>();
          sum += "\n"

        }

        Some(sum)
    }
}

fn main() {
  println!("{:?}", print(5));
  println!("{:?}", print(3));
}

#[test]
fn basic_test() {
  assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
  assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
  assert_eq!(print(-3),None);
  assert_eq!(print(2),None);
  assert_eq!(print(0),None);
  assert_eq!(print(1), Some("*\n".to_string()) );
}

// 0 0 * 0 0 
// 0 * * * 0
// * * * * * 
// 0 * * * 0
// 0 0 * 0 0
