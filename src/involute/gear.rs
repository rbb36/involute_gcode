// use std::f64::consts::PI;

use crate::geometry::Point;
use crate::involute::gear_params::GearParams;
use crate::involute::tooth_face::ToothFace;

// const CLIMB_CUT:bool = true;

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub struct Gear {
    pub params: GearParams,
}

impl Gear {
    // creates a face starting at {+x, 0} and extending out and clockwise
    pub fn tooth_face(&self, num_steps: u32) -> ToothFace {
        let step_size = self.params.u_max() / num_steps as f64;
        let mut points: Vec<Point> = Vec::new();
        for offset in 0..=num_steps {
            let u = offset as f64 * step_size;
            let x = self.params.tooth_profile_x(u);
            let y = self.params.tooth_profile_y(u);
            let point: Point = Point{x, y};
            points.push(point);
        }
        ToothFace{points}
    }
    pub fn print_profile(&self, num_steps: u32) {
        let step_size = self.params.u_max() / num_steps as f64;
        for offset in 0..=num_steps {
            let u = offset as f64 * step_size;
            let x = self.params.tooth_profile_x(u);
            let y = self.params.tooth_profile_y(u);
            println!("{}\t{}\t{}\t{}", offset, u, x, y);
        }
    }
}
