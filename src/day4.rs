// In how many assignment pairs does one range fully contain the other?
// Each line in the input has the format "x-y,a-b" 

type ContainsFN = fn(&(i32, i32), &(i32, i32)) -> bool;

fn contains_fully(range1: &(i32, i32), range2: &(i32, i32)) -> bool {
    (range2.0 - range1.0) >= 0 && (range1.1 - range2.1) >= 0
}

fn contains_part(range1: &(i32, i32), range2: &(i32, i32)) -> bool {
    (range1.0 <= range2.0 && range1.1 >= range2.0) ||
        (range1.0 <= range2.1 && range1.1 >= range2.1)
}

fn get_ranges(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut chunks = line.splitn(2, ",");
    let first_range_chunk = chunks.next().unwrap();
    let second_range_chunk = chunks.next().unwrap();
 
    let mut range_1 = first_range_chunk.splitn(2, "-");
    let mut range_2 = second_range_chunk.splitn(2, "-");
 
    let mut left = range_1.next().unwrap();
    let mut right = range_1.next().unwrap();

    let left_range = 
        (left.parse::<i32>().expect("a"), right.parse::<i32>().expect("a"));
    
    left = range_2.next().unwrap();
    right = range_2.next().unwrap();

    let right_range = 
        (left.parse::<i32>().expect("a"), right.parse::<i32>().expect("a"));

    let ranges = (left_range, right_range);

    ranges
}

pub fn solve(input_path: &str, is_part_1: bool) -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string(input_path)?;
    let mut sum = 0;

    for line in content.lines() {
        let ranges = get_ranges(&line);

        let contains_fn: ContainsFN = if is_part_1 { contains_fully } else { contains_part };

        if contains_fn(&ranges.0, &ranges.1) || contains_fully(&ranges.1, &ranges.0) {
            sum += 1;
        } 
    }

    Ok(sum)
} 
