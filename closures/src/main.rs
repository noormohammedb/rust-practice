use std::{thread, time::Duration};

pub fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    // thread::sleep(Duration::from_secs(2));
    thread::sleep(Duration::from_millis(500));
    intensity
}

fn main() {
    // println!("{}", simulate_expensive_calculation(2));

    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
    closures::closure_two();
    closures::closure_three();
}

pub struct Cacher<T: Fn(u32) -> u32> {
    pub calculation: T,
    pub value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
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

pub fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulate_expensive_calculation(intensity);

    // let expensive_closure = |num| {
    // let expensive_closure = |num: u32| -> u32 {
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(500));
        num
    });

    if intensity < 25 {
        println!("Today, do {}, pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}
