mod day1;
mod day2;
mod day3;

fn main() {
    let sum_max_calories = day1::calculate_max_calories("./day1/calories.txt")
        .expect("ERROR TRYING TO RUN DAY1 CHALLENGE");
    println!("RESULT DAY 1: {sum_max_calories}");

    let mut score = day2::part1("./day2/strategy.txt")
        .expect("ERROR TRYING TO RUN DAY2 PART 1 CHALLENGE");
    println!("RESULT DAY 2 - part 1: {score}");

   score = day2::part2("./day2/strategy.txt")
        .expect("ERROR TRYING TO RUN DAY2 PART 2 CHALLENGE");
    println!("RESULT DAY 2 - part 2: {score}");
}

