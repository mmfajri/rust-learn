// NOTE: MAIN Function
fn main() {
    println!("Hello, world!");
}

// NOTE: Unit Testing
#[test]
fn hello_test() {
    println!("Helloe test!");
}

// NOTE: Variable (IMMUTABLE)
#[test]
fn test_variable() {
    let name = "YEAH ULALALAL";
    println!("Hello {}", name);
}

// NOTE: Variable (MUTATABLE)
#[test]
fn test_mutable() {
    let mut name = "yeaahhow";
    println!("Hello {}", name);

    name = "howdie";
    println!("Hello {}", name);
}

// NOTE: Static Typing
#[test]
fn test_static_typing() {
    let mut name = "yeaahhow";
    println!("Hello {}", name);

    // name = 10; --> CAN'T DO THIS, RIDICULOUS, because is static typing
    name = "name again";
    println!("Hello {}", name);
}

// NOTE: Shadowing
#[test]
fn test_shadowing() {
    let name = "yeaahhow";
    println!("Hello {}", name);

    // name = 10; --> CAN'T DO THIS, RIDICULOUS, because is static typing
    let name = 10;
    println!("Hello {}", name);
}

// NOTE: Data Type
#[test]
fn test_explicit() {
    let age: i32 = 20;
    println!("{}", age);

    let num: usize;
    num = 30;
    println!("{}", num)
}
#[test]
fn test_number() {
    let a: i8 = 20;
    println!("{}", a);

    let b: f32;
    b = 10.5;
    println!("{}", b)
}

// NOTE: Conversion Data Type
#[test]
fn test_number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i16 = a as i16;
    println!("{}", c);
}

// NOTE:Numeric Operation
#[test]
fn test_numeric_operation() {
    let a = 10;
    let b = 10;
    let c = a * b;
    let d = a / b;
    println!("{}", c);
    println!("{}", d);

    // Can also support augmented operation (-=, +=, etc)
    let mut num = 10;

    num -= 10;
    println!("{}", num);

    num += 10;
    println!("{}", num);

    num *= 10;
    println!("{}", num);
}

// NOTE: Boolean
#[test]
fn test_boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}
