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
    println!("reference points to large string on heap address: {:p}", reference);
    println!("bytesize of String is {}", large.len());
    println!("bytesize of reference is {}", mem::size_of_val(&large));
    println!("bytes representation of large is {:?}", large.as_bytes());

    // &large // cannot return reference to object that is owned by this function
    large // can return the object, passing the ownership to caller
}

fn references() -> String {
    let mut string_object_created_in_method = string_factory();
    string_object_created_in_method.push('?'); // 
    println!("mutated string {}",string_object_created_in_method);

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
    println!("length of vector: {}",vector_from_array.len());

    let mut vector_of_tuples = vec![("synth", "micromonsta"), ("controller", "hapax"), ("mixer", "bluebox")];   
    println!("capacity of the tuplevector is: {}", vector_of_tuples.capacity());
    vector_of_tuples.push(("controller", "launchpad"));
    println!("after pushing 1, capacity tuplevector is: {}", vector_of_tuples.capacity());

    let (a, b) = vector_of_tuples[0];
    println!("deconstructed tuple into {} {}", a, b);
}

fn looping_and_matching() {
    let vector_of_tuples = vec![("synth", "micromonsta"), ("controller", "hapax"), ("mixer", "bluebox")];   

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

    struct Device {
        device_type: DeviceType,
        power_usage_amps: f32,
        name: String,
        //name: &'static str //static lifetime, this referenced value should live on after this block
    }

    impl Device {
        fn is_high_powered(&mut self) -> bool {
            self.power_usage_amps > 2.0
        }
    }

    use DeviceType::*;
    let device0 = Device {
        device_type: Synth,
        power_usage_amps: 1.5,
        name: "MicroMonsta".to_string(),
    };
    let device1 = Device {
        device_type: Synth,
        power_usage_amps: 0.5,
        name: "MicroMonsta".to_string(),
    };
    let device2 = Device {
        device_type: Controller,
        power_usage_amps: 2.0,
        name: "Hapax".to_string(),
    };
    
    let device3 = Device {
        device_type: Mixer,
        power_usage_amps: 2.0,
        name: "Bluebox".to_string(),
    };

    let vector = vec![device0, device1, device2, device3];
    for mut device in vector {
        match device.is_high_powered() {
            true => println!("low power {:?}, {}", device.device_type, device.name),
            false => println!("high power {:?}, {}", device.device_type, device.name),
        }
    }
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
}
