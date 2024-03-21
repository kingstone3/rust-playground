fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let mut x = 6;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    let num: i32 = "42".parse().expect("Not a number!");
    println!("The value of num is: {}", num);

    let nums = [1, 2, 3, 4, 5];
    let nums2 = [3; 5];
    let num3: [i32; 5] = [1, 2, 3, 4, 5];
}
