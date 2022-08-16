use crate::geometry::Arc;
use crate::geometry::Circle;
use crate::geometry::Point;

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
    pub fn reverse_order(&self) -> ToothFace {
        let mut new_points: Vec<Point> = Vec::new();
        for offset in 0..self.points.len() {
            let index = self.points.len() - 1 - offset;
            let old_point: &Point = self.points.get(index).unwrap();
            let new_point: Point = old_point.copy();
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
    pub fn root_arc_to(&self, next:&Self) -> Arc {
        let start = self.last().copy();
        let end = next.first().copy();
        let cx = (end.x + start.x) / 2.0;
        let cy = (end.y + start.y) / 2.0;
        let center:Point = Point{x:cx, y:cy};
        let radius = center.distance_to(&start);
        println!("start: {:?}, end: {:?}", start, end);
        println!("cx: {:.3}, cy {:.3}, center {:?}, radius {:.3}",
                 cx, cy, center, radius);
        let start_angle = center.angle_to(&start);
        let end_angle = center.angle_to(&end);
        let included_angle = start_angle - end_angle;//end_angle - start_angle; // start_angle - end_angle;
        // -1.0 * (end_angle - start_angle);
        println!("sa{:.3}, ea{:.3}, ia{:.3}", start_angle, end_angle, included_angle);
        let circle:Circle = Circle{center, radius};
        Arc{circle, start_angle, included_angle}
    }
    pub fn print_coords(&self) {
        for index in 0..self.points.len() {
            let point: &Point = self.points.get(index).unwrap();
            println!("{}\t{}", point.x, point.y);
        }
    }
}
