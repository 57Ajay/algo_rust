mod aoc;
mod examples;
fn main() {
    // examples::ferris_wheel::main();
    let res = aoc::day3::main().unwrap();
    println!("answer: {:?}", res);
}
