use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    let mut w_buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    io::stdin().read_line(&mut w_buf).unwrap();

    let mut it = buf.split_whitespace().map(|x| x.parse::<usize>().unwrap());

    let number_children = it.next().unwrap();
    let max_allowed_weight = it.next().unwrap();

    let mut children_age_arr = w_buf
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    children_age_arr.sort_unstable();

    let (mut i, mut j) = (0usize, number_children - 1);
    let mut gondolas = 0usize;

    while i <= j {
        if children_age_arr[i] + children_age_arr[j] <= max_allowed_weight {
            i += 1;
        }

        if j == 0 {
            gondolas += 1;
            break;
        }
        j -= 1;

        gondolas += 1;
    }

    println!("{}", gondolas);
}
