use std::fmt::Debug;

fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> 
    where T: Debug + Copy
{
    let mut diff_vec: Vec<T> = Vec::new();
    for el_a in &a {
        let is_included = b.iter().any(|&x| x==*el_a);
        if !is_included {
            diff_vec.push(*el_a);
        }
    }
    diff_vec
}

// non Generic way
// fn array_diff2 (a : Vec<i32>, b: Vec<i32>) -> Vec<i32> {
//     let mut diff_vec : Vec<i32> =  Vec::new();
//     for el_a in &a {
//         for el_b in &b {
//             println!("{} - {}",el_a, el_b );
//             diff_vec.push(*el_a);
//         }
//     }
//     diff_vec
// }

fn main() {
    println!("{:?}", array_diff(vec![1,2,3,4], vec![1,4,5]));

}



// Add your tests here
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
        assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
        assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
    }
}