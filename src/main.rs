use std::collections::VecDeque;
use std::fmt::*;
use std::mem;

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

fn references() -> String {
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

/*
fn result_and_handling() {
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Result::Err("division by zero".to_string())
        } else {
            Result::Ok(numerator / denominator)
        }
    }

    match divide(4, 0) {
        Ok(_) => println!("it was ok"),
        Err(e) => println!("it was an error {}", e),
    }
}
*/

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

    let num_vec = vec![2,4,6];
    let double_vec = &num_vec
        .iter()
        .map(|x| x*2)
        .collect::<Vec<i32>>();

    println!("mapped vector{:?}", double_vec);

    num_vec
        .iter()
        .enumerate()
        .for_each(|(index, value)| { 
            println!("{}:{}",index,value); 
        })
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
            Some(
                Self {
                    measurement: parts[1],
                    date: parts[0],
                    value: value.unwrap()
                }
            )
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
    println!("{:?}", measurements)


}

fn main() {
    mutability();
    types();
    byte_manipulation();
    references();
    unicode();
    arrays();
    vectors_and_tuples();
    looping_and_matching();
    structs_enums_and_impl();
    option_and_handling();
    //result_and_handling();
    vecdeq();
    traitbounds();
    chaining();
    iterators();
    closures();
    functional();
}
