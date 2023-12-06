use shared::Day;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod shared;
mod utils;

fn print_answer(day: usize, part: u32, result: &str) {
    println!("Answer to Day {}, part {} is ... {}", day, part, result);
}

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let solutions: Vec<Box<dyn Day>> = vec![
        Box::new(day_01::Solution {}),
        Box::new(day_02::Solution {}),
        Box::new(day_03::Solution {}),
        Box::new(day_04::Solution {}),
        Box::new(day_05::Solution {}),
    ];

    for (i, solution) in solutions.iter().enumerate() {
        print_answer(i + 1, 1, &solution.part_1_with_input().to_string());
        print_answer(i + 1, 2, &solution.part_2_with_input().to_string());
    }

    Ok(())
}
