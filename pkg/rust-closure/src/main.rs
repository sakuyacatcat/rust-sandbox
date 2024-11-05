use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

struct Cache<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cache<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
    V: Clone,
{
    fn new(calculation: T) -> Cache<T, K, V> {
        Cache {
            calculation
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        self.value.entry((arg).or_insert_with(|| (self.calculation)(arg))).clone()
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cache::new(|num| {
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
    let mut c = Cache::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
