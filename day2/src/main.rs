use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result, Write};

use std::net::IpAddr;
use std::{fmt::Debug, time::Duration};
struct Foo {
    x: (u32, u32),
    y: u32,
}

fn sleep_for(secs: f32) {
    if let Ok(duration) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

// fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
//     // let s = if let Some(s) = maybe_string {
//     //     s
//     // } else {
//     //     return Err("No string provided".to_string());
//     // };
//     // let first_byte_char = if let Some(first_char) = s.chars().next() {
//     //     first_char
//     // } else {
//     //     return Err("String is empty".to_string());
//     // };
//     // let digit = if let Some(digit) = first_byte_char.to_digit(16) {
//     //     digit
//     // } else {
//     //     return Err(format!("Invalid hex character: {first_byte_char}"));
//     // };
//     // Ok(digit)
//     // below version is more concise and idiomatic use of `let else`
//     let Some(s) = maybe_string else {
//         return Err("No string provided".to_string());
//     };
//     let Some(first_byte_char) = s.chars().next() else {
//         return Err("String is empty".to_string());
//     };
//     let Some(digit) = first_byte_char.to_digit(16) else {
//         return Err(format!("Invalid hex character: {first_byte_char}"));
//     };
//     Ok(digit)
// }

fn main() {
    // ============= Pattern Match =============
    // 1、Inrrefutable Patterns
    fn take_tuple(tuple: (i32, u32, String)) {
        // let x = tuple.0;
        // let y = tuple.1;
        // let z = tuple.2;
        // println!("x: {}, y: {}, z: {}", x, y, z);

        // Using pattern matching to destructure the tuple
        // let (x, y, z) = tuple;
        // println!("x: {}, y: {}, z: {}", x, y, z);
        // we can also ignore some values
        let (_, y, z) = tuple;
        println!("y: {}, z: {}", y, z);
    }

    take_tuple((1, 2, "hello".to_string()));
    // .. allows you to ignore multiple values at once.
    // the pattern work with array as well

    // 2、Matching Values
    // the match keyword let u match a value against one or more patterns
    // the pattern can be simple values, like switch in ohter languages
    // but in rust, match can be more powerful which can express more complex conditions
    let input = 'x';
    match input {
        'q' => println!("Quit"),
        'a' | 's' | 'w' | 'd' => println!("Move"),
        '0'..='9' => println!("Number"),
        // a variable in the pattern can create a binding that can be used in the match arm, we will talk about this later
        // there is a match guard which will be matched only if the condition is true
        // this is not the same as the if statement in the =>, witch represents the arm is matched already
        key if key.is_lowercase() => println!("Lowercase letter: {}", key),
        // wildcard pattern
        _ => println!("Other input"),
    }
    // @ syntax allows you to create a binding in the pattern
    let opt = Some(123);
    match opt {
        outer @ Some(inner) => {
            println!("outer: {outer:?}, inner: {inner}");
        }
        None => {}
    }

    // 3、Destructuring Structs
    // let foo = Foo { x: (1, 2), y: 3 };
    // match foo {
    //     Foo { y: 2, x: i } => println!("y = 2, x = {i:?}"),
    //     Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
    //     Foo { y, .. } => println!("y = {y}, other fields were ignored"),
    // }

    // 4、Destructuring Enums
    enum MyEnum {
        Variant1(u32),
        Variant2 { x: u32, y: u32 },
        Variant3,
    }
    let my_enum = MyEnum::Variant2 { x: 1, y: 2 };
    match my_enum {
        MyEnum::Variant1(value) => println!("Variant1 with value: {}", value),
        MyEnum::Variant2 { x, y } => println!("Variant2 with x: {}, y: {}", x, y),
        MyEnum::Variant3 => println!("Variant3"),
    }
    // rust has a few control flow constructs that are different from other languages, and they are using for pattern matching
    // - if let
    // - while let
    // - let else

    //  ==================let control flow=================

    // 5、if let
    // if let allows you to execute different code depending on whether a pattern matched
    sleep_for(-10.0);
    sleep_for(0.8);
    // Unlike match, if let does not have to cover all branches. This can make it more concise than match.
    // Unlike match, if let does not support guard clauses for pattern matching.

    // 6、while let
    // Like if let, while let allows you repeatedly test a value against a pattern.
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(ch) = name.pop() {
        dbg!(ch);
    }
    // the code above is as same as below
    let mut name = String::from("Comprehensive Rust 🦀");
    loop {
        match name.pop() {
            Some(ch) => dbg!(ch),
            None => break,
        };
    }

    // 7、let else
    // For the common case of matching a pattern and returning from the function, use let else.
    // The else case must be diverge(return break or panic -- anything but failling off the end of the block)
    // hex_or_die_trying(Some("1".to_string()))
    //     // success path transformer
    //     .map(|digit| println!("First hex digit: {digit}"))
    //     // error path transformer
    //     .unwrap_or_else(|err| {
    //         eprintln!("Error: {err}");
    //         std::process::exit(1);
    //     });

    // ============= Methods and Traits =============
    // 8、Methods
    // rust allow you to associate function with newtype, do it with impl block(called methods)
    #[derive(Debug)]
    struct CarRace {
        name: String,
        laps: Vec<i32>,
    }

    impl CarRace {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                laps: Vec::new(),
            }
        }
        fn add_lap(&mut self, lap: i32) {
            self.laps.push(lap);
        }

        fn print_laps(&self) {
            println!("Recorded {} laps for {}:", self.laps.len(), self.name);
            for (idx, lap) in self.laps.iter().enumerate() {
                println!("Lap {idx}: {lap} sec");
            }
        }

        fn finish(self) {
            let total: i32 = self.laps.iter().sum();
            println!("Race {} is finished, total lap time: {}", self.name, total);
        }
    }

    let mut race = CarRace::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

    // 9、Traits
    // rust let us abstract over types with traits, like interfaces in other languages
    // trait Pet {
    //     fn talk(&self) -> String;

    //     // default methods can be defined in traits
    //     fn greet(&self) -> () {
    //         println!("Oh you're a cutie! What's your name? {}", self.talk());
    //     }
    // }
    // struct Dog {
    //     name: String,
    //     age: i8,
    // }
    // // impl trait for type
    // impl Pet for Dog {
    //     fn talk(&self) -> String {
    //         format!(
    //             "Woof! My name is {} and I'm {} years old.",
    //             self.name, self.age
    //         )
    //     }
    // }
    // let fido = Dog {
    //     name: String::from("Fido"),
    //     age: 5,
    // };
    // fido.greet();
    // multiple impl block is allowed
    // we can associate some methods with the type and we can also implement the trait for the type

    // 11、supertraits
    // a trait can require the type implementing it to also implement another trait, this is called supertraits
    // here "Animal" is the supertrait of "AnotherPet"
    // any types must implement Animal trait while they want to implement AnotherPet trait
    // trait Animal {
    //     fn let_count(&self) -> u32;
    // }

    // trait AnotherPet: Animal {
    //     fn name(&self) -> String;
    // }

    // struct Cat(String);

    // impl AnotherPet for Cat {
    //     fn name(&self) -> String {
    //         self.0.clone()
    //     }
    // }

    // impl Animal for Cat {
    //     fn let_count(&self) -> u32 {
    //         4
    //     }
    // }

    // let cat = Cat("Whiskers".to_string());
    // println!("cat named {} has {} legs", cat.name(), cat.let_count());

    // 12、Associated Types
    // The assiciated types is a placeholder types which are supplied by the trait implementation

    #[derive(Debug)]
    struct Meters(i32);
    #[derive(Debug)]
    struct MetersSquared(i32);

    trait Multiply {
        // this is an associated type
        type Output;
        // &self is the syntax sugar for self: &Self
        // self is the value and Self is the type of the value
        fn multiply(&self, other: &Self) -> Self::Output;
    }

    impl Multiply for Meters {
        type Output = MetersSquared;

        fn multiply(&self, other: &Self) -> Self::Output {
            MetersSquared(self.0 * other.0)
        }
    }

    let m1 = Meters(3);
    let m2 = Meters(4);
    let result = m1.multiply(&m2);
    println!(
        "Result of multiplying {} and {} is {:?}",
        m1.0, m2.0, result.0
    );
    // for associated types, the key one is the implementor but the caller who chooses the "output type"
    // Many standard library traits have associated types, including arithmetic operators and Iterator.

    // 13、Derived Traits
    // Rust provides a number of traits that can be derived automatically using the #[derive] attribute.
    // here is an example:
    #[derive(Debug, Default)]
    struct Player {
        name: String,
        strength: u8,
        hit_points: u8,
    }
    // Default trait allows you to create a default value for the type using Type::default()
    // let p1 = Player::default();
    // // Clone trait allows you to create a copy of the value using value.clone()
    // let mut p2 = p1.clone();
    // p2.name = "Hero".to_string();
    // println!("Player 1: {:?}", p1);
    // println!("Player 2: {:?}", p2);

    // derivation is implemented by macros, and many crates provide useful derive macros to add useful functionality.
    // For example, `serde` can derive serialization and deserialization support for a struct using #[derive(Serialize, Deserialize)].
    // Derivation is usually provided for traits that have a common boilerplate-y implementation that is correct for most cases.
    // Here is an example to impl `Clone` trait manually:
    impl Clone for Player {
        fn clone(&self) -> Self {
            Player {
                name: self.name.clone(),
                strength: self.strength,
                hit_points: self.hit_points,
            }
        }
    }
    let p1 = Player {
        name: "Player1".to_string(),
        strength: 10,
        hit_points: 100,
    };
    let p2 = p1.clone();
    println!("Player 1: {:?}", p1);
    println!("Player 2: {:?}", p2);

    // 14、exercise: Generic Logger
    // trait Logger {
    //     fn log(&self, verbosity: u8, message: &str);
    // }
    // struct StdErrorLogger;

    // impl Logger for StdErrorLogger {
    //     fn log(&self, verbosity: u8, message: &str) {
    //         eprintln!("verbosity={verbosity}: {message}");
    //     }
    // }

    // struct VerbosityFilter {
    //     max_verbosity: u8,
    //     inner: StdErrorLogger,
    // }

    // impl Logger for VerbosityFilter {
    //     fn log(&self, verbosity: u8, message: &str) {
    //         if verbosity <= self.max_verbosity {
    //             self.inner.log(verbosity, message);
    //         }
    //     }
    // }

    // let logger = VerbosityFilter {
    //     max_verbosity: 3,
    //     inner: StdErrorLogger,
    // };
    // logger.log(5, "FYI");
    // logger.log(2, "Uhoh");

    // 15、Generics
    // Rust supports generics, which allow you to abstract over types in algorithms or data structures.
    fn pick<T>(condition: bool, left: T, right: T) -> T {
        if condition { left } else { right }
    }

    println!("picked a number: {:?}", pick(true, 222, 333));
    println!("picked a string: {:?}", pick(false, 'L', 'R'));
    // Rust infers a type for T based on the types of the arguments and return value.
    // Rust compiler will transfer the generic code to non-generic code according to the details of the usage.
    // This process is called monomorphization.

    // 16、Generic Bounds
    // When working with generics, you always want the type to implement some traits, so that you can call the methods on it.
    fn duplicate<T: Clone>(a: T) -> (T, T) {
        (a.clone(), a.clone())
    }

    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    // When multiple traits are necessary, use + to join them.
    // fn duplicate1<T: Clone + Debug>(a: T) -> (T, T) {
    //     (a.clone(), a.clone())
    // }

    // or you can also use where clause to specify the bounds.
    // fn dumplicate2<T>(a: T) -> (T, T)
    // where
    //     T: Clone + Debug,
    // {
    //     (a.clone(), a.clone())
    // }

    // 17、Generic Data Type
    // You can use generics to abstract the concrate field type.
    // trait Logger {
    //     fn log(&self, verbosity: u8, message: &str);
    // }
    // struct StdErrorLogger;

    // impl Logger for StdErrorLogger {
    //     fn log(&self, verbosity: u8, message: &str) {
    //         eprintln!("verbosity={verbosity}: {message}");
    //     }
    // }

    // struct VerbosityFilter<L: Logger> {
    //     max_verbosity: u8,
    //     inner: L,
    // }

    // // <L :Logger> is the generic declaration block, introducing a type L and it's trait bound .
    // // VerbosityFilter<L> means this is the impl of `Logger`` for the detailed type `VerbosityFilter<L>`.
    // impl<L: Logger> Logger for VerbosityFilter<L> {
    //     fn log(&self, verbosity: u8, message: &str) {
    //         if verbosity <= self.max_verbosity {
    //             self.inner.log(verbosity, message);
    //         }
    //     }
    // }

    // let logger = VerbosityFilter {
    //     max_verbosity: 3,
    //     inner: StdErrorLogger,
    // };
    // logger.log(5, "FYI");
    // logger.log(2, "Uhoh");

    // 18、Generic Trait
    // Traits can also be generic, just like types and functions.
    // The parameters of the trait can get concrete when it is used.
    // For example: From<T> trait is used tto defined type conversion.

    #[derive(Debug)]
    struct Foo(String);

    // defined how to convert from u32 to Foo
    impl From<u32> for Foo {
        fn from(value: u32) -> Self {
            Foo(format!("Converted from integer: {value}"))
        }
    }

    impl From<bool> for Foo {
        fn from(from: bool) -> Foo {
            Foo(format!("Converted from bool: {from}"))
        }
    }

    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    dbg!(from_int.0);
    dbg!(from_bool.0);
    // Generic traits take types as “input”, while associated types are a kind of “output” type. A trait can have multiple implementations for different input types.
    trait Greet {
        fn greet(&self);
    }

    // this will implement the Greet trait for all types that implement Display trait
    impl<T: std::fmt::Display> Greet for T {
        fn greet(&self) {
            println!("Hello, {}!", self);
        }
    }
    // Here String has already implemented Display trait, so here is a conflict.
    // impl Greet for String {
    //     fn greet(&self) {
    //         println!("Special greeting for String!");
    //     }
    // }
    // **Designed like this can eliminate the implicit choic of which implementation to use. And make sure the code is predictable and clear.**

    // 19、Impl trait
    // Similar to trait bound, impl trait can be used in parameters and return value
    fn add_42_million(x: impl Into<i32>) -> i32 {
        x.into() + 42_000_000
    }
    // impl trait allows you work with types that you can't named, but the meaning of `impl Trait` is different for parameters and return values.
    // - for parameters, it looks like a generic type with trait bounds, here is the example:
    // fn add_42_million2<T: Into<i32>>(x: T) -> i32 {
    //     x.into() + 42_000_000
    // }

    // - for return values, it means the function will return a type that implements the trait, but you don't need to specify the type.
    // this can be useful when you don't want to expose the concrete type in public API

    fn pair_of(x: u32) -> impl std::fmt::Debug {
        (x + 1, x - 1)
    }
    let many = add_42_million(42_i8);
    dbg!(many);
    let debuggable = pair_of(27);
    dbg!(debuggable);

    // Inferences in return position is difficult. A function returning `impl trait` picks the concrete type it returns based on the function body.
    // We have no need to spicify the concrete type.
    // Unlike generic function such as `fn collect<T>() -> T`, the caller must choose the concrete type for `T` when calling the function.
    // But in impl trait, it is not our work.

    // 20、dyn Trait
    struct Dog {
        name: String,
        age: i8,
    }
    struct Cat {
        lives: i8,
    }

    trait Pet {
        fn talk(&self) -> String;
    }

    impl Pet for Dog {
        fn talk(&self) -> String {
            format!("Woof, my name is {}!", self.name)
        }
    }

    impl Pet for Cat {
        fn talk(&self) -> String {
            String::from("Miau!")
        }
    }

    // Uses generics and static dispatch.
    fn generic(pet: &impl Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }

    // Uses type-erasure and dynamic dispatch.
    fn dynamic(pet: &dyn Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }

    let cat = Cat { lives: 9 };
    let dog = Dog {
        name: String::from("Fido"),
        age: 5,
    };

    generic(&cat);
    generic(&dog);

    dynamic(&cat);
    dynamic(&dog);
    // Generics, including impl Trait, use monomorphization to create a specialized instance of the function for each different type that the generic is instantiated with.
    // such as: fn foo<T>(x: T) when we calling the function with i32 and f64, the compiler will generate two versions of the function: `foo_i32` and `foo_f64`.
    // and with the mode, there is no runtime overhead, because the compiler knows the concrete type at compile time, which is also called static dispath.

    // When we use `dyn trait` it instead uses dynamic dispatch through a vtable.(which is also called trait object)
    // This means that there’s a single version of fn dynamic that is used regardless of what type of Pet is passed in.
    // When using `dyn trait`, the trait object must be behind a pointer type, such as `Box<dyn Pet>`, `&dyn Pet`, or `Rc<dyn Pet>`.
    // ** A dyn Trait is considered to be “type-erased”, because we no longer have compile-time knowledge of what the concrete type is. **

    // ============== Standard Library Types =============
    // Rust comes with a standard library which helps establish a set of common types used by Rust libraries and programs.
    // Standard library helps us to work together smoothly because we use the same String type.
    // Rust contains serveral layers of standard library: `core`, `alloc`, `std`
    // - `core`: includes the most basic types and functions that don’t depend on libc, allocator or even the presence of an operating system.
    // - `alloc`: includes types which require a global heap allocator, such as Vec, Box and Arc.
    // - `std`: includes all the types in `core` and `alloc`, plus additional types that require an operating system, such as threads, file I/O and networking.

    // 21、Option
    // Option is a type that represents there is a value typed T or not.
    // - Some(T) represents a value of type T
    // - None represents the absence of a value
    // For example: `String::find` retunrs Option<usize>
    let name = "Löwe 老虎 Léopard Gepardi";
    let position: Option<usize> = name.find('é');
    dbg!(position);
    match position {
        Some(position) => println!("Found 'é' at position: {position}"),
        None => println!("'é' not found"),
    }
    // unwrap returns the value inside Some<T>, or panic if it is None
    assert_eq!(position.unwrap(), 14);
    // position = name.find("Z");
    // expect is similar to unwrap, but it can take a error message
    // assert_eq!(position.expect("Character not found"), 0);
    // the nice optimization is that Option<T> has the same size as T in-memory.
    // to emphasize: this is the zero cost abstraction.

    // 22、Result
    // Result is similar to Option, but indicate the successful or failure of an operation.
    // - Ok(T) represents a successful operation
    // - Err(E) represents a failure operation
    // For example: `File::open` returns Result<File, std::io::Error>
    // let file: Result<File, std::io::Error> = File::open("diary.txt");
    // As with Option, the successful result is in OK<T>, forcing the developer to extract it. This encourages for error checking.
    // In the case where the error is never happen, we could use `unwrap` to handle the successful result.

    // match file {
    //     Ok(mut file) => {
    //         let mut contents = String::new();
    //         if let Ok(bytes) = file.read_to_string(&mut contents) {
    //             println!("Dear diary: {contents} ({bytes} bytes)");
    //         } else {
    //             println!("Could not read file content");
    //         }
    //     }
    //     Err(err) => {
    //         println!("The diary could not be opened: {err}");
    //     }
    // }
    // Result is the standard type to implement error handling as we will see later.

    // 23、String
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    s1.push_str(" World");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    // 24、Vec
    // Vec is the standard resizable heap-allocated buffer:
    // Through Vec::new to create a empty vector
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    // Through Vec::with_capacity to create a vector with a given capacity
    let mut v2 = Vec::with_capacity(v1.len() + 1);
    // extend is a method that takes an iterator and appends all of its elements to the vector.
    v2.extend(v1.iter());
    v2.push(9099);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // Through vec! macro to create a vector
    let mut v3 = vec![0, 0, 1, 2, 3, 4];
    // retain the only even number
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
    // Vec it also the generic type too, but we have no need to specify it as it was inferred at the first push operation.
    // To index the vector you maybe use [], but they will panic if the index is out of bounds.
    // So that use `get` method instead as it will return an Option<&T>
    // The `pop` will remove the last element

    // 25、HashMap
    // Standard hash map with protection against HashDoS attacks
    // hashmap is a k v store, and it is a generic type
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn", 207);
    page_counts.insert("Grimms' Fairy Tales", 751);
    page_counts.insert("Pride and Prejudice", 303);
    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        // use entry method to get the entry of the map with the key
        // if there is no the specific key, it will insert a new key with the default value
        let page_count: &mut i32 = page_counts.entry(book).or_insert(0);
        *page_count += 1;
    }
    dbg!(page_counts);

    // HashMap is a generic type, so we can use it to store any type of key and value.
    // The key must implement the `Hash` and `Eq` traits.
    // The value must implement the `Debug` trait.
    // Alternatively, HashMap can be built from any `Iterator` of key-value pairs.
    let page_counts2 = HashMap::from([
        ("Adventures of Huckleberry Finn", 207),
        ("Grimms' Fairy Tales", 751),
        ("Pride and Prejudice", 303),
    ]);
    dbg!(page_counts2);

    // ============== Standard Library Traits =============
    // 26、Comparisons
    // These traits support comparing between values.
    // All traits can be derived for the types that all the field implement the trait.

    // - PartialEq and Eq
    // PartialEq is a partial equivalence relation, with required implementation of `eq` method and `ne` method.
    // which is required when use `==` and `!=` operator.

    #[derive(Debug)]
    struct Key {
        id: u32,
        metadata: Option<String>,
    }

    impl PartialEq for Key {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
        fn ne(&self, other: &Self) -> bool {
            self.id != other.id
        }
    }

    // And Eq is the full equivalence relation, and implies `PartialEq`
    // which means Eq is the subset of PartialEq.(trait Eq: PartialEq)
    // Functions that require full equivalence will use `Eq` trait as it's tarit bound.

    // - PartialOrd and Ord
    // PartialOrd defines a partial order relation with a `partial_cmp` method.
    // which is required when use `<=`, `>=`, `<`, `>` operator.
    #[derive(PartialEq, Eq)]
    struct Citation {
        author: String,
        year: u16,
    }

    impl PartialOrd for Citation {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.author.partial_cmp(&other.author) {
                Some(std::cmp::Ordering::Equal) => self.year.partial_cmp(&other.year),
                author_ord => author_ord,
            }
        }
    }

    let c1 = Citation {
        author: "Alice".to_string(),
        year: 2020,
    };
    let c2 = Citation {
        author: "Bob".to_string(),
        year: 2021,
    };

    println!("c1 <= c2: {}", c1 <= c2);

    // 27、Operator
    // Operator overloading is implemented via traits `sdt::ops`
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: u32,
        u: u32,
    }

    impl std::ops::Add for Point {
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                u: self.u + rhs.u,
            }
        }
    }

    let pp1 = Point { x: 1, u: 2 };
    let pp2 = Point { x: 3, u: 4 };
    let p3 = pp1 + pp2;
    println!("pp1: {pp1:?}");
    println!("pp2: {pp2:?}");
    println!("p3: {p3:?}");

    // 28、From and Into
    // From and Into are traits that allow for type conversion.
    // Unlike `as`, these traits correspond to lossless, infallible conversions.

    // From trait use to convert value to the caller type
    let s = String::from("Hello World");
    let addr = IpAddr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s}, {addr}, {one}, {bigger}");

    // Into will be automatically implemented while From trait is implemented.

    // 29、Casting
    // Rust doesn't support implicit type conversion, but it supports explicit type conversion by using `as` keyword.
    // These generally follow C semantics where those are defined.
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);

    //Casting with as is a relatively sharp tool that is easy to use incorrectly,
    // and can be a source of subtle bugs as future maintenance work changes the types that are used or the ranges of values in types.

    // 30、Read and Write
    // Use `Read` and `BufRead`, you can abstract over the source of bytes(u8).
    fn read_line_count<R: Read>(reader: R) -> usize {
        let buf_reader = BufReader::new(reader);
        buf_reader.lines().count()
    }
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", read_line_count(slice));
    let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();
    println!("lines in file: {}", read_line_count(file));

    // Similar, Write abstract the sink of bytes(u8).
    fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
        writer.write_all(msg.as_bytes())?;
        writer.write_all("\n".as_bytes())
    }
    let mut buffer = Vec::new();
    log(&mut buffer, "Hello").unwrap();
    log(&mut buffer, "World").unwrap();
    println!("Logged: {buffer:?}");

    // 31、Default
    // The Default trait produce a default value for a type
    #[derive(Debug)]
    struct Implemented(String);

    impl Default for Implemented {
        fn default() -> Self {
            Self("John Smith".into())
        }
    }

    let d1 = Implemented::default();
    println!("d1: {d1:?}");

    // 32、exercies：rot13
    struct RotDecoder<R: Read> {
        input: R,
        rot: u8,
    }

    impl<R: Read> Read for RotDecoder<R> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let size = self.input.read(buf)?;
            for b in &mut buf[..size] {
                if b.is_ascii_alphabetic() {
                    let base = if b.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                    *b = (*b - base + self.rot) % 26 + base;
                }
            }
            Ok(size)
        }
    }
    // The derived implementation will produce a value where all fields are set to their default values.

    // ============== Closures =============
    // 33、Syntax
    // Closure is created by vertical bars |..|
    let double_it = |x| x * 2;
    let res = double_it(2);
    println!("res: {res}");

    // 34、Capturing
    let mut max_value = 5;
    let mut clamp = |v: i32| {
        max_value += 1;
        if v > max_value { max_value } else { v }
    };
    let res = clamp(6);
    println!("res: {res}");

    // 35、Closure trait
    // Closure or lambda expression implement special Fn, FnMut, FnOnce traits.
    // normal fn type: fn(..) -> T refer to function pointer, either the address of the function
    // or a closure that captures nothing.

    // func decleared as `FnOnce` which is the lowest restriction of the passed function
    // as it is required the function can be called once at least.
    fn apply_and_log(
        func: impl FnOnce(&'static str) -> String,
        func_name: &'static str,
        input: &'static str,
    ) {
        println!("Calling {func_name}({input}): {}", func(input))
    }
    let suffix = "-itis";
    // - `Fn`: neither consumes nor mutates captured values
    // it can be called needing the share reference of the closure, which means the closure can be executed repeatedly and even concurrently.
    let add_suffix = |x| format!("{x}{suffix}");
    apply_and_log(&add_suffix, "add_suffix", "senior");
    apply_and_log(&add_suffix, "add_suffix", "appenix");

    let mut v = Vec::new();
    // - `FnMut`: might mutate captured values
    // The closure object is accessed via exclusive reference, so it can be called repeatedly but not concurrently.
    let mut accumulate = |x| {
        v.push(x);
        v.join("/")
    };
    apply_and_log(&mut accumulate, "accumulate", "red");
    apply_and_log(&mut accumulate, "accumulate", "green");
    apply_and_log(&mut accumulate, "accumulate", "blue");

    // - `FnOnce`: consumes captured values
    // you may only call it once. Doing so consumes the closure and any values captured by move.
    let take_and_reverse = |prefix| {
        let mut acc = String::from(prefix);
        acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
        acc
    };
    apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");

    // FnMut is a subtype of FnOnce. Fn is a subtype of FnMut and FnOnce
    // I.e. you can use an FnMut wherever an FnOnce is called for, and you can use an Fn wherever an FnMut or FnOnce is called for.
    // As you can think, FnOnce is the most restrictive trait, and Fn is the least restrictive trait.

    // How can we distinguish what the closure type is? Fn, FnMut, FnOnce?
    // It depends on how to capture the environment and refer from the least restriction.
    // for example:
    // ```rust
    /*
    let suffix = "-itis";
        let add_suffix = |x| format!("{x}{suffix}"); // 捕获 suffix（不可变借用）
        apply_and_log(&add_suffix, "add_suffix", "senior"); // 可多次调用
        apply_and_log(&add_suffix, "add_suffix", "appendix");
     */
    // We just need to read the value of the suffix(share reference), so it is Fn.
    // As it is the same, accumulate will mutate the Vec, so it is FnMut.
    // And take_and_reverse will consume the Vec, so it is FnOnce.

    // 36、exercises: log filter
    trait Logger {
        fn log(&self, verbosity: u8, message: &str);
    }
    struct StderrLogger;
    impl Logger for StderrLogger {
        fn log(&self, verbosity: u8, message: &str) {
            eprintln!("verbosity={verbosity}: {message}");
        }
    }

    struct Filter<L, F>
    where
        L: Logger,
        F: Fn(u8, &str) -> bool,
    {
        inner_logger: L,
        func: F,
    }

    impl<L, F> Filter<L, F>
    where
        L: Logger,
        F: Fn(u8, &str) -> bool,
    {
        fn new(inner_logger: L, func: F) -> Self {
            Self { inner_logger, func }
        }
        fn log(&self, verbosity: u8, message: &str) {
            if (self.func)(verbosity, message) {
                self.inner_logger.log(verbosity, message);
            }
        }
    }

    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
