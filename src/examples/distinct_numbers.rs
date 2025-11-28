use std::collections::HashSet;
use std::io;
#[allow(dead_code)]
pub fn main() {
    let mut values = String::new();
    let mut integers = String::new();

    io::stdin().read_line(&mut values).unwrap();
    io::stdin().read_line(&mut integers).unwrap();

    let values = values.trim().parse::<usize>().unwrap();

    if values == 1 {
        println!("1");
        return;
    }

    let mut set = HashSet::with_capacity(values);

    for num in integers.split_whitespace() {
        let val = num.parse::<usize>().unwrap();
        set.insert(val);
    }

    println!("{}", set.len());
}
