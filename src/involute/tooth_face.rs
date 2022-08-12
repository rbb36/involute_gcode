use std::f64::consts::PI;

use crate::geometry::{Line,Point};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ToothFace {
    pub points: Vec<Point>,
}

impl ToothFace {
    pub fn first(&self) -> &Point {
        self.points.get(0).unwrap()
    }
    pub fn last(&self) -> &Point {
        self.points.get(self.points.len() - 1).unwrap()
    }
    pub fn mill_offset(&self,
                       prev: &ToothFace,
                       next: &ToothFace,
                       mill_d: f64,
                       add_margin: f64) -> ToothFace {
        // let s_first: &Point = self.first();
        // let p_first: &Point = prev.first();
        // let n_first: &Point = next.first();
        // let s_last: &Point = self.last();
        // let p_last: &Point = prev.last();
        // let n_last: &Point = next.last();
        let total_offset: f64 = mill_d / 2.0 + add_margin;
        let mut points: Vec<Point> = Vec::new();
        // 1. move self.first() along path to previous.last()
        let base_angle =
            (prev.last().y - self.first().y)
            .atan2(prev.last().x - self.first().x);
        points.push(self.first().translate(base_angle, total_offset));
        // 2. move all except first and last orthogonal to a
        // line from prior to next
        for x in 1..=(self.points.len() - 2) {
            let prio = self.points.get(x - 1).unwrap();
            let this = self.points.get(x).unwrap();
            let post = self.points.get(x + 1).unwrap();
            let angle = (post.y - prio.y).atan2(post.x - prio.x);
            let ortho = angle - (PI/2.0);
            // println!("angle: {}, ortho: {}", angle, ortho);
            points.push(this.translate(ortho, total_offset));
        }
        {
            // offset self last segment
            let prio = self.points.get(self.points.len() - 2).unwrap();
            let post = next.first();
            let p1: Point = Point{x:prio.x, y:prio.y};
            let p2: Point = Point{x:self.last().x, y:self.last().y};
            let face_l: Line = Line{p1, p2};
            let face_lo: Line = face_l.offset_right(total_offset);
            // offset segment from self.last() to next.first()
            let p1: Point = Point{x:self.last().x, y:self.last().y};
            let p2: Point = Point{x:post.x, y:post.y};
            let tip_l: Line = Line{p1, p2};
            let tip_lo: Line = tip_l.offset_right(total_offset);
            // intersect those two lines
            let intersect = face_lo.intersect(tip_lo);
            points.push(intersect)
        }
        ToothFace{points}
    }
    pub fn copy(&self) -> ToothFace {
        let mut new_points: Vec<Point> = Vec::new();
        for index in 0..self.points.len() {
            let old: &Point = self.points.get(index).unwrap();
            new_points.push(Point{x:old.x, y:old.y});
        }
        ToothFace{points:new_points}
    }
    pub fn rotate(&self, center: &Point, rads: f64) -> ToothFace {
        let mut new_points: Vec<Point> = Vec::new();
        for index in 0..self.points.len() {
            let old_point: &Point = self.points.get(index).unwrap();
            let new_point: Point = old_point.rotate(center, rads);
            new_points.push(new_point);
        }
        ToothFace{points:new_points}
    }
    // Note: also reverses the order of the points, to make
    // gcode generation simpler
    pub fn mirror_about_x_and_reverse_order(&self) -> ToothFace {
        let mut new_points: Vec<Point> = Vec::new();
        for offset in 0..self.points.len() {
            let index = self.points.len() - 1 - offset;
            let old_point: &Point = self.points.get(index).unwrap();
            let new_point: Point = old_point.mirror_about_x();
            new_points.push(new_point);
        }
        ToothFace{points:new_points}
    }
    pub fn mirror_about_x(&self) -> ToothFace {
        let mut new_points: Vec<Point> = Vec::new();
        for index in 0..self.points.len() {
            let old_point: &Point = self.points.get(index).unwrap();
            let new_point: Point = old_point.mirror_about_x();
            new_points.push(new_point);
        }
        ToothFace{points:new_points}
    }
    pub fn print_coords(&self) {
        for index in 0..self.points.len() {
            let point: &Point = self.points.get(index).unwrap();
            println!("{}\t{}", point.x, point.y);
        }
    }
    pub fn print_gcode(&self) {
        for point in &self.points {
            println!("G1 X{:.3} Y{:.3}", point.x, point.y);
        }
    }
    pub fn print_gcode_with_slope(&self, prev_z:f64, face_dz:f64) {
        let dz = face_dz / ((self.points.len() as f64) - 1.0);
        for i in 0..self.points.len() {
            let point = self.points.get(i).unwrap();
            let z = prev_z + ((i as f64) * dz);
            println!("G1 X{:.3} Y{:.3} Z{:.3}", point.x, point.y, z);
        }
    }
}
