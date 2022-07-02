fn main() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    fn do_twice<T>(f: T, arg: i32) -> i32
    where
        T: Fn(i32) -> i32,
    {
        f(arg) + f(arg)
    }

    // Fn, FnMut and FnOnce
    let result = do_twice(add_one, 5);
    println!("The Result is: {}", result);

    let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_statuses);

    fn _returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    fn _returns_closure_v2(a: i32) -> Box<dyn Fn(i32) -> i32> {
        if a > 0 {
            Box::new(move |b| a + b)
        } else {
            Box::new(move |b| a - b)
        }
    }
}
