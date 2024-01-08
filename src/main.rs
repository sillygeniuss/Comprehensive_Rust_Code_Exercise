fn main() {
    println!("Edit me!");
    // static typing
    let x: i32 = 10;
    println!("x: {x}");
    // becuz of static typing, {x} is immutable, this will not work
    // x = 20;
    // println!("x: {x}");

    // String | &str
    // default immutable
    let plantform = "x86_64";
    println!("plantform: {plantform}");

    // explicit mutable string
    let mut system = "Linux";
    println!("system: {system}");

    system = "Windows";
    println!("system: {system}");

    // escape quotes with # or \
    println!(r#"<a href="link.html">link</a>"#);

    let n = 20;
    println!("fib({n}): {}", fib(n));
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
