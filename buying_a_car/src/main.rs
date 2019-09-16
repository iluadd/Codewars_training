fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    // initilize start parameters
    let mut old_car_price = old as f64;
    let mut new_car_price = new as f64;
    let mut current_savings = 0 as f64;
    let mut current_prec = perc;
    let mut months = 0 ;

    loop {
        // check buying opportunity
        if old_car_price + current_savings >= new_car_price {
            break;
        }

        old_car_price = old_car_price - current_prec * old_car_price / 100.0;
        new_car_price = new_car_price - current_prec * new_car_price / 100.0;

        // update savigs and perc
        current_savings += saving as f64;
        if months % 2 == 0 {
            current_prec += 0.5;
        }

        months += 1;

    }

    let leftover = (current_savings + old_car_price) - new_car_price;
    (months, leftover.round() as i32)
}       


fn main() {
    println!("{:?}", nb_months(2000, 8000, 1000, 1.5));
    println!("{:?}", nb_months(18000, 32000, 1500, 1.25));
    println!("{:?}", nb_months(7500, 32000, 300, 1.55));
    println!("{:?}", nb_months(12000, 8000, 1000, 1.5));
    println!("{:?}", nb_months(8000, 8000, 1000, 1.5));
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

