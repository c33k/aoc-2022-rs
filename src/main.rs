mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let mut score = day1::calculate_max_calories("./day1/calories.txt")
        .expect("ERROR TRYING TO RUN DAY1 CHALLENGE");
    println!("RESULT DAY 1: {score}"); 

    score = day2::part1("./day2/strategy.txt")
        .expect("ERROR TRYING TO RUN DAY2 PART 1 CHALLENGE");
    println!("RESULT DAY 2 - part 1: {score}");
    score = day2::part2("./day2/strategy.txt")
        .expect("ERROR TRYING TO RUN DAY2 PART 2 CHALLENGE");
    println!("RESULT DAY 2 - part 2: {score}");

    score = day3::part1("./day3/rucksack_reorganization.txt")
        .expect("ERROR TRYING TO RUN DAY3 PART 1");
    println!("RESULT DAY 3 - part 1: {score}");
    score = day3::part2("./day3/rucksack_reorganization.txt")
        .expect("ERROR TRYING TO RUN DAY3 PART 2");
    println!("RESULT DAY 3 - part 2: {score}");

    let mut day4_result = day4::solve("./day4/input.txt", true)
        .expect("ERROR TRYING TO RUN DAY4 PART 1");
    println!("RESULT DAY 4 - part 1: {day4_result}");
    day4_result = day4::solve("./day4/input.txt", false)
        .expect("ERROR TRYING TO RUN DAY4 PART 2");
    println!("RESULT DAY 4 - part 2: {day4_result}");
}

