use std::num::ParseFloatError;
use std::f64::consts::PI;

// use millionths of a unit to mean equal
pub const PRECISION: f64 = 1_000_000.0;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    // returns on the range -PI <= angle < PI.
    pub fn angle_to(&self, other: &Self) -> f64 {
        return (other.y - self.y).atan2(other.x - self.x);
    }
    pub fn rotate(&self, center: &Point, rads: f64) -> Point {
        let x1 = self.x - center.x;
        let y1 = self.y - center.y;
        let x2 = x1 * rads.cos() - y1 * rads.sin();
        let y2 = y1 * rads.cos() + x1 * rads.sin();
        let x3 = x2 + center.x;
        let y3 = y2 + center.y;
        Point{x:x3, y:y3}
    }
    pub fn mirror_about_x(&self) -> Point {
        Point{x:self.x, y:self.y * -1.0}
    }
    pub fn translate(&self, angle: f64, distance: f64) -> Point {
        let x = self.x + angle.cos() * distance;
        let y = self.y + angle.sin() * distance;
        // println!("cos: {}, sin: {}, self {:?}, new {:?}", angle.cos(), angle.sin(), self, Point{x, y});
        Point{x, y}
    }
    pub fn distance_to(&self, other: &Self) -> f64 {
        return ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let sxc: i64 = (self.x * PRECISION).round() as i64;
        let oxc: i64 = (other.x * PRECISION).round() as i64;
        let syc: i64 = (self.y * PRECISION).round() as i64;
        let oyc: i64 = (other.y * PRECISION).round() as i64;
        sxc == oxc && syc == oyc
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}
impl Line {
    // returns the smaller included angle, on the range -PI to PI,
    // from self to other about their common point.
    // 
    // positive is counterclockwise
    // 
    // throws ParseFloatError if the two lines do not have a common
    // point, becase I don't know how to throw an error yet, except
    // by causing one.
    pub fn angle_rads(&self, other: &Line) -> Result<f64, ParseFloatError> {
        let this_angle: f64;
        let that_angle: f64;
        if self.p1 == other.p1 {
            this_angle = self.p1.angle_to(&self.p2);
            that_angle = other.p1.angle_to(&other.p2);
        } else if self.p1 == other.p2 {
            this_angle = self.p1.angle_to(&self.p2);
            that_angle = other.p2.angle_to(&other.p1);
        } else if self.p2 == other.p1 {
            this_angle = self.p2.angle_to(&self.p1);
            that_angle = other.p1.angle_to(&other.p2);
        } else if self.p2 == other.p2 {
            this_angle = self.p2.angle_to(&self.p1);
            that_angle = other.p2.angle_to(&other.p1);
        } else {
            return "foo".parse::<f64>();
        }
        let diff = that_angle - this_angle;
        if diff < -1.0*PI {
            return Ok(2.0 * PI + diff);
        } else if diff > PI {
            return Ok(-2.0 * PI + diff);
        }
        Ok(that_angle - this_angle)
    }
    pub fn offset_right(&self, dist: f64) -> Line {
        let angle = (self.p2.y - self.p1.y).atan2(self.p2.x - self.p1.x);
        let ortho = angle - PI/2.0;
        let p1: Point = self.p1.translate(ortho, dist);
        let p2: Point = self.p2.translate(ortho, dist);
        Line{p1, p2}
    }
    // returns the slope and intercept
    pub fn get_mb(&self) -> (f64,f64) {
        // y = mx + b
        // b = mx - y
        // mx1 - y1 = mx2 - y2
        // m = (y1 - y2) / (x1 - x2)
        // b = mx1 - y1
        let m: f64 = (self.p1.y - self.p2.y) / (self.p1.x - self.p2.x);
        let b: f64 = self.p1.y - m * self.p1.x;
        (m, b)
    }
    pub fn intersect(&self, other: Line) -> Point {
        let (self_m, self_b): (f64, f64) = self.get_mb();
        let (other_m, other_b): (f64, f64) = other.get_mb();
        println!("sm {}, sb {}, om {}, ob {}", self_m, self_b, other_m, other_b);
        // y = self_m * x + self_b;
        // y = other_m * x + other_b;
        // self_m * x + self_b = other_m * x + other_b;
        // self_m * x = other_m * x + other_b - self_b;
        // self_m * x - other_m * x = other_b - self_b;
        // x * (self_m - other_m) = other_b - self_b;
        // x = (other_b - self_b) / (self_m - other_m);
        let x = (other_b - self_b) / (self_m - other_m);
        let y = self_m * x + self_b;
        let oy = other_m * x + other_b;
        println!("x: {}, self y: {}, other y: {}", x, y, oy);
        Point{x, y}
    }
}
