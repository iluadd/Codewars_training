fn bmi(weight: u32, height: f32) -> &'static str {
    let mut bmi = weight as f32 / height.powi(2);
    bmi = bmi.round();
    match bmi {
       bmi if bmi <= 18.5 => return "Underweight",
       bmi if bmi <= 25.0 => return "Normal",
       bmi if bmi <= 30.0 => return "Overweight",
       _ => return "Obese",
    };
}

fn main() {
    println!("{}",bmi(80, 1.80));
}


#[test]
fn basic_unit_test() {
  assert_eq!(bmi(60, 1.9), "Underweight");
  assert_eq!(bmi(80, 1.80), "Normal");
  assert_eq!(bmi(90, 1.80), "Overweight");
  assert_eq!(bmi(100, 1.80), "Obese");
}


// Write function bmi that calculates body mass index (bmi = weight / height ^ 2).

// if bmi <= 18.5 return "Underweight"

// if bmi <= 25.0 return "Normal"

// if bmi <= 30.0 return "Overweight"

// if bmi > 30 return "Obese