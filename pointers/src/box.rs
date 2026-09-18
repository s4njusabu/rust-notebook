fn main() {
    let a1 = 10;             // stack value
    let a2 = Box::new(10);   // heap value, a2 holds pointer to it

    println!("A");
    println!("{a1} : {:p}", &a1); // address of a1
    println!("{a2} : {:p}", a2);  // address a2 points to (heap)

    let b1 = a1;             // copy (i32 implements Copy by default)
    let b2 = a2.clone();     // clone -> new heap allocation
    let b3 = &a2;            // reference to a2

    println!("B");
    println!("{b1} : {:p}", &b1); // address of b1
    println!("{b2} : {:p}", b2);  // address b2 points to (different from a2)
    println!("{b3} : {:p}", *b3); // deref b3 -> same address as a2
}