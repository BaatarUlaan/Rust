#[derive(Clone)]
struct A {
    x: u32,
    y:u32,
}

#[derive(Copy, Clone)]
struct B {
    x: u32,
}

fn main() {
    let mut a1 = A { x: 1 , y: 2 };
    let mut kb=&mut a1.x;
    *kb+=1;
    println!("a1.x = {}",kb);
    
}