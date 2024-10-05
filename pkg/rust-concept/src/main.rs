fn main() {
    // shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", tup.1);

    // list type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _last = a[a.len() - 1];
    let _b = [3; 5];
    let _months = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    // function
    print_labeled_measurement(10, "minutes");

    // formula and expression
    let y = {
        let x = 5;
        x + 1
    };
    println!("The value of y is: {}", y);

    // function with return value
    let x = five();
    println!("The value of x is: {}", x);

    // if expression
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("number was something other than zero");
    }

    // let with if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;
    'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {}", counter);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    println!("{}", type_of(&(1..4).rev()));

    // for
    let a = [10, 20, 30, 40, 50];
    for num in a {
        println!("the value is: {}", num);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }

    // ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The value is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
