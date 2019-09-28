use std::collections::HashMap;

fn mean(town: &str, strng: &str) -> f64 {
    let city_data_vector = parse_weather_log(&town, strng);
    if city_data_vector[0] == -1.0{
        return -1.0
    }
    let vec_sum: f64 = Iterator::sum(city_data_vector.iter());
    vec_sum / city_data_vector.len() as f64
}

fn variance(town: &str, strng: &str) -> f64 {
    let city_data_vector = parse_weather_log(&town, strng);
    if city_data_vector[0] == -1.0{
        return -1.0
    }
    let meanz = mean(town, strng);
    let vec_sum: Vec<_> = city_data_vector.iter()
                                          .map(|&x| x-meanz)
                                          .collect();
    let vec_sum: f64 = vec_sum.iter().map(|&x| x*x).sum();
    (vec_sum / city_data_vector.len() as f64)
}


fn parse_weather_log(town: &str, data: &str) -> Vec<f64>{
    // maka a Hash for each town
    let mut towns = HashMap::new();

    for line in data.lines(){
        let each_line = line.replace(",", " " ).replace(":", " ");
        let town_vec: Vec<&str> = each_line.split(" ").collect();

        for elem in &town_vec {
            let _cur_num = match elem.parse::<f64>() {
                Ok(values) => {
                    if towns.contains_key(town_vec[0]){

                        let  smth  = towns.entry(town_vec[0].to_owned()).or_insert(vec![2f64]);
                        smth.push(values);
                    } 
                    else {
                        towns.entry(town_vec[0].to_owned()).or_insert(vec!(values));
                    }

                },
                Err(_) => {},
            };   
        };


    };
    match towns.get(town) {
        Some(_number) => towns[town].to_vec() ,
        _ => vec!(-1.0)
    }
}

fn main(){
    // let res = mean("London", &da_ta()[..]);
    // println!("London mean {}", res);

    // let res = mean("Beijing", &da_ta()[..]);
    // println!("Beijing mean {}", res);

    // let res = variance("London", &da_ta()[..]);
    // println!("London var {}", res);

    // let res = variance("Beijing22", &da_ta()[..]);
    // println!("Beijing22 var {}", res);

}


#[cfg(test)]
    mod tests {
    use super::*;

    fn da_ta() -> String {
    
// don't move the string below

let dr0 = r#"Rome:Jan 81.2,Feb 63.2,Mar 70.3,Apr 55.7,May 53.0,Jun 36.4,Jul 17.5,Aug 27.5,Sep 60.9,Oct 117.7,Nov 111.0,Dec 97.9
London:Jan 48.0,Feb 38.9,Mar 39.9,Apr 42.2,May 47.3,Jun 52.1,Jul 59.5,Aug 57.2,Sep 55.4,Oct 62.0,Nov 59.0,Dec 52.9
Paris:Jan 182.3,Feb 120.6,Mar 158.1,Apr 204.9,May 323.1,Jun 300.5,Jul 236.8,Aug 192.9,Sep 66.3,Oct 63.3,Nov 83.2,Dec 154.7
NY:Jan 108.7,Feb 101.8,Mar 131.9,Apr 93.5,May 98.8,Jun 93.6,Jul 102.2,Aug 131.8,Sep 92.0,Oct 82.3,Nov 107.8,Dec 94.2
Vancouver:Jan 145.7,Feb 121.4,Mar 102.3,Apr 69.2,May 55.8,Jun 47.1,Jul 31.3,Aug 37.0,Sep 59.6,Oct 116.3,Nov 154.6,Dec 171.5
Sydney:Jan 103.4,Feb 111.0,Mar 131.3,Apr 129.7,May 123.0,Jun 129.2,Jul 102.8,Aug 80.3,Sep 69.3,Oct 82.6,Nov 81.4,Dec 78.2
Bangkok:Jan 10.6,Feb 28.2,Mar 30.7,Apr 71.8,May 189.4,Jun 151.7,Jul 158.2,Aug 187.0,Sep 319.9,Oct 230.8,Nov 57.3,Dec 9.4
Tokyo:Jan 49.9,Feb 71.5,Mar 106.4,Apr 129.2,May 144.0,Jun 176.0,Jul 135.6,Aug 148.5,Sep 216.4,Oct 194.1,Nov 95.6,Dec 54.4
Beijing:Jan 3.9,Feb 4.7,Mar 8.2,Apr 18.4,May 33.0,Jun 78.1,Jul 224.3,Aug 170.0,Sep 58.4,Oct 18.0,Nov 9.3,Dec 2.7
Lima:Jan 1.2,Feb 0.9,Mar 0.7,Apr 0.4,May 0.6,Jun 1.8,Jul 4.4,Aug 3.1,Sep 3.3,Oct 1.7,Nov 0.5,Dec 0.7
"#;
        return String::from(dr0);
    }

    fn assert_fuzzy_mean(town: &str, strng: &str, expect: f64) -> () {
        let merr = 1e-2;
        println!(" town: {:?}", town);
        let actual = mean(town, strng);
        println!("Actual {:e}", actual);
        println!("Expect {:e}", expect);
        let inrange = (actual - expect).abs() <= merr;
        if inrange == false {
            println!("Expected value near: {:e} but got: {:e}", expect, actual);
        }
        println!("{}", inrange == true);
        assert_eq!(inrange, true);
    }
    fn assert_fuzzy_var(town: &str, strng: &str, expect: f64) -> () {
        let merr = 1e-2;
        println!(" town: {:?}", town);
        let actual = variance(town, strng);
        println!("Actual {:e}", actual);
        println!("Expect {:e}", expect);
        let inrange = (actual - expect).abs() <= merr;
        if inrange == false {
            println!("Expected value near: {:e} but got: {:e}", expect, actual);
        }
        println!("{}", inrange == true);
        assert_eq!(inrange, true);
    }

    #[test]
    fn basic_tests() {
        let da_ta = &da_ta();
        assert_fuzzy_mean("London", da_ta, 51.199999999999996);
        assert_fuzzy_mean("Beijing", da_ta, 52.416666666666664);
        assert_fuzzy_var("London", da_ta, 57.42833333333374);
        assert_fuzzy_var("Beijing", da_ta, 4808.37138888889);
        assert_fuzzy_var("Beijing22", da_ta, -1.0);
        
    }
}
