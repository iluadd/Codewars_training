fn main() {
    println!("Hello, world!");
}


#[test]
fn basic_unit_test() {
  assert_eq!(bmi(80, 1.80), "Normal");
}


// Write function bmi that calculates body mass index (bmi = weight / height ^ 2).

// if bmi <= 18.5 return "Underweight"

// if bmi <= 25.0 return "Normal"

// if bmi <= 30.0 return "Overweight"

// if bmi > 30 return "Obese