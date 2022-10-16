//!
//! Custom math functions needed by the game.
//!

use embedded_graphics::prelude::Point;

/// Return true if the given square defined by point `top_left1` and size `size1`
/// intersects the given square defined by point `top_left2` and size `size2`.
/// # Arguments
/// * `top_left1` - the top left point of the first square
/// * `size1` - the size of the first square
/// * `top_left2` - the top left point of the second square
/// * `size2` - the size of the second square
pub fn intersects(top_left1: Point, size1: u32, top_left2: Point, size2: u32) -> bool {
    let size1 = size1 as i32;
    let size2 = size2 as i32;
    intersects1d(
        top_left1.x,
        top_left1.x + size1,
        top_left2.x,
        top_left2.x + size2,
    ) && intersects1d(
        top_left1.y,
        top_left1.y + size1,
        top_left2.y,
        top_left2.y + size2,
    )
}

/// Return true if the given two interval intersect.
/// # Arguments
/// * `min1` - interval 1 min value
/// * `max1` - interval 1 max value
/// * `min2` - interval 2 min value
/// * `max2` - interval 2 max value
fn intersects1d(min1: i32, max1: i32, min2: i32, max2: i32) -> bool {
    contains1d(min1, min2, max2)
        || contains1d(max1, min2, max2)
        || contains1d(min2, min1, max1)
        || contains1d(max2, min1, max1)
}

/// Is the given value contained in the given interval?
/// # Arguments
/// * `x` - the value to check
/// * `min` - interval minimum value
/// * `max` - interval maximum value
fn contains1d(x: i32, min: i32, max: i32) -> bool {
    x >= min && x <= max
}

#[cfg(test)]
mod tests {
    use super::intersects;
    use crate::math::intersects1d;
    use embedded_graphics::prelude::Point;

    #[test]
    fn shapes_on_top_of_each_other_intersects_test() {
        let shape1_point = Point::new(1, 1);
        let shape1_size = 8;
        let shape2_point = Point::new(1, 1);
        let shape2_size = 8;
        assert!(intersects(
            shape1_point,
            shape1_size,
            shape2_point,
            shape2_size
        ))
    }

    #[test]
    fn shapes_intersect_intersects_test() {
        let shape1_point = Point::new(1, 1);
        let shape1_size = 8;
        let shape2_point = Point::new(8, 8);
        let shape2_size = 8;
        assert!(intersects(
            shape1_point,
            shape1_size,
            shape2_point,
            shape2_size
        ))
    }

    #[test]
    fn shapes_touch_on_boundary_intersect_intersects_test() {
        let shape1_point = Point::new(1, 1);
        let shape1_size = 8;
        let shape2_point = Point::new(9, 9);
        let shape2_size = 8;
        assert!(intersects(
            shape1_point,
            shape1_size,
            shape2_point,
            shape2_size
        ))
    }

    #[test]
    fn shapes_do_not_intersect_intersects_test() {
        let shape1_point = Point::new(1, 1);
        let shape1_size = 8;
        let shape2_point = Point::new(10, 10);
        let shape2_size = 8;
        assert!(!intersects(
            shape1_point,
            shape1_size,
            shape2_point,
            shape2_size
        ))
    }

    #[test]
    fn shapes_far_from_each_other_intersects_test() {
        let shape1_point = Point::new(1, 1);
        let shape1_size = 8;
        let shape2_point = Point::new(100, 100);
        let shape2_size = 8;
        assert!(!intersects(
            shape1_point,
            shape1_size,
            shape2_point,
            shape2_size
        ))
    }

    #[test]
    fn intersects1d_test() {
        assert!(!intersects1d(10, 20, -10, 0));
        assert!(!intersects1d(10, 20, 8, 9));
        assert!(intersects1d(10, 20, 8, 10));
        assert!(intersects1d(10, 20, 8, 11));
        assert!(intersects1d(10, 20, 11, 12));
        assert!(intersects1d(10, 20, 10, 20));
        assert!(intersects1d(10, 20, 18, 20));
        assert!(intersects1d(10, 20, 18, 21));
        assert!(intersects1d(10, 20, 20, 21));
        assert!(!intersects1d(10, 20, 21, 22));
        assert!(!intersects1d(10, 20, 30, 40));
        assert!(intersects1d(10, 20, -10, 40));
    }
}
