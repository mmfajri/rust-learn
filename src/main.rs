// NOTE: MAIN Function
fn main() {
    println!("Hello, world!");
}

// NOTE: Unit Testing
// to run the program run this prompt
// cargo test <name_function> -- --exact-nocapture
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

// NOTE: Char
#[test]
fn test_char() {
    let a = 'a';
    let b = 'b';

    println!("{} {}", a, b)
}

// NOTE: Tupel
#[test]
fn test_tupel() {
    let mut data: (i32, f64, bool) = (8, 4.23, true);
    println!("{:?}", data);

    //how to access the data on the tuple
    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("{} {} {}", a, b, c);

    // desctructuring the tupel
    let (x, y, _) = data;
    println!("{} {}", x, y);

    data.0 = 19;
    data.1 = 30.2;
    println!("{:?}", data);
}

// NOTE: Unit
// for function who doesn't need return any value
fn unit() {
    println!("This is Unit Rust, Empty Tuple")
}
#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    // you can use this to declare empty tupel
    let datas: () = ();
    println!("{:?}", datas);
}

// NOTE: Array
#[test]
fn test_array() {
    let mut array = [1, 2, 3, 4, 5];
    let array_explicit: [i32; 5] = [6, 7, 8, 9, 0]; // --> explicit

    println!("{:?}", array);
    println!("{:?}", array_explicit);

    let a = array[0];
    let b = array[1];
    let c = array[2];

    println!("{} {} {}", a, b, c);

    array[0] = 11;
    array[1] = 12;
    println!("{:?}", array);
}

// NOTE: Constant
const MINIMUM: i32 = 5;
#[test]
fn test_const() {
    const MAXIMUM: i32 = 100;
    println!("{} {}", MINIMUM, MAXIMUM);
}

// NOTE: Memory Management (Stack and Heap)
fn function_a() {
    let a = 10;
    let b = String::from("bard");

    println!("{} {}", a, b);
}
fn function_b() {
    let a = 10;
    let b = String::from("lala");

    println!("{} {}", a, b);
}
#[test]
fn test_stack_heap() {
    function_a();
    function_b();
}

// NOTE: String Slice
// for &str --> fix sized string  --> save it stack
#[test]
fn test_string_slice() {
    let name: &str = "        kjkjakj kjfkj ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

// NOTE: String
// for string_type --> not fix sized string --> save it heap
#[test]
fn test_string_type() {
    let mut name: String = String::from("Allo Test");
    name.push_str("guraaaa");
    println!("{}", name);

    let brip = name.replace("Test", "brip");
    println!("{}", brip);
}

// NOTE :Ownership
// Only 1 Ownership (variable) for 1 value
#[test]
fn test_data_copy() {
    //--> only for the data type that saved on the stack
    let a = 10;
    let mut b = a; // --> this only copy the value from variable a to b
    println!("{} {}", a, b);

    b = 50; // --> this only affect the value of variable b
    println!("{} {}", a, b);
}
#[test]
fn test_ownership_movement() {
    let name: String = String::from("Allo");
    println!("{}", name);

    let mut name1: String = name;
    println!("{}", name1);
    // println!("{}", name); //--> you can't do this on heap, the value of name already move their ownership to name

    name1 = String::from("braaap");
    println!("{}", name1);
}
#[test]
fn test_clone() {
    // Alternative to "Copy" the heap variable but it costly because it will to create a new stack
    // and then move it value them to heap
    let address: String = String::from("Baras");

    let city: String = address.clone();
    println!("{} || {}", address, city);
}

// NOTE: IF Expression
#[test]
fn test_if_expression() {
    let a = 6;

    if a >= 8 {
        println!("Good");
    } else if a >= 5 {
        println!("Medium");
    } else {
        println!("Bad");
    }
}

// NOTE: Let Statement
#[test]
fn test_let_statement() {
    let value = 7;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Medium"
    } else if value >= 3 {
        "Bad"
    } else {
        "Undefined"
    };
    println!("Result value -> {}", result);
}

// NOTE: Loop
#[test]
fn test_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("Counter : {}", counter)
    }
}

// NOTE: Loop return value
#[test]
fn test_loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

// NOTE: Loop Label
#[test]
fn test_loop_lable() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }
            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

// NOTE: While Loop
#[test]
fn test_while_loop() {
    println!("This is while loop test");
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter {}", counter);
        }
        counter += 1;
    }
}

// NOTE: For Loop
#[test]
fn test_array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value: {}", array[index]);
        index += 1;
    }
}

#[test]
fn test_for_loop_array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for item in array {
        println!("Value {}", item);
    }
}

#[test]
fn test_range() {
    // this is range exclusive
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Range Start {}", range.start); //Inclusive
    println!("Range End {}", range.end); //Exclusive

    for i in range {
        println!("Value {}", array[i])
    }
}

#[test]
fn test_range_inclusive() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..=4;
    println!("Range Start {}", range.start()); //Inclusive
    println!("Range End {}", range.end()); //Exclusive

    for i in range {
        println!("Value {}", array[i])
    }
}

// NOTE: Function
#[test]
fn say_hello() {
    println!("Allo...")
}
fn say_goodbye(name: &str, message: &str) {
    println!("Bye {}, {}", name, message)
}
#[test]
fn test_function() {
    say_hello();
    say_goodbye("Allo", "Have a nice day!");
}

// NOTE: Function Return Value
#[test]
fn test_function_return_value() {
    let result = factorial(5);
    println!("Result from Factorial Function = {}", result);
}
fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    } else {
        let mut result = 1;
        for i in 1..=n {
            result *= i;
        }
        result
    }
}

#[test]
fn test_recursive_function() {
    let result = recursive_factorial(5);
    println!("Result from Factorial Function = {}", result);
}

fn recursive_factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    return n * recursive_factorial(n - 1);
}
