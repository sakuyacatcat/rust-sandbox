use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // closure that captures its environment
    let x = 4;
    // x is captured from the environment
    // and x's ownership is moved into the closure
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // iterator
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

struct Cacher<T, A, R>
where
    T: Fn(A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    calculation: T,
    value: HashMap<A, R>,
}

impl<T, A, R> Cacher<T, A, R>
where
    T: Fn(A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    fn new(calculation: T) -> Cacher<T, A, R> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> R {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let result = (self.calculation)(arg.clone());
                // don't need to clone arg because it's already passed to this struct scope
                self.value.insert(arg, result.clone());
                result
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v3 = c.value(1);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 1);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
