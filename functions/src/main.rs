fn main() {
    println!("Hello, world!");

    another_function(5);

    another_function2();

    println!(
        "The value of another_function3 is: {}",
        another_function3(2)
    );

    another_function4();

    another_function5();

    another_function6();

    another_function7();
}

fn another_function(i: i32) {
    println!("The value of i is: {}", i);
}

fn another_function2() -> i32 {
    5
}

fn another_function3(i: i32) -> i32 {
    if i > 5 {
        5
    } else {
        6
    }
}

fn another_function4() {
    let x = if true { 5 } else { 6 };

    println!("The value of x is: {}", x);
}

fn another_function5() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // loop 本身是一个表达式，它的值是靠 break 返回的值
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn another_function6() {
    while true {
        println!("again!");
    }
}

fn another_function7() {
    for i in 1..4 {
        println!("{}!", i);
    }
}
