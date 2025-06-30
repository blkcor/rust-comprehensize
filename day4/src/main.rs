pub mod shapes;
pub mod tests;
pub mod utils;

use shapes::rect::Rectangle;
struct SliceIterator<'s> {
    slice: &'s [i32],
    i: usize,
}

impl<'s> Iterator for SliceIterator<'s> {
    type Item = &'s i32;
    fn next(&mut self) -> Option<Self::Item> {
        // reach the end of the slice return None
        if self.i == self.slice.len() {
            None
        } else {
            // return the element of the index
            let element = &self.slice[self.i];
            self.i += 1;
            Some(element)
        }
    }
}

struct Grid {
    x_records: Vec<u32>,
    y_records: Vec<u32>,
}

impl IntoIterator for Grid {
    // Item: the type iterator over
    type Item = (u32, u32);
    // IntoIter: the type of the iterator
    type IntoIter = GridIter;
    // IntoIter and Item are linked: the iterator must have the same Item type, which means that it returns Option<Item>
    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            x_index: 0,
            y_index: 0,
        }
    }
}

struct GridIter {
    grid: Grid,
    x_index: usize,
    y_index: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x_index >= self.grid.x_records.len() {
            self.x_index = 0;
            self.y_index += 1;
            if self.y_index >= self.grid.y_records.len() {
                return None;
            }
        }
        let res = Some((
            self.grid.x_records[self.x_index],
            self.grid.y_records[self.y_index],
        ));
        self.x_index += 1;
        res
    }
}

fn main() {
    // =============== Iterators ===============
    // 1、Motivation
    // If you want to iterate over an array, you will need to define:
    // - some state to keep track of the current position in the array, such as an index.
    // - a condition to determine if the iteration is done.
    // - logic for updating the state of iteration each loop.
    // - logic for retrieving each element for each loop.

    // In other languages, you will do it like this:
    // int[] array = {1, 2, 3, 4, 5};
    // int index = 0;
    // while (index < array.length) {
    //     int element = array[index];
    //     index++;
    // }

    // In Rust, we bundle the logic and the state into an object called iterator.
    // Rust do not has the same c-style for loop, but we can use while loop to implement it.
    let vec = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    while i < vec.len() {
        println!("{}", vec[i]);
        i += 1;
    }

    // 2、Iterator trait
    // The Iterator trait defined how an object can process a sequence of values.
    let slice = &[2, 4, 6, 8];
    let iter = SliceIterator { slice, i: 0 };
    for elem in iter {
        dbg!(elem);
    }

    // 3、Iterator helper methods
    // Iterator trait provides 70+ helper methods that can be used to build customized iterators.
    let result: i32 = (0..10).filter(|x| x % 2 == 0).map(|x| x * x).sum();
    dbg!(result);
    // The Iterator tarit implements many common functional programming operations over collections.
    // eg: filter, map, fold, sum, etc.
    // Many of these helper methods take the original iterator and produce a new iterator with different behavior.
    // Some methods, like sum and count, consume the iterator and pull all of the elements out of it.
    // These methods are designed to be chained together so that it's easy to build a custom iterator that does exactly what you need.

    // 4、collect
    // collet let you create a collcetion from an iterator.
    let primes = vec![2, 3, 5, 7, 11, 13];
    let prime_squares = primes.into_iter().map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", prime_squares);
    // Any iterator can be collected into a Vec | VecDeque | HashSet
    // Iterator that can produce key value pairs can be collected into HashMap | BTreeMap
    // These collections implement the FromIterator trait, that is why they can work like this.

    // There are two ways to specify the generic type of the collection:
    // some_iterator.collect::<COLLECTION_TYPE>(), as shown. The _ shorthand used here lets Rust infer the type of the Vec element
    // With the type inference: let prime_squares: Vec<_> = some_iterator.collect()

    // 5、IntoIterator trait
    // The Iterator trait tells you how to iterate over a type.
    // The relatied trait IntoIterator tells you how to convert a type into an iterator.
    // It is used automatically in for loops.
    let grid = Grid {
        x_records: vec![3, 5, 7, 9],
        y_records: vec![10, 20, 30, 40],
    };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
    // =============== Modules ===============
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Area: {}", rect.area());
    // =============== Tests ===============
}
