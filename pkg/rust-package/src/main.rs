use std::collections::HashMap as HM;

fn main() {
    let mut map = HM::new();
    map.insert(1, 2);

    println!("{:?}", map);
}
