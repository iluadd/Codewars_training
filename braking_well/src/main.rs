fn dist(v: f64, mu: f64) -> f64 {   
    let reaction_delay = 1.0;
    let g = 9.81;
    let v_ci = (v * 5.0) / 18.0;
    let reaction_dist = reaction_delay * v_ci;
    let braking_distance = v_ci*v_ci/(2.0*mu*g);
    reaction_dist + braking_distance
}

fn speed(d: f64, mu: f64) -> f64 {    
    let g = 9.81;
    let discr: f64 = (2.0*mu*g).powf(2.0) + 4.0*d*2.0*mu*g;
    let x1: f64= (- 2.0*mu*g + discr.sqrt())/ (2.0);
    let _x2: f64 = (- 2.0*mu*g - discr.sqrt())/ (2.0);
    (x1*18.0)/5.0

}

fn main() {
    println!("{}",dist(144.0, 0.3));
    println!("{}",speed(311.83146449201496, 0.3));
    println!("{}",speed(83.9598760937531, 0.7));
    println!("{}",speed(92.0, 0.5));
}




fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-12;
    let inrange =
        if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
    if inrange == false {
        println!("Expected value must be near: {:e} but was:{:e}", expected, actual);
    } else {
        //println!("....... GOOD\n");
    }
    assert_eq!(true, inrange);
}

fn dotest1(v:f64, mu:f64, exp: f64) -> () {
    assert_fuzzy_equals(dist(v, mu), exp);
}
fn dotest2(d:f64, mu:f64, exp: f64) -> () {
    assert_fuzzy_equals(speed(d, mu), exp);
}

#[test]
fn basic_tests_dist(){
    dotest1(144.0, 0.3, 311.83146449201496);
    dotest1(92.0, 0.5, 92.12909477605366);
}
#[test]
fn basic_tests_speed(){
    dotest2(159.0, 0.8, 153.79671564846308);
    dotest2(164.0, 0.7, 147.91115701756493);
}