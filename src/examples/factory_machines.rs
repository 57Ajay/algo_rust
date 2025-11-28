use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    let mut mt = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    io::stdin().read_line(&mut mt).unwrap();

    let mut it = buf.split_whitespace().map(|x| x.parse::<u128>().unwrap());

    let _ = it.next().unwrap();
    let total_work = it.next().unwrap();

    let mut machine_time = mt
        .split_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    machine_time.sort_unstable();

    let mut low = 0u128;
    let mut high = 1_000_000_000_000_000_000u128;

    while low < high {
        let mid = low + (high - low) / 2;

        let produced: u128 = machine_time.iter().map(|t| mid as u128 / *t as u128).sum();

        if produced >= total_work as u128 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    println!("{}", low);
}
