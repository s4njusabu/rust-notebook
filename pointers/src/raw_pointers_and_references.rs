fn main() {
    let mut num1 = 10;

    let p1: *const i32 = &num1 as *const i32;
    let p2: *mut i32 = &mut num1 as *mut i32;

    unsafe {
        println!("Before modification: {} {}", *p1, *p2);
        *p2 = 11;
        println!("After modification: {} {}", *p1, *p2);
    }

    let mut num2 = 10;

    let r1: &i32 = &num2;
    println!("Before modification: {}", *r1);

    let r2: &mut i32 = &mut num2;
    println!("Before modification: {}", *r2);
    *r2 = 11;
    println!("After modification: {}", *r2);
}
