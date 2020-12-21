use std::io;
use helpers::calculate_weight_on_mars;
mod helpers;

fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}
