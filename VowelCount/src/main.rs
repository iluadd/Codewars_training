fn get_count(string: &str) -> usize {
  let vowels_count = string.chars()
                        .filter(|&x| "aeiou".contains(x))
                        .count();
  vowels_count              
}

fn main() {
    println!("{}", get_count("abracadabra"));
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}


// fn main() {
//     println!("{}", "abcd".chars().all(|x| x.is_alphanumeric()));
// }
// println!("{}", (0..1u32<<30).filter(|&i| i.count_ones() == 15).count());

// Return the number (count) of vowels in the given string.

// We will consider a, e, i, o, and u as vowels for this Kata.

// The input string will only consist of lower case letters and/or spaces.
