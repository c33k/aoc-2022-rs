use std::collections::HashSet;

fn get_value(c: &char) -> u32 {
    // a-z - 1 to 26
    // A-Z - 27 - 53
    if c.is_lowercase() {
        return (*c as u32) - 96; // 97 is a's value on ASCII.
    }

    return (*c as u32) - 38; // 65 (A's ASCII value) - 27 (as per line 5)
}

// Time complexity: O(N)
// Space complexity: O(N)
pub fn part1(path: &str) -> Result<u32, std::io::Error> {
    let rucksacks = std::fs::read_to_string(path)?;
    
    let mut letters_set: HashSet<char> = HashSet::new();
    let mut sum: u32 = 0;

    for line in rucksacks.lines() {
        // 1 - go through first half of the string, adding each char to Set
        // 2 - go through second half and find repeated character
        // 3 - get priority value of character and add to sum
        println!(">>>>>>>>> STARTING {line}");

        letters_set.clear();
        
        let half = (line.len() as i32) / 2; 

        for (i, c) in line.chars().enumerate() {
            if (i as i32) < half {
                letters_set.insert(c);
                println!("Adding {} to SET", c);
            } else {
                if letters_set.contains(&c) {
                    println!("FOUND on second half: value of  {} is {}", c, get_value(&c));
                    sum += get_value(&c);
                    break;
                }
            }
        } 
    }

    Ok(sum)
}
