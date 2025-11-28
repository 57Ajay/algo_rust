use std::collections::VecDeque;
use std::io;
#[allow(dead_code)]
pub fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let buf = buf.trim().parse::<usize>().unwrap();

    let mut q = VecDeque::<usize>::new();

    for i in 1..=buf {
        q.push_back(i);
    }

    let mut first = true;
    while q.len() > 0 {
        let s = q.pop_front().unwrap();
        q.push_back(s);

        let v = q.pop_front().unwrap();
        if !first {
            print!(" ");
        }
        print!("{}", v);
        first = false;
    }
}
