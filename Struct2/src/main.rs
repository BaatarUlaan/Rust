#[derive(Clone)]
struct A {
    x: u32,
}

#[derive(Copy, Clone)]
struct B {
    x: u32,
}

fn main() {
    let a1 = A { x: 1 };
    let a2 = a1;            // ✅ 所有权移动，没有Copy
    let a3 = a1.clone();    // ❌ a1所有权已然移动

    let b1 = B { x: 1 };
    let b2 = b1;            // ✅ 自动复制（Copy）
    let b3 = b1.clone();    // ✅ 当然也可以手动 clone（但没必要）
}