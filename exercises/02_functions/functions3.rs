fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    // u8 -> 8bit - ((2^8)-1) - 0-255
    let num: u8 = 255;
    call_me(num);
}
