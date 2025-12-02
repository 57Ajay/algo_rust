#[allow(dead_code)]
pub fn main() -> Option<u32> {
    let mut password: u32 = 0;
    let mut dial: i32 = 50;

    let data = std::fs::read("src/aoc/input.txt").expect("failed to read");

    for line in data.split(|b| *b == b'\n') {
        if !line.is_empty() {
            // update_dial(line, &mut dial, &mut password);
            update_dial_as_per_0x434c49434b(line, &mut dial, &mut password);
        }
    }

    Some(password)
}

#[allow(dead_code)]
fn update_dial(line: &[u8], dial: &mut i32, password: &mut u32) {
    let direction = line[0];

    let num = atoi(&line[1..]);

    match direction {
        b'L' => {
            *dial = (*dial - num).rem_euclid(100);
        }
        b'R' => {
            *dial = (*dial + num).rem_euclid(100);
        }
        _ => unreachable!(),
    }

    if *dial == 0 {
        *password += 1;
    }
}

#[allow(dead_code)]
fn update_dial_as_per_0x434c49434b(line: &[u8], dial: &mut i32, password: &mut u32) {
    let dir = line[0];
    let steps = atoi(&line[1..]);

    let hits = match dir {
        b'L' => {
            let start = *dial % 100;
            let first_k = if start == 0 { 100 } else { start };
            if steps >= first_k {
                1 + (steps - first_k) / 100
            } else {
                0
            }
        }
        b'R' => {
            let start = *dial % 100;
            let first_k_raw = (100 - start) % 100;
            let first_k = if first_k_raw == 0 { 100 } else { first_k_raw };
            if steps >= first_k {
                1 + (steps - first_k) / 100
            } else {
                0
            }
        }
        _ => unreachable!(),
    };

    *password += hits as u32;

    match dir {
        b'L' => *dial = (*dial - steps).rem_euclid(100),
        b'R' => *dial = (*dial + steps).rem_euclid(100),
        _ => unreachable!(),
    };
}

fn atoi(bytes: &[u8]) -> i32 {
    let mut n = 0;
    for b in bytes {
        n = n * 10 + (b - b'0') as i32;
    }
    n
}
