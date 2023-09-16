// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me("LALALA");
}

fn call_me(num: &str) {
    for i in 0..2 {
        println!("Ring! Call number {} + 1", num );
    }
}
