
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    let l1 = List::Cons(1, Box::new(List::Nil));

    println!("{:?}", l1);
    if let List::Cons(n1, val) = l1 {
        println!("{n1} {val:?}");
    }
}