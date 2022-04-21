#[derive(Debug)]
#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn inspect(&self) {
        println!("{}", format!("Point({}, {})", self.x, self.y));
    }
}

#[no_mangle]
pub extern "C" fn new_point(x: i32, y: i32) -> *mut Point {
    let p = Box::new(Point { x, y });
    Box::into_raw(p)
}

#[no_mangle]
pub extern "C" fn inspect_point(p: *mut Point) {
    unsafe {
        let point = Box::from_raw(p);
        point.inspect();
    }
}
