use std::collections::HashSet;
use std::collections::HashMap;

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
        letters_set.clear();
        
        let half = (line.len() as i32) / 2; 

        for (i, c) in line.chars().enumerate() {
            if (i as i32) < half {
                letters_set.insert(c);
            } else {
                if letters_set.contains(&c) {
                    sum += get_value(&c);
                    break;
                }
            }
        } 
    }

    Ok(sum)
}

pub fn part2(path: &str) -> Result<u32, std::io::Error> {
    let rucksacks = std::fs::read_to_string(path)?;
    
    let mut letters: HashMap<char, u32> = HashMap::new();
    let mut sum = 0;

    for (i, line) in rucksacks.lines().enumerate() {
        // Each group has 3 lines. So we use MOD to keep track of line in group
        let cur_line_in_group = (i % 3) as u32;

        if cur_line_in_group == 0 { 
            letters.clear();
        }
    
        for c in line.chars() {
            if letters.contains_key(&c) {
                // if we are on last line of the group and the letter appeared
                // ... on the previous two groups, se add it's value to sum.
                if cur_line_in_group == 2 && *letters.get(&c).unwrap() == 1 {
                    sum += get_value(&c);
                    break;
                } else {
                    // already found this in the past, so increment the group
                    if let Some(x) = letters.get_mut(&c) {
                        *x = cur_line_in_group;
                    }
                }
            } else {
                // if it is not inserted and it is in the first group, we insert
                // ... it. But if it is not in the first group, it means it 
                // ... didn't appear on previous groups. So no need to count it.
                if cur_line_in_group == 0 {
                    letters.insert(c, cur_line_in_group);
                }
            }
        }
    }

    Ok(sum)
}
