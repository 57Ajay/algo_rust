#[allow(dead_code)]
pub fn main() -> Option<u64> {
    let data = std::fs::read("src/aoc/input3.txt").expect("failed to read");
    let mut total_joltage: u64 = 0;
    for bank in data.split(|x| *x == b'\n') {
        if !bank.is_empty() {
            let largest_joltage = get_largest_joltage_p2(bank);
            total_joltage += largest_joltage;
        }
    }
    Some(total_joltage)
}

#[allow(dead_code)]
fn get_largest_joltage(bank: &[u8]) -> u64 {
    let a = bank[0];
    let b = bank[1];

    let mut largest_joltage: [u8; 2] = [a, b];

    for i in 1..bank.len() - 1 {
        let a = bank[i];
        let b = bank[i + 1];
        if a > largest_joltage[0] {
            largest_joltage[0] = a;
            largest_joltage[1] = b;
        } else if b > largest_joltage[1] {
            largest_joltage[1] = b;
        }
    }

    let num = (largest_joltage[0] - b'0') * 10 + largest_joltage[1] - b'0';
    return num as u64;
}

#[allow(dead_code)]
fn get_largest_joltage_p2(bank: &[u8]) -> u64 {
    let max12 = max_12(bank);
    return max12;
}

#[allow(dead_code)]
fn max_12(seq: &[u8]) -> u64 {
    const K: usize = 12;

    let mut drop = seq.len().saturating_sub(K);
    let mut stack: Vec<u8> = Vec::with_capacity(K);

    for &ch in seq {
        let digit = ch - b'0';

        while drop > 0 && !stack.is_empty() && stack.last().unwrap() < &digit {
            stack.pop();
            drop -= 1;
        }

        stack.push(digit);
    }

    stack.truncate(K);

    let mut val: u64 = 0;
    for d in stack {
        val = val * 10 + d as u64;
    }

    val
}
