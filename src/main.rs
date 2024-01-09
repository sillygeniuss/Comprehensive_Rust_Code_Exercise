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
    let platform = "x86_64";
    println!("platform: {platform}");

    // explicit mutable string
    let mut system = "Linux";
    println!("system: {system}");

    system = "Windows";
    println!("system: {system}");

    // escape quotes with # or \
    println!(r#"<a href="link.html">link</a>"#);

    let n = 20;
    println!("fib({n}): {}", fib(n));

    stage_condition();
    stage_tuples_arrays();
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn stage_condition() {
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

fn stage_tuples_arrays() {
    // array
    let mut a: [i32; 10] = [42;10];
    println!("index 5 is {}", a[4]);
    // println!("out-of-bounds value is {}", a[10]);
    println!("complete array content is {a:?}");
    println!("complete array pretty content is {a:#?}");

    a[4] = 10;
    println!("change value of index 5: {}", a[4]);

    // tuples
    let t: (i32, f32, bool) = (10, 3.14, true);
    println!("index 1 of tuples is: {}", t.1);

    let arr1 = [1, 2, 3, 4, 5];
    match arr1 {
        [a@.., x, b] if x == 4 => {
            println!("prefix arr{:#?}", a);
        }
        _ => {
            println!("not match");
        }
    }
}
