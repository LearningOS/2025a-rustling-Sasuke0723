// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch  for a subcommand
// hint.

// I AM DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
