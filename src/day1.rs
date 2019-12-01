use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn get_fuel_required(mass: u32) -> u32 {
    match (mass as f32 / 3.0).floor() as i32 - 2 {
        n if n > 0 => n as u32,
        _ => 0,
    }
}

#[aoc(day1, part1)]
pub fn solve_part_1(modules: &[u32]) -> u32 {
    modules
        .iter()
        .map(|module| get_fuel_required(*module))
        .sum::<u32>()
}

#[aoc(day1, part2)]
pub fn solve_part_2(modules: &[u32]) -> u32 {
    modules
        .iter()
        .map(|module| get_fuel_required(*module))
        .map(|fuel_required| {
            let additional_fuel_required = {
                let mut current = get_fuel_required(fuel_required);
                let mut sum = current;

                while current > 0 {
                    current = get_fuel_required(current);
                    sum += current;
                }

                sum
            };

            fuel_required + additional_fuel_required
        })
        .sum()
}
