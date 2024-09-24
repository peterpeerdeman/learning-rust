use std::any::Any;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::*;
use std::mem;
use std::result::Result;
use std::sync::Arc;
use std::sync::Mutex;

fn types() -> char {
    let value: u8 = 61;
    let char = value as char;
    println!("{char}");
    let string = "hellö world";
    println!("{string} has {} chars", string.chars().count());
    println!("{string} has {} bytes", string.len());
    char
}

fn mutability() -> i32 {
    let mut i = 1_000_000;
    i = i + 15 + 20;
    println!("Hello, mutable int! {i}");
    i
}

fn byte_manipulation() {
    let data = b"abc";
    println!("hex representation of data is {:x?}", data);
}

fn string_factory() -> String {
    let large = r#"313213j123 "bla" // \n \t jlslkjdsf sdf kjdsflkj"#.to_string();
    let reference = &large;
    println!(
        "reference points to large string on heap address: {:p}",
        reference
    );
    println!("bytesize of String is {}", large.len());
    println!("bytesize of reference is {}", mem::size_of_val(&large));
    println!("bytes representation of large is {:?}", large.as_bytes());

    // &large // cannot return reference to object that is owned by this function
    large // can return the object, passing the ownership to caller
}

fn references() {
    let value = 7;
    let reference = &7;

    //println!("value and reference equality: {}", value == reference);
    println!("value and dereferenced equality: {}", value == *reference);
}

fn moving_references() -> String {
    let mut string_object_created_in_method = string_factory();
    string_object_created_in_method.push('?'); //
    println!("mutated string {}", string_object_created_in_method);

    print_without_passing_ownership(&string_object_created_in_method);

    string_object_created_in_method
}

fn unicode() {
    let thumbs = '👍';
    println!("thumbs as hex: {:x?}", thumbs as u32);
    println!("hex as thumbs: \u{1f44d}");
}

fn print_without_passing_ownership(string_reference: &String) {
    // string_reference.push('!'); // -> can't mutate value of passed reference
    println!("printing without mutating: {string_reference}");
}

fn arrays() {
    let fruits = ["Apple", "Pear", "Banana"];
    let prefilled_with_apples = ["Apple"; 10];
    let sliced_inclusive = &fruits[1..=2];
    let sliced_exclusive = &prefilled_with_apples[2..=5];
    println!("inclusive slice: {:?}", sliced_inclusive);
    println!("exclusive slice: {:?}", sliced_exclusive);
}

fn vectors_and_tuples() {
    let vector_from_array: Vec<&str> = ["bluebox", "micromonsta", "se02"].into();
    println!("length of vector: {}", vector_from_array.len());

    let mut vector_of_tuples = vec![
        ("synth", "micromonsta"),
        ("controller", "hapax"),
        ("mixer", "bluebox"),
    ];
    println!(
        "capacity of the tuplevector is: {}",
        vector_of_tuples.capacity()
    );
    vector_of_tuples.push(("controller", "launchpad"));
    println!(
        "after pushing 1, capacity tuplevector is: {}",
        vector_of_tuples.capacity()
    );

    let (a, b) = vector_of_tuples[0];
    println!("deconstructed tuple into {} {}", a, b);
}

fn looping_and_matching() {
    let vector_of_tuples = vec![
        ("synth", "micromonsta"),
        ("controller", "hapax"),
        ("mixer", "bluebox"),
    ];

    for device in vector_of_tuples {
        match device {
            ("synth", s) => println!("device {} is a synth", s),
            ("controller", s) => println!("device {} is a controller", s),
            _ => println!("device is something else"),
        }
    }
}

