use std::iter;

fn print(n: i32) -> Option<String> {
    if n < 1 {
        None
    } else {
        // let mut temp = String::new();
        let mut sum = String::new();
        let fwd_numbers: Vec<i32> = (1..(n as i32)+1).collect();
        let bkw_numbers: Vec<i32> = (1..(n as i32)).rev().collect();
        let mut numbers = [&fwd_numbers[..], &bkw_numbers[..]].concat();
        println!("{:?} - {:?}",fwd_numbers, bkw_numbers);
        println!("{:?}",numbers);

        numbers.retain(|&i|i % 2 != 0);

        println!("{:?}",numbers);

        for i in n+1..(n+1)/2 {
            let temp = iter::repeat("*").take(i as usize)
                                        .collect::<String>() + "\n";
            println!("{:?}",temp);                    
            sum += &temp;
        }

        // let sum: String = sum.to_string();
        Some(sum)
    }
}

fn main() {
// println!("Hello, world!");
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

