use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    File::open("data/day01-input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let total_fuel = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map(|x| x / 3 - 2)
        .sum::<i32>();

    println!("Total Fuel (Part 1): {}", total_fuel);

    let total_fuel = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map(calc_fuel)
        .sum::<i32>();

    println!("Total Fuel (Part 2): {}", total_fuel);
}

fn calc_fuel(mass: i32) -> i32 {
    let fuel_for_mass = mass / 3 - 2;
    if fuel_for_mass > 0 {
        fuel_for_mass + calc_fuel(fuel_for_mass)
    } else {
        0
    }
}
