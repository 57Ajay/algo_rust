use algo_rust::data_structures::lists::singly_linked_list;
use std::io;

#[allow(dead_code)]
struct Pair {
    val: usize,
    idx: usize,
}

#[allow(dead_code)]
pub fn main() {
    let mut sll_stack = singly_linked_list::LinkedList::<Pair>::new();

    let mut arr_size = String::new();
    let mut arr_values = String::new();

    io::stdin().read_line(&mut arr_size).unwrap();
    io::stdin().read_line(&mut arr_values).unwrap();

    let arr_size = arr_size.trim().parse::<usize>().unwrap();

    let arr_values = arr_values
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    assert!(arr_size == arr_values.len());

    for i in 0..arr_size {
        while let Some(top) = sll_stack.peek() {
            if top.val < arr_values[i] {
                break;
            }
            sll_stack.pop();
        }

        if let Some(top) = sll_stack.peek() {
            println!("{}", top.idx);
        } else {
            println!("{}", 0);
        }

        sll_stack.push(Pair {
            val: arr_values[i],
            idx: i + 1,
        });
    }
}

// much better solution (submited to cses)
// use std::io::{self, Read};
//
// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();
//     let mut it = input.split_whitespace();
//
//     let n: usize = it.next().unwrap().parse().unwrap();
//     let arr: Vec<usize> = (0..n)
//         .map(|_| it.next().unwrap().parse().unwrap())
//         .collect();
//
//     let mut stack: Vec<(usize, usize)> = Vec::new();
//
//     let mut out = String::new();
//
//     for i in 0..n {
//         while let Some(&(top_val, _)) = stack.last() {
//             if top_val < arr[i] {
//                 break;
//             }
//             stack.pop();
//         }
//
//         if let Some(&(_, idx)) = stack.last() {
//             out.push_str(&format!("{} ", idx));
//         } else {
//             out.push_str("0 ");
//         }
//
//         stack.push((arr[i], i + 1));
//     }
//
//     print!("{}", out.trim_end());
// }