fn structs_enums_and_impl() {
    #[derive(Debug)]
    enum DeviceType {
        Synth,
        Controller,
        Mixer,
    }

    #[derive(Debug)]
    struct Device {
        device_type: DeviceType,
        power_usage_amps: f32,
        name: String,
        connected: bool,
        //name: &'static str //static lifetime, this referenced value should live on after this block
    }

    impl Device {
        // takes unmutable reference to self
        fn is_high_powered(&self) -> bool {
            self.power_usage_amps > 2.0
        }

        // take mutable reference to self
        fn connect(&mut self) {
            self.connected = true;
        }
    }

    use DeviceType::*;
    let device0 = Device {
        device_type: Synth,
        power_usage_amps: 1.5,
        name: "MicroMonsta".to_string(),
        connected: false,
    };
    let device1 = Device {
        device_type: Synth,
        power_usage_amps: 0.5,
        name: "MicroMonsta".to_string(),
        connected: false,
    };
    let device2 = Device {
        device_type: Controller,
        power_usage_amps: 2.0,
        name: "Hapax".to_string(),
        connected: false,
    };

    let device3 = Device {
        device_type: Mixer,
        power_usage_amps: 2.0,
        name: "Bluebox".to_string(),
        connected: false,
    };

    let mut vector = vec![device0, device1, device2, device3];
    // ownership of vector is passed to the for loop:
    for device in &mut vector {
        match device.is_high_powered() {
            true => println!("low power {:?}, {}", device.device_type, device.name),
            false => println!("high power {:?}, {}", device.device_type, device.name),
        }

        match device.device_type {
            Synth => device.connect(),
            _ => (),
        }
    }
    println!("vector values mutated in for loop: {:?}", vector);
}

fn option_and_handling() {
    fn take_fifth(vector: &Vec<i32>) -> Option<i32> {
        if vector.len() < 5 {
            None
        } else {
            Some(vector[4])
        }
    }
    let long_array = vec![1, 2, 3, 4, 5];
    let short_array = vec![1, 2, 3];

    println!("unwrapping some{}", take_fifth(&long_array).unwrap());
    //println!("{}",take_fifth(short_array).unwrap()); -> will crash with panic (unwrap None)

    println!(
        "unwrapping none, falling back to value {}",
        take_fifth(&short_array).unwrap_or(0)
    );

    let number = take_fifth(&short_array).unwrap_or_else(|| {
        println!("something went wrong while unwrapping");
        0
    });
    println!("unwrapping or else, returning fallback: {}", number);
}

fn result_and_handling() {
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("het gaat fout".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

    match divide(4.0, 0.0) {
        Ok(_) => println!("it was ok"),
        Err(e) => println!("it was an error {}", e),
    }
}

fn vecdeq() {
    let mut vecdeq = VecDeque::from(vec![0, 1, 2]);
    let item = vecdeq.pop_front().unwrap();
    vecdeq.push_front(5);
    println!("{:?}", item);
    println!("{:?}", vecdeq);
}

fn traitbounds() {
    fn print_only_stringlike<T>(input: T)
    where
        T: AsRef<str> + Display,
    {
        println!("{}", input);
    }

    print_only_stringlike("this is an &string");
    print_only_stringlike("this is an String".to_string());
    //print_only_stringlike(1); // -> AsRef<str> not satisfied
}

fn chaining() {
    let range = 1..=10;
    let vec = range.collect::<Vec<u8>>();
    println!("numbers 1 to 10: {:?}", vec);

    let subvec = vec.into_iter().skip(2).take(3).collect::<Vec<u8>>();
    println!("{:?}", subvec);
}

fn iterators() {
    let vector = (0..=10).collect::<Vec<i32>>();
    let vector_a = vector.iter().map(|x| x + 2).collect::<Vec<i32>>();
    let vector_b = vector.iter().map(|x| x * x).collect::<Vec<i32>>();

    let mut vector_mut = vector.clone();
    vector_mut.iter_mut().for_each(|x| *x = *x + 100);
    println!("{:?}", vector_a);
    println!("{:?}", vector_b);
    println!("{:?}", vector);
}

fn closures() {
    let outside = 4;
    let closure = |inside: i32| outside + inside;
    println!("calling closure {:?}", closure(5));

    let num_vec = vec![2, 4, 6];
    let double_vec = &num_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();

    println!("mapped vector{:?}", double_vec);

    num_vec.iter().enumerate().for_each(|(index, value)| {
        println!("{}:{}", index, value);
    });
}

fn functional() {
    #[derive(Debug)]
    struct Measurement {
        date: &'static str,
        measurement: &'static str,
        value: f32,
    }

    impl Measurement {
        fn new(line: &'static str) -> Option<Measurement> {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                return None;
            }
            let value = parts[2].parse::<f32>();
            if !value.is_ok() {
                return None;
            }
            Some(Self {
                measurement: parts[1],
                date: parts[0],
                value: value.unwrap(),
            })
        }
        fn print(&self) {
            println!("{}, {}, {}", self.date, self.measurement, self.value,);
        }
    }

    let measurement_logs = [
        "20240917 temperature 19.0",
        "20240916 temperature 18.0",
        "\n",
        "20240915 temperature ",
        "20240914 temperature 0",
        "\n",
        "20240912 temperature 21.0",
    ];

    let measurements = measurement_logs
        .iter()
        .filter_map(|line| Measurement::new(line))
        .collect::<Vec<Measurement>>();
    println!("{:?}", measurements);
    measurements[0].print();
}

