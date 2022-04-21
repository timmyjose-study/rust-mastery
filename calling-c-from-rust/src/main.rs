extern "C" {
    pub fn strlen(s: *const u8) -> usize;
}

fn main() {
    let string = "Hello, world";
    unsafe {
        println!("{:?} has length {}", string, strlen(string.as_ptr()));
    }
}
