use std::f64::consts::PI;

use crate::geometry::{Point,Line};

const ORIGIN: Point = Point{x:0.0, y:0.0};
const PRECISION: f64 = 1000.0;

#[cfg(test)]
mod point_tests {
    use super::*; // needed to access crate imports
    #[test]
    fn _point_tests_are_running() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn equality_minute_difference() {
        let base: Point = Point{x:0.0, y:0.0};
        let equal: Point = Point{x:0.00000000000000001, y:-0.0000000000000001};
        assert_eq!(base, equal);
    }
    #[test]
    fn equality_x_different() {
        let base: Point = Point{x:0.0, y:0.0};
        let unequal: Point = Point{x:0.1, y:0.0};
        assert!(base != unequal, "base ({:?}) equals unequal ({:?})", base, unequal);
    }
    #[test]
    fn equality_y_different() {
        let base: Point = Point{x:0.0, y:0.0};
        let unequal: Point = Point{x:0.0, y:-0.1};
        assert!(base != unequal, "base ({:?}) equals unequal ({:?})", base, unequal);
    }
    #[test]
    fn origin_angle_to_zero() {
        let pt = Point{x:1.0, y:0.0};
        let actual = ORIGIN.angle_to(&pt);
        let expected = 0.0;
        assert_eq!(actual, expected);
    }
    #[test]
    fn origin_angle_to_030() {
        let pt = Point{x:0.866, y:0.5};
        let actual = ORIGIN.angle_to(&pt);
        let expected = PI / 6.0;
        assert_eq!((PRECISION * expected).round() as i64,
                   (PRECISION * actual).round() as i64);
    }
    #[test]
    fn origin_angle_to_120() {
        let pt = Point{x:-0.5, y:0.866};
        let angle_to = ORIGIN.angle_to(&pt);
        assert_eq!((PRECISION * angle_to).round() as i64,
                   (PRECISION * 4.0 * PI / 6.0).round() as i64);
    }
    #[test]
    fn origin_angle_to_180() {
        let pt = Point{x:-0.5, y:0.0};
        let angle_to = ORIGIN.angle_to(&pt);
        assert_eq!((PRECISION * angle_to).round() as i64,
                   (PRECISION * PI).round() as i64);
    }
    #[test]
    fn origin_angle_to_240() {
        let pt = Point{x:-0.5, y:-0.866};
        let angle_to = ORIGIN.angle_to(&pt);
        assert_eq!((PRECISION * angle_to).round() as i64,
                   (PRECISION * (-4.0 * PI / 6.0)).round() as i64);
    }
    #[test]
    fn origin_angle_to_300() {
        let pt = Point{x:0.5, y:-0.866};
        let angle_to = ORIGIN.angle_to(&pt);
        assert_eq!((PRECISION * angle_to).round() as i64,
                   (PRECISION * (-2.0 * PI / 6.0)).round() as i64);
    }
    #[test]
    fn rotate_090() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.rotate(&ORIGIN, PI / 2.0);
        let expected = Point{x:0.321, y:0.6};
        assert_eq!(expected, actual);
    }
    #[test]
    fn rotate_180() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.rotate(&ORIGIN, PI);
        let expected = Point{x:-0.6, y:0.321};
        assert_eq!(expected, actual);
    }
    #[test]
    fn rotate_270() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.rotate(&ORIGIN, 3.0 * PI / 2.0);
        let expected = Point{x:-0.321, y:-0.6};
        assert_eq!(expected, actual);
    }
    #[test]
    fn rotate_360() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.rotate(&ORIGIN, 2.0 * PI);
        let expected = pt;
        assert_eq!(expected, actual);
    }
    #[test]
    fn rotate_420() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.rotate(&ORIGIN, 2.0 * (PI + PI / 6.0));
        let expected = Point{x:0.5779941546, y:0.359115242};
        assert_eq!(expected, actual);
    }
    #[test]
    fn mirror_about_x() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.mirror_about_x();
        let expected = Point{x:0.6, y:0.321};
        assert_eq!(expected, actual);
    }
    #[test]
    fn translate() {
        let pt = Point{x:0.6, y:-0.321};
        let actual = pt.translate(PI / 4.0, 1.4142135623);
        let expected = Point{x:1.6, y:0.679};
        assert_eq!(expected, actual);
    }
    #[test]
    fn distance_to() {
        let pt = Point{x:0.6, y:-0.321};
        let other = Point{x:1.6, y:0.679};
        let actual = pt.distance_to(&other);
        let expected = 1.4142;
        assert_eq!((PRECISION * expected).round() as i32,
                   (PRECISION * actual).round() as i32);
    }
}

#[cfg(test)]
mod line_tests {
    use super::*; // needed to access crate imports
    #[test]
    fn _line_tests_are_running() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn angle_rads() {
        let p1: Point = Point{x:0.0, y:0.0};
        let p2: Point = Point{x:1.0, y:0.0};
        let p3: Point = Point{x:1.0, y:0.0};
        let p4: Point = Point{x:1.0, y:1.0};
        let base: Line = Line{p1, p2};
        let other: Line = Line{p1:p3, p2:p4};
        let actual: f64 = base.angle_rads(&other).unwrap();
        let expected: f64 = -1.0 * PI / 2.0;
        assert_eq!((PRECISION * expected).round() as i32,
                   (PRECISION * actual).round() as i32);
    }
    #[test]
    fn offset_right() {
        let p1: Point = Point{x:0.0, y:0.0};
        let p2: Point = Point{x:1.0, y:1.0};
        let p3: Point = Point{x:1.0, y:-1.0};
        let p4: Point = Point{x:2.0, y:0.0};
        let base: Line = Line{p1, p2};
        let actual: Line = base.offset_right(1.4142135623);
        let expected: Line = Line{p1:p3,p2:p4};
        assert_eq!(actual, expected);
    }
    #[test]
    fn get_mb() {
        let p1: Point = Point{x:1.0, y:1.0};
        let p2: Point = Point{x:2.0, y:-9.0};
        let base: Line = Line{p1, p2};
        let (actual_m, actual_b): (f64, f64) = base.get_mb();
        let expected_m: f64 = -10.0;
        let expected_b: f64 = 11.0;
        assert_eq!(expected_m, actual_m);
        assert_eq!(expected_b, actual_b);
    }
    #[test]
    fn intersect() {
        let p1: Point = Point{x:0.0, y:0.0};
        let p2: Point = Point{x:1.0, y:1.0};
        let p3: Point = Point{x:0.0, y:3.0};
        let p4: Point = Point{x:1.0, y:2.0};
        let base: Line = Line{p1, p2};
        let other: Line = Line{p1:p3, p2:p4};
        let actual: Point = base.intersect(other);
        let expected: Point = Point{x:1.5, y:1.5};
        assert_eq!(expected, actual);
    }
}