fn some_and_find() {
    let some_are_none = vec![Some("yes"), Some("yes"), None];
    let result1 = some_are_none.iter().all(|x| x.is_some());

    let result2 = some_are_none.iter().any(|x| x.is_some());

    println!("all are some: {}", result1);
    println!("some are some: {}", result2);

    let some_are_none = vec![Some(1), Some(1), None];
    let folded_total = some_are_none
        .iter()
        .fold(0, |total_so_far, next| total_so_far + next.unwrap_or(0));
    println!("folded total: {}", folded_total);

    let found_item = some_are_none
        .iter()
        .rev() //start at the end
        .find(|item| {
            return item.unwrap_or(0) == 1;
        });
    println!("found item: {}", found_item.unwrap().unwrap());
}

fn cycle() {
    let even_odd = vec!["even", "odd"];

    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();
    println!("{:?}", even_odd_vec);
}

fn debug_logging() {
    let number = 5;
    dbg!(number);
}

fn lifetimes() {
    #[derive(Debug)]
    struct City<'a> {
        name: &'a String,
        population: u32,
    }

    impl City<'_> {
        fn grow(&mut self) {
            self.population = self.population * 2;
        }
    }

    let cities = vec!["Purmerend".to_string(), "Amsterdam".to_string()];

    let mut my_city = City {
        name: &cities[0],
        population: 1800,
    };

    dbg!(my_city.name);
    dbg!(my_city.population);
    my_city.grow();
    dbg!(my_city.population);
}

fn cell_refcel() {
    #[derive(Debug)]
    struct Phone {
        name: &'static str,
        weight_gram: u32,
        on_sale: Cell<bool>,
        bought_by_customer_id: RefCell<u32>,
    }

    let nokia_3330 = Phone {
        name: "Nokia 3310",
        weight_gram: 400,
        on_sale: Cell::new(false),
        bought_by_customer_id: RefCell::new(0),
    };

    dbg!(&nokia_3330.on_sale);
    nokia_3330.on_sale.set(true);
    dbg!(&nokia_3330.on_sale);
    dbg!(&nokia_3330.bought_by_customer_id);
    nokia_3330.bought_by_customer_id.replace(101332);
    dbg!(&nokia_3330.bought_by_customer_id);

    let borrowed_bought_value = nokia_3330.bought_by_customer_id.borrow_mut();
    //let borrowed_bought_value2 = nokia_3330.bought_by_customer_id.borrow_mut(); //this would panic runtime
    dbg!(borrowed_bought_value);
}

