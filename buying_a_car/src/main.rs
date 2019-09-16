fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    // your code
    (1,2)
}       



fn main() {
    println!("{:?}", nb_months(2000, 8000, 1000, 1.5));
}





fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
}

#[test]
fn basics_nb_months() {
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(12000, 8000, 1000, 1.5 , (0, 4000));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
}