use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug)]
struct Point(i32, i32);

fn say_hello(name: String) {
    println!("Hello, {}!", name);
}

#[derive(Debug)]
struct WaitToDrop(i32);

impl Drop for WaitToDrop {
    fn drop(&mut self) {
        println!("Dropping WaitToDrop with value {}", self.0);
    }
}

#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// A representation of a software package
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: String::new(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        })
    }

    /// Set the package version.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Set the package authors.
    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Add an additional dependency.
    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Set the language. If not set, language defaults to None.
    fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    fn build(self) -> Package {
        self.0
    }
}

#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

struct Dog {
    name: String,
    age: u8,
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

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: SubTree<T>,
    right: SubTree<T>,
}

#[derive(Debug)]
struct SubTree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: SubTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: SubTree(None),
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

impl<T: Ord> SubTree<T> {
    fn insert(&mut self, value: T) {
        match self {
            // if the node is empty, itself is the node
            SubTree(None) => {
                *self = SubTree(Some(Box::new(Node {
                    value,
                    left: SubTree(None),
                    right: SubTree(None),
                })));
            }
            SubTree(Some(node)) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
            }
        }
    }

    fn has(&self, value: &T) -> bool {
        match self {
            SubTree(None) => false,
            SubTree(Some(node)) => {
                if value == &node.value {
                    true
                } else if value < &node.value {
                    node.left.has(value)
                } else {
                    node.right.has(value)
                }
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            SubTree(None) => 0,
            // recursive call to the left and right subtree
            SubTree(Some(node)) => 1 + node.left.len() + node.right.len(),
        }
    }
}

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn cab_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

// equal to: fn find_nearest<'a, 'b>(points: &'a [Point], p: &'b Point) -> &'a Point {
fn find_nearest<'a>(points: &'a [Point], p: &Point) -> &'a Point {
    let mut nearest = &points[0];
    for point in points {
        if cab_distance(point, p) < cab_distance(nearest, p) {
            nearest = point;
        }
    }
    nearest
}

#[derive(Debug)]
enum HighlightColor {
    Pink,
    Yellow,
}

#[derive(Debug)]
struct Highlight<'document> {
    slice: &'document str,
    color: HighlightColor,
}