fn mutex() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("{:?}", my_mutex);
    println!("{:?}", mutex_changer);
    *mutex_changer = 6;
    println!("{:?}", mutex_changer);

    let mut second_mut_changer = my_mutex.try_lock();
    if let Ok(value) = &second_mut_changer {
        dbg!(value);
    } else {
        dbg!("I didn't get the (second) lock");
    };
    std::mem::drop(mutex_changer);

    // immediately change without variable
    *my_mutex.lock().unwrap() = 7;
    println!("{:?}", my_mutex);

    *my_mutex.lock().unwrap() = 10;
    println!("{:?}", my_mutex);
}

fn multithreading_naive() {
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread_one = std::thread::spawn(move || {
        for i in 0..10 {
            *my_number1.lock().unwrap() += 1;
            println!("printing thread 1 iteration {}: {:?}", i, my_number1);
        }
    });

    let thread_two = std::thread::spawn(move || {
        for i in 0..10 {
            *my_number2.lock().unwrap() += 1;
            println!("printing thread 2 iteration {}: {:?}", i, my_number2);
        }
    });

    thread_one.join().unwrap();
    thread_two.join().unwrap();
    println!("done multithreading_naive");
}

fn multithreading() {
    let my_number = Arc::new(Mutex::new(0));
    let mut join_handle_vec = vec![];

    for thread in 0..2 {
        let my_number_clone = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || {
            for i in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
                println!(
                    "printing thread {} iteration {}: {:?}",
                    thread, i, my_number_clone
                );
            }
        });
        join_handle_vec.push(handle);
    }

    join_handle_vec.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });
    println!("{:?}", my_number);
    println!("done multithreading_naive");
}

fn channels() {
    use std::sync::mpsc::channel;
    let (sender, receiver) = channel();

    let sender_clone = sender.clone();

    let mut handlers = vec![];

    let handle1 = std::thread::spawn(move || {
        sender.send("Send a &str").unwrap();
    });

    let handle2 = std::thread::spawn(move || {
        sender_clone.send("Send another &str").unwrap();
    });

    handlers.push(handle1);
    handlers.push(handle2);

    for _ in handlers {
        // still prints in random order, depending on which thread finishes first
        println!("{:?}", receiver.recv().unwrap());
    }
}

fn big_multithreading() {
    use std::sync::mpsc::channel;
    use std::thread::spawn;

    const TOTAL: i32 = 1_000_000;
    const THREADS: i32 = 10;
    let per_thread = TOTAL / THREADS;

    let (sender, receiver) = channel();
    let big_vec = vec![0; TOTAL as usize];
    let mut result = vec![];
    let mut handlers = vec![];

    for i in 0..THREADS {
        let sender_clone = sender.clone();
        let mut work: Vec<u8> = Vec::with_capacity(per_thread as usize);
        let start = (i * per_thread) as usize;
        let end = ((i + 1) * per_thread) as usize;
        work.extend(&big_vec[start..end]);
        let handle = spawn(move || {
            for number in work.iter_mut() {
                *number += 1;
            }
            sender_clone.send(work).unwrap();
        });
        handlers.push(handle);
    }
    drop(sender);

    for handle in handlers {
        handle.join().unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        result.push(results);
    }

    let flattened_result = result.into_iter().flatten().collect::<Vec<u8>>();

    println!(
        "big work result length: {}, range {:?}",
        flattened_result.len(),
        &flattened_result[1_000..1_100]
    );
}

fn box_heap() {
    #[derive(Debug)]
    struct LargeStruct {
        data: [u8; 1_000_000],
    }

    let large_object = Box::new(LargeStruct {
        data: [0; 1_000_000],
    });
    println!("{:?}", large_object.type_id());
}

