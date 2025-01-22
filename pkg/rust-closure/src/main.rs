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
}

// struct Cache<T, K, V>
// where
//     T: Fn(K) -> V,
//     K: Eq + Hash + Copy,
// {
//     calculation: T,
//     value: HashMap<K, V>,
// }

// impl<T, K, V> Cache<T, K, V>
// where
//     T: Fn(K) -> V,
//     K: Eq + Hash + Copy,
//     V: Clone,
// {
//     fn new(calculation: T) -> Cache<T, K, V> {
//         Cache {
//             calculation
//             value: HashMap::new(),
//         }
//     }

//     fn value(&mut self, arg: K) -> V {
//         self.value.entry((arg).or_insert_with(|| (self.calculation)(arg))).clone()
//     }
// }

struct Cacher<F, A, R>
where
    F: Fn(A) -> R,
    A: Eq + Hash + Copy,
    R: Clone,
{
    calculation: F,
    values: Option<A, R>,
}

impl<F, A, R> Cacher<F, A, R>
where
    F: Fn(A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    fn new(calculation: F) -> Cacher<F, A, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> R {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let result = (self.calculation)(arg.clone());
                self.values.insert(arg.clone(), result.clone());
                result
            },
        }
    }
}

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32
// {
//     calculation: T,
//     value: Option<u32>,
// }

// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }
//     }
// }


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num: &u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(&intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(&intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(&intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cache::new(|a: &u32| *a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
