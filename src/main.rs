// use math::round;

fn main() {
    let min_point: f64 = 26.0;
    let max_point: f64 = 92.0;
    let min_max_diff: f64 = set_min_max(min_point,max_point);
    let max_tics: f64 = 10.0;

    let range: f64 = nice_numb(min_max_diff, false);
    let tick_spacing: f64 = nice_numb( range / (max_tics -1.0) , true);
    let nice_min: f64 = (min_point / tick_spacing).floor() * tick_spacing;
    let nice_max: f64 = (max_point / tick_spacing).ceil() * tick_spacing;
    println!("The value of range is: {}", range);
    println!("The value of tick_spacing is: {}", tick_spacing);
    println!("Nice min is: {}", nice_min);
    println!("Nice max is: {}", nice_max);
}

fn nice_numb(min_max: f64, round: bool) -> f64 {
    let log_of_min_max = min_max.log10();
    let exponent = log_of_min_max.floor();
    let fraction = min_max / 10.0_f64.powf(exponent);
    let nice_fraction;

    if round {
        if fraction < 1.5 {
            nice_fraction = 1.0;
        } else if fraction < 3.0 {
            nice_fraction = 2.0;
        } else if fraction < 7.0 {
            nice_fraction = 5.0;
        } else {
            nice_fraction = 10.0;
        }
    } else {
        if fraction <= 1.0 {
            nice_fraction = 1.0;
        } else if fraction <= 2.0 {
            nice_fraction = 2.0;
        } else if fraction <= 5.0 {
            nice_fraction = 5.0;
        } else {
            nice_fraction = 10.0;
        }
    }
    nice_fraction * 10.0_f64.powf(exponent)
}

fn set_min_max(min: f64, max: f64) -> f64 {
    let res = max - min;
    println!("min_max_diff= {}",res);
    res
}
