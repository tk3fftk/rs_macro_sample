use funclog::funclog;
use tomlstruct::tomlstruct;
use typename::{TypeName, TypeNameTrait};

tomlstruct! {
    [Hello]
    name = "hello"
    version = 1.0
}

#[derive(TypeName)]
struct Hello2;

#[funclog]
fn hello() {
    println!("hello");
}

fn main() {
    let _ = Hello {
        name: String::from("hello"),
        version: 1.0,
    };

    let x = Hello2;
    dbg!(x.type_name());

    hello();
}
