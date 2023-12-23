fn main() {
    test();
    println!("{}", true_or_false());
    test2();
    test3();

    let pointer = pointer();
    println!("{:?}", pointer);
}

fn test() {
    println!("{}World", "Hellow ");
}

fn test2() {
    for i in 1..11 {
        println!("Counting...{}", i);
    }
}

fn test3() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter first number:");
    std::io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    println!("Enter second number:");
    std::io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num1: i32 = num1.trim().parse().expect("Not a number");
    let num2: i32 = num2.trim().parse().expect("Not a number");

    println!("{} + {} = {}", num1, num2, num1 + num2);
}

fn true_or_false() -> bool {
    true
}

fn pointer() -> *mut i32 {
    let mut x = 5;
    let y = &mut x;
    return y as *mut i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true_or_false(), true);
    }
}
