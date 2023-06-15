
extern "C" {
    pub fn print();
}

#[no_mangle]
pub extern "C" fn hello() -> i32 {
    unsafe { print(); }
    42
}

