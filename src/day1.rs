// The solution bellow uses the fact that we just need to handle the first 3 max.
// In this way, our solution is O(1) in space and O(N) in time (we need to read 
// the full file one time)
//
// In the comments, there is a solution using a vector to store the totals.
// For summing the X max, we sort the array and iterate over it in reverse 
// order. This solution is more generic and has O(N) space complexity and 
// O(NlogN) time complexity. But I've decided to keep the more verbose one
// because it is totally unnecessary to use vector and sort for this specific
// instance of the problem. 

pub fn calculate_max_calories(file_path: &str) -> 
    Result<u32, std::io::Error> {
    
    // let mut calories_totals = Vec::new();
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current_total: u32 = 0;
    let content = std::fs::read_to_string(file_path)?;
    
    for line in content.lines() {
        if line.is_empty() {
            // calories_totals.push(current_total);
            if current_total > first {
                let aux = second;
                second = first;
                third = aux;
                first = current_total;
            } else if current_total > second {
                third = second;
                second = current_total;
            } else if current_total > third {
                third = current_total;
            }

            current_total = 0;
        } else {
            let calories = line.parse::<u32>()
                .expect("Invalid input. Couldn't convert to u32.");
            current_total = current_total + calories;
        }
    }

    Ok(first + second + third)

    //calories_totals.sort();
    
    //let len = calories_totals.len();
    //let total = calories_totals[len - 1] + 
    //calories_totals[if len-2 >= 0 { calories_totals[len-2] } else { 0 } ] +
    //calories_totals[if len-3 >= 0 { calories_totals[len-3] } else { 0 } ];
    
    // Ok(calories_totals.iter().rev().take(3).sum())
}