fn main() {
    // =============== Memory Management ===============
    // 1、Stack and Heap
    // Program allocate memory in two ways:
    // - Stack: contiguous area of memory for local variables
    //          - Values have fixed sizes known at compile time.
    //          - Extremely fast: just move the stack pointer
    //          - Easy to manage: follow the function calls
    //          - Great memory locality: the cache hit rate is high for the variables in the memory is store adjacent to each other.
    // - Heap: Storage of values outside of function calls.
    //          - Values have dynamic sizes known at runtime.
    //          - Slightly slower than the stack: some book-keeping needed.
    //          - No guarantee of memory locality.

    // 2、The Approaches to Memory Management
    // Traditional, languages have fallen into two broad categories:
    // - Full control via manual memory management: C, C++, Pascal
    // - Full safety via automatic memory management at runtime: Java, Python, Go, Haskell,

    // Rust offers a new mix:
    // Full control and safety via compile time enforcement of correct memory management.

    // 3、Ownership
    // All variables binding has a scope where they are valid and it is an error to use a variable outside of its scope.
    {
        let p = Point(1, 2);
        println!("p.x: {}", p.0);
        // at the end of the scope, the variable p will be dropped and the value will be freed
    }
    // println!("p.x: {}", p.0); // error: borrow checker error

    // 4、Move Semantics
    // An assignment will move the ownership between variables.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // error: borrow checker error
    // The assignment of s1 to s2 transfers the ownership
    // When s1 move out of the scope, nothing happens because s1 is not the owner of the value.
    // When s2 move out of the scope, the value will be freed.

    // When you pass a variable to a function, the value is assigned to the function parametes
    // This transfer the ownership
    say_hello(s2);
    // say_hello(s2); // error: use of moved value: `s2`

    // The behavior of rust is different from c++, which copies by value unless you use `std::move`(and the move constructor is defined)
    // In the top level, is is just the movement of the ownership, rust compiler will mark the old variable is invalid and the new variable is valid.
    // In the bottom level, the complier will optimize the generated machine code, such as just let the new variable use the old variable's memory addree
    // rather than copy the value in the memory.

    // In rust, clone is explicitly use `clone` method(Copy trait provided)

    // 5、Clone
    // Sometimes you want to get the copy of a value. Clone trait accomplishes it.
    let s3 = String::from("hello from v3");
    let s4 = s3.clone();
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    // 1、The idea of Clone is to mark the happen of  allocation of the heap memory.
    // rust's pholosophy is to make the memory spend as explicit as possible. so there is no implicit allocation of the heap memory(copy constructor for c++).
    // with searching `.clone` in our code, we can position the memory allocation quickly.
    // 2、The default behavior of Clone is deep copy.
    // but we can customize it by implementing the Clone trait manually.

    // 6、Copy
    // for the heap allocated value, the default behavior is move semantics.
    // but for the stack allocated value, the default behavior is copy semantics.
    let x = 1;
    let y = x;
    dbg!(x);
    dbg!(y);

    // 7、Drop
    // Value which implements the `Drop` trait can execute some code when they are out of the scope.
    {
        let w = WaitToDrop(1);
        println!("w is created: {:?}", w);
    }
    println!("w is out of scope");
    // Why doesn’t Drop::drop take self?
    // If it did, std::mem::drop would be called at the end of the block, resulting in another call to Drop::drop, and a stack overflow!

    // 8、exercise： Builder Type
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    dbg!(&base64);
    let log = PackageBuilder::new("log")
        .version("0.4")
        .language(Language::Rust)
        .build();
    dbg!(&log);
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .language(Language::Rust)
        .build();
    dbg!(serde);

    // =============== Smart Pointers ===============
    // 9、Box<T>
    // Box is a owned pointer to a heap allocated value
    let five = Box::new(5);
    // Rust will dereference the pointer and get the value automatically
    // Box<T> implements Deref<Target = T>, which means that you can call methods from T directly on a Box<T>.
    println!("five: {}", five);
    // Recusive data type or data type has the dynamic size cannot be stored on the stack.
    // We can do this with Box because the size of the pointer is fixed
    let list: List<i32> = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
    // Box can be very helpful when:
    // - have a type whose size can’t be known at compile time, but the rust complier wants to know the exact size.
    // - want to transfer a large amount of data, but you don’t want to copy it into memory.
    // #[derive(Debug)]
    // struct LargeData {
    //     data: [u8; 1024 * 1024], // 1MB 的数组
    // }

    // If we pass the the array directly, it will be copied the whole 1MB data into the function
    // fn process_data(data: LargeData) {
    //     println!("Processing data: {:?}", data);
    //     // 函数结束时，`data` 和其堆内存会被自动释放
    // }
    // let big_data = LargeData {
    //     data: [0; 1024 * 1024],
    // };
    // // 转移 Box 的所有权到函数，仅移动指针（无数据拷贝）
    // process_data(big_data);

    // 10、Rc
    // Rc is a reference-counted shared pointer
    // Use this when you need to refer to the same data in multiple places.
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    let count = Rc::strong_count(&a);
    println!("strong count a: {}", count);
    let count = Rc::strong_count(&b);
    println!("strong count b: {}", count);

    let c = Rc::weak_count(&a);
    println!("weak count c: {}", c);

    // 11、Owned Trait Objects
    // We previously saw how trait objects can be used with references, e.g &dyn Pet
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog {
            name: String::from("Fido"),
            age: 5,
        }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
    // type that implements the given trait may have different sizes
    // this make it impossible to have thing like Vec<dyn Pet>
    // dyn Pet is a way to tell the compiler about a dynamically sized type that implements Pet.
    // In the example, pets is allocated on the stack and the vector data is on the heap. The two vector elements are fat pointers:
    // - A fat pointer is a double-width pointer It has two components: a pointer to the actual object
    // and a pointer to the virtual method table (vtable) for the Pet implementation of that particular object.
    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    // reference half width to fat pointer
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    // fat pointer
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());

    // 12、exercise: binary tree
    let mut tree = BinaryTree::new();
    assert_eq!(tree.len(), 0);
    tree.insert(2);
    assert_eq!(tree.len(), 1);
    tree.insert(1);
    assert_eq!(tree.len(), 2);
    tree.insert(2); // not a unique item
    assert_eq!(tree.len(), 2);
    tree.insert(3);
    assert_eq!(tree.len(), 3);

    // =============== Borrow Checker ===============
    // 13、Interior Mutability
    // In some cases ,it is vary necessary to modify the value behind the shared reference.
    // For example, a shared data structure might have an internal cache, and wish to update that cache from read-only methods.

    // 14、Cell
    // Cell wraps a value and allow getting and setting the value using only a shared reference.
    // Cell<T> does not return the reference to the inner value, it operates value through copying and move value.
    // So Cell<T> is useful for the value implement the `Copy` trait.(bool, i32, etc)
    let mut ten = Cell::new(10);
    let v = ten.get_mut();
    *v = 20;
    println!("ten: {}", ten.get()); // 20

    // 15、RefCell
    // RefCell allows access and mutate the value by Ref and RefMut that emulates the &T and  &mut T without actually being rust reference.
    // RefCell executes the borrow check at runtime using the inner counter preventing existing of a RefMut alongside with other RefMut or Ref.
    // By implementing Deref (and DerefMut for RefMut), these types allow calling methods on the inner value without allowing references to escape.
    // This make it be used like the normal reference.
    // impl<T> Deref for Ref<T> {
    //     type Target = T;
    //     fn deref(&self) -> &T { /* 返回内部值的真实引用 */
    //     }
    // }

    let ref_cell = RefCell::new(String::from("hello"));
    let borrowed = ref_cell.borrow(); // Ref<String>
    println!("Length: {}", borrowed.len()); // 自动解引用

    // =============== Lifetimes ===============
    // 16、Lifetime Annotation
    // Reference has a lifetime, which must not outlive the value it refers to.
    // This is verified by the borrow checker.
    // Lifetimes can be implicit, this is  what we have seen so far.
    // But lifetime could also be explicit: &'a T
    // 'a T is a borrowed T which is valid for at least the lifetime 'a
    // It is the ownership rather than lifetime annotation that control the existence of the reference.
    // And lifetime annotation is just the mark of static analysis, and it will not effect the actual life time of the reference.
    let p1 = Point(1, 2);
    let p2 = Point(3, 4);
    let p = left_most(&p1, &p2);
    println!("p: {:?}", p);
    // what is the lifetime of the p?
    // i think: lifetime(p) = min(lifetime(p1), lifetime(p2))

    // 17、Lifetime Elision
    // The borrow checker is a static analysis tool, it will not effect the actual life time of the reference.
    // So the lifetime elision is a rule that the borrow checker will use to infer the lifetime of the reference.
    // The rule is:
    // 1、Each parameter that is a reference gets its own lifetime parameter.
    // 2、If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    // 3、If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.
    // 4、Otherwise, the lifetime of the input lifetimes will be the same as the lifetime of the output lifetimes.
    let points = vec![Point(1, 2), Point(3, 4), Point(5, 6)];
    let p = Point(2, 3);
    let nearest = find_nearest(&points, &p);
    println!("nearest: {:?}", nearest);

    // 18、Lifetime in Data Structures
    // If a data structure contains references, it must be annotated with a lifetime:
    let doc = String::from("The quick brown fox jumps over the lazy dog.");
    let noun = Highlight {
        slice: &doc[16..19],
        color: HighlightColor::Yellow,
    };
    let verb = Highlight {
        slice: &doc[20..25],
        color: HighlightColor::Pink,
    };
    // drop(doc);
    dbg!(noun);
    dbg!(verb);
    // In the above example, the annotation on Highlight enforces that the data underlying the contained &str lives at least as long as any instance of Highlight that uses that data. A struct cannot live longer than the data it references.
    // If doc is dropped before the end of the lifetime of noun or verb, the borrow checker throws an error.
}
