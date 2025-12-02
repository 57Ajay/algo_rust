#[allow(dead_code)]
pub fn main() -> Option<u64> {
    let data = std::fs::read("src/aoc/input2.txt").expect("failed to read");
    let mut v = Vec::<u64>::with_capacity(1024);

    for range in data.split(|b| *b == b',') {
        get_invalid_ids(range, &mut v);
    }

    let total_sum = v.iter().sum::<u64>();
    // println!("{:?}", v);
    Some(total_sum)
}

#[allow(dead_code)]
fn get_invalid_ids(range: &[u8], vec: &mut Vec<u64>) {
    let mut it = range.split(|c| *c == b'-');
    let s_bytes = match it.next() {
        Some(v) => v,
        None => return,
    };
    let e_bytes = match it.next() {
        Some(v) => v,
        None => return,
    };

    let start = parse_u64(s_bytes);
    let end = parse_u64(e_bytes);

    for n in start..=end {
        if is_repeating_p2(n) {
            vec.push(n);
        }
    }
}

#[allow(dead_code)]
fn parse_u64(bytes: &[u8]) -> u64 {
    let mut n = 0;
    for &b in bytes {
        if b < b'0' || b > b'9' {
            continue;
        }
        n = n * 10 + (b - b'0') as u64;
    }
    n
}

#[allow(dead_code)]
fn is_repeating(n: u64) -> bool {
    let mut buf = [0u8; 20];
    let mut curr = n;
    let mut len = 0;

    if n == 0 {
        return false;
    }

    while curr > 0 {
        buf[len] = (curr % 10) as u8;
        curr /= 10;
        len += 1;
    }

    if len % 2 != 0 {
        return false;
    }

    let digits = &buf[..len];
    let half = len / 2;

    for i in 0..half {
        if digits[i] != digits[i + half] {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
fn is_repeating_p2(n: u64) -> bool {
    let mut buf = [0u8; 20];
    let mut curr = n;
    let mut len = 0;

    if n == 0 {
        return false;
    }

    while curr > 0 {
        buf[len] = (curr % 10) as u8;
        curr /= 10;
        len += 1;
    }

    let digits = &buf[..len];

    for k in 1..=len / 2 {
        if len % k == 0 {
            let first_chunk = &digits[..k];
            if digits.chunks(k).all(|chunk| chunk == first_chunk) {
                return true;
            }
        }
    }

    false
}

