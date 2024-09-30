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
    let a: [i32; 5] = [1,2,3,4,5];
    let _first = a[0];
    let _last = a[a.len() - 1];
    let _b = [3; 5];
    let _months = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];

    // function
    print_labeled_measurement(10, "minutes");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The value is: {}{}", value, unit_label);
}
