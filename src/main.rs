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

    condition_test();
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn condition_test() {
    let x = 20;

    // difference between expression and statement
    let size = if x < 20 { "small" } else { "large" };
    // let size = if x < 20 { "small"; } else { "large" };

    println!("size: {size}");

    for x in 1..5 {
        println!("x: {}", x);
    }

    // use `break` for expression
    let (mut a, mut b) = (100, 53);
    let result = loop {
        if a == b {
            break a;
        }

        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    };
    println!("result: {}", result);

    // use `label` break nested loop
    'flag: for x in 1..5 {
        println!("x: {}", x);
        let mut i = 0;
        while i < x {
            i += 1;
            if i == 3 {
                break 'flag;
            }
        }
    }
}
