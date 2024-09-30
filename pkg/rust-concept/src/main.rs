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
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", tup.1);

    // list type
    let a: [i32; 5] = [1,2,3,4,5];
    let first = a[0];
    let last = a[a.len() - 1];
    let b = [3; 5];
    let months = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
}
