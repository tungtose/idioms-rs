use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::take(name),
        }
    }
}

fn main() {
    let mut a = MyEnum::A {
        name: "name".to_string(),
        x: 0,
    };

    a_to_b(&mut a);

    if let MyEnum::B { name } = a {
        println!("Switched to B, {}", name);
    }
}
