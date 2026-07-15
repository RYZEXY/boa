use diplomat as _;
use diplomat_runtime as _;

fn main() {
    let result = boa_ffi::ffi::add(2, 3);
    println!("Diplomat demo: 2 + 3 = {result}");
}
