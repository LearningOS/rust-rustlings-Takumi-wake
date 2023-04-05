// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.



fn main() {
    call_me(3);

    let mut y = 0;
    let x = if y == 2 {1;} else {0;};
    let z = y = 0;
    println!("z is {:?}", z);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
