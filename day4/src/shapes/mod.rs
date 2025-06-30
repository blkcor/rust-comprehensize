pub mod circle;
pub mod rect;

// add some public functions here.
pub fn describe() {
    println!("This is a shapes module");
}

mod internal {
    pub(crate) fn helper() {
        println!("This is a helper function");
    }
}
