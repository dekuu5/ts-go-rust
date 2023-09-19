fn main() {

    let mut foo = Some(4);

    if let Some(shit) = foo {
        println!("{}", shit);
        shit = 555;
    }
    println!("Hello, world!");
}