fn dyn_trait() {
    trait JustATrait: fmt::Debug {}

    enum EnumOfNumbers {
        I8(i8),
        AnotherI8(i8),
        OneMoreI8(i8),
    }
    impl JustATrait for EnumOfNumbers {}

    impl fmt::Debug for EnumOfNumbers {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                EnumOfNumbers::I8(value) => write!(f, "I8({})", value),
                EnumOfNumbers::AnotherI8(value) => write!(f, "AnotherI8({})", value),
                EnumOfNumbers::OneMoreI8(value) => write!(f, "OneMoreI8({})", value),
            }
        }
    }

    fn returns_a_trait() -> Box<dyn JustATrait> {
        let some_enum = EnumOfNumbers::I8(8);
        Box::new(some_enum)
    }

    let my_trait = returns_a_trait();
    println!("formatted trait with dyn {:?}", my_trait)
}

fn dereferencing() {
    use std::ops::Deref;

    struct HoldsAnumber(u8);
    impl Deref for HoldsAnumber {
        type Target = u8;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let my_number = HoldsAnumber(10);
    println!("dereferencing my own struct: {}", *my_number + 20)
}

mod print_things {
    #[derive(Debug)]
    pub struct PrintThings {
        pub first: u8,
        second: u8,
        third: u8,
    }
    impl PrintThings {
        pub fn create(i: u8) -> Self {
            Self {
                first: i,
                second: 2,
                third: 3,
            }
        }
        pub fn prints_one_thing<T: std::fmt::Display>(input: T) {
            println!("{}", input)
        }
    }

    impl std::fmt::Display for PrintThings {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}, {}, {}", self.first, self.second, self.third)
        }
    }
}

fn mods() {
    use crate::print_things::PrintThings;
    PrintThings::prints_one_thing(6);
    let my_struct = PrintThings::create(10);
    PrintThings::prints_one_thing(&my_struct);
    println!("custom displayed struct {}", my_struct);
}

fn rayon() {
    use rayon::prelude::*;
    use std::time::Instant;
    const TOTAL_ITEMS: usize = 1_200_000;

    let mut my_vec = vec![0; TOTAL_ITEMS];
    let before1 = Instant::now();
    my_vec
        .iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number+=index+1);
    println!("no rayon: {}ms {:?}", before1.elapsed().as_millis(), &my_vec[5000..5005]);

    let mut my_vec = vec![0; TOTAL_ITEMS];
    let before2 = Instant::now();
    my_vec
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number+=index+1);
    println!("rayon: {}ms {:?}", before2.elapsed().as_millis(), &my_vec[5000..5005]);
}

fn serde() {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: u8,
        y: u8
    }
    let point = Point{x: 1, y:2};
    println!("serde_json::to_string {}", serde_json::to_string(&point).unwrap());

}

async fn future_blocks() {
    use std::time::Duration;
    let future_1 = async {
        std::thread::sleep(Duration::from_millis(15));
        println!("async from 1")
    };

    let future_2 = async {
        std::thread::sleep(Duration::from_millis(5));
        println!("async from 2")
    };

    ((), ()) = futures::join!(future_1, future_2)
}

fn futures() {
    let future = future_blocks();
    futures::executor::block_on(future);
}

fn tokio() {
    todo!("implement")
}

fn main() {
    mutability();
    types();
    byte_manipulation();
    references();
    moving_references();
    unicode();
    arrays();
    vectors_and_tuples();
    looping_and_matching();
    structs_enums_and_impl();
    option_and_handling();
    result_and_handling();
    vecdeq();
    traitbounds();
    chaining();
    iterators();
    closures();
    functional();
    some_and_find();
    cycle();
    debug_logging();
    lifetimes();
    cell_refcel();
    mutex();
    multithreading_naive();
    multithreading();
    channels();
    big_multithreading();
    box_heap();
    dyn_trait();
    dereferencing();
    mods();
    rayon();
    serde();
    futures();
    tokio();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn types_returns_char() {
        let result = types();
        assert_eq!(result, '=');
    }

    #[test]
    fn mutability_returns_integer() {
        let result = mutability();
        assert_eq!(result, 1_000_035);
    }

    #[test]
    fn string_factory_returns_string() {
        let result = string_factory();
        assert_eq!(result.len(), 48);
    }
}
