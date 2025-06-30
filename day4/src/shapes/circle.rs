use super::internal::helper;
use super::rect::Rectangle;

pub fn compare_area(circle_radius: f64, rect: &Rectangle) -> bool {
    helper();
    std::f64::consts::PI * circle_radius.powi(2) > rect.area()
}
