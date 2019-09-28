fn angle(n: u32) -> u32 {
  // code hrer
  180*(n-2)
}

fn main() {
        println!("{}",angle(4) );
}



#[test]
fn normal_tests() {
    assert_eq!(angle(3), 180);
    assert_eq!(angle(4), 360);
    assert_eq!(angle(5), 540);
}