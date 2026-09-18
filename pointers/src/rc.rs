use std::rc::Rc;

fn main() {
    let r1 = Rc::new(Box::new(10));
    let r2 = Rc::clone(&r1);

    println!("{}", Rc::strong_count(&r2));


    println!("{}", **r1);
}