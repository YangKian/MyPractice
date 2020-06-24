use std::thread;
use std::time::Duration;

//  We can create a struct that will hold the closure and the resulting value
// of calling the closure. The struct will execute the closure only if we need
// the resulting value, and it will cache the resulting value so the rest of
// our code doesn’t have to be responsible for saving and reusing the result.
//
// All closures implement at least one of the traits: Fn, FnMut, or FnOnce.
// Add types to the Fn trait bound to represent the types of the parameters
// and return values the closures must have to match this trait bound.
struct Cacher<T>
where T: Fn(u32) -> u32, {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // we’re using a closure because we want to define the code to call at
    // one point, store that code, and call it at a later point;
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// /**
//  Closures can capture values from their environment in three ways, which directly map to
//  the three ways a function can take a parameter: taking ownership, borrowing mutably, and
//  borrowing immutably. These are encoded in the three Fn traits as follows:
//
//     - FnOnce consumes the variables it captures from its enclosing scope, known as
//       the closure’s environment. To consume the captured variables, the closure must
//       take ownership of these variables and move them into the closure when it is
//       defined. The Once part of the name represents the fact that the closure can’t
//       take ownership of the same variables more than once, so it can be called only once.
//     - FnMut can change the environment because it mutably borrows values.
//     - Fn borrows values from the environment immutably.
//
//  All closures implement FnOnce because they can all be called at least once.
//  Closures that don’t move the captured variables also implement FnMut, and
//  closures that don’t need mutable access to the captured variables also implement Fn.
// */