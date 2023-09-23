// Rock, paper and scissor
// A and X - rock
// B and Y - paper
// C and Z - scissor

fn get_value(letter: &str) -> u32 {
    match letter {
        "A" | "X" => { return 1; } 
        "B" | "Y" => { return 2; }
        "C" |"Z"  => { return 3; }
        _ => { return 0; }
    }
}

fn get_value_wld(letter: &str) -> u32 {
     match letter {
        "X" => { return 0; } 
        "Y" => { return 3; }
        "Z"  => { return 6; }
        _ => { return 0; }
    }

}

pub fn part1(path: &str) -> Result<u32, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let mut score = 0;

    for line in content.lines() {
        let mut chunks = line.splitn(2, " ");
        let a = chunks.next().unwrap();
        let b = chunks.next().unwrap();
        
        let v_a= get_value(a);
        let v_b = get_value(b);

        score += v_b;

        if v_a == v_b {
            score += 3;
        } else if (v_b == 1 && v_a == 2) || 
            (v_b == 2 && v_a == 3) || 
            (v_b == 3 && v_a == 1) {
            score += 0;
        } else {
            score += 6;
        }
    }

    Ok(score)
}

pub fn part2(path: &str) -> Result<u32, std::io::Error> {
    // Now X, Y and Z change meaning:
    // X - LOOSE (0)
    // Y - DRAW (3)
    // Z - WIN (6)

    let content = std::fs::read_to_string(path)?;
    let mut score = 0;

    for line in content.lines() {
        let mut chunks = line.splitn(2, " ");
        let a = chunks.next().unwrap();
        let b = chunks.next().unwrap();
        
        score += get_value_wld(b);
        let v_a = get_value(a);

        if b == "Z" { // win
            score += if v_a == 3 { 1 } else { v_a+1 };         
        } else if b == "Y" { // draw
            score += v_a;
        } else { // loose
            score += if v_a == 1 { 3 } else { v_a-1 };         
        }
    }

    Ok(score)
}
