use std::f64::consts::PI;

use crate::geometry::{Line, Point, PRECISION};

#[derive(Debug)]
// #[derive(PartialEq)] // auto-generates eq(), removed for hand-coded (below)
#[allow(dead_code)]
pub struct Gear {
    pub module: f64, // (aka: m)
    pub num_teeth: f64, // (aka: z)
    pub pressure_angle_degrees: f64, // (aka: a)
    pub profile_shift: f64, // (aka: x, roughly: (z8 =~ .4, z16+ = 0))
}

impl Gear {
    pub fn print_coords(&self, num_face_steps:u32) {
        for face in self.tooth_faces(num_face_steps) {
            face.print_coords();
        }
    }
    pub fn print_traverse_to_start_gcode(&self,
                                         num_face_steps:u32,
                                         mill_size:f64,
                                         mill_offset:f64,
                                         start_z:f64,
                                         final_depth:f64,
                                         z_step:f64) {
        let offset_faces: Vec<ToothFace> =
            self.mill_offset(num_face_steps, mill_size, mill_offset);
        let face = offset_faces.get(0).unwrap();
        let traverse_z = start_z + 10.0;
        let line_up_x = face.first().x;
        let line_up_y = face.first().y;
        let line_up_z = start_z + 1.0;
        println!(";; -------------------------------------");
        println!(";; Involute Gear G-Code");
        println!(";; -------------------------------------");
        println!(";; Module: m={:.3}", self.module);
        println!(";; Num Teeth: z={:.3}", self.num_teeth);
        println!(";; Pressure Angle: a={:.3}", self.pressure_angle_degrees);
        println!(";; Profile Shift: x={:.3}", self.profile_shift);
        println!(";; Num Face Steps: {:.3}", num_face_steps);
        println!(";; Mill Size: {:.3}", mill_size);
        println!(";; Mill Offset: {:.3}", mill_offset);
        println!(";; Start Z: {:.3}", start_z);
        println!(";; Final Depth: {:.3}", final_depth);
        println!(";; Z Step: {:.3}", z_step);
        println!(";; -------------------------------------");
        println!("G0 Z{:.3}", traverse_z);
        println!("G0 X{:.3} Y{:.3}", line_up_x, line_up_y);
        println!("M00 (stop for inspection)");
        println!("G0 Z{:.3}", line_up_z);
        println!();
    }
    pub fn print_one_layer_slope_gcode(&self,
                                       num_face_steps: u32,
                                       mill_size: f64,
                                       mill_offset: f64,
                                       start_z: f64,
                                       end_z: f64) {
        let offset_faces: Vec<ToothFace> =
            self.mill_offset(num_face_steps, mill_size, mill_offset);
        let num_faces = offset_faces.len();
        let num_teeth = num_faces / 2;
        let total_dz = end_z - start_z;
        let face_dz = total_dz / (num_faces as f64);
        let mut prev_z = start_z;
        for this_tooth_i in 0..num_teeth {
            let this_face_i = this_tooth_i * 2;
            let next_face_i = (this_face_i + num_faces + 1) % num_faces;
            let subs_face_i = (this_face_i + num_faces + 2) % num_faces;
            let this_face = offset_faces.get(this_face_i).unwrap();
            let next_face = offset_faces.get(next_face_i).unwrap();
            let subs_face = offset_faces.get(subs_face_i).unwrap();
            // print this face G-code with slope.
            this_face.print_gcode_with_slope(prev_z, face_dz);
            prev_z += face_dz;
            // print tip line to next face.
            println!();
            // -- not actually needed, it will do this implicitly
            // this_last_point = this_face.last();
            // next_first_point = next_face.first();
            // print next face.
            next_face.print_gcode_with_slope(prev_z, face_dz);
            prev_z += face_dz;
            // print arc to subsequent face.
            let arc_0 = next_face.last();
            let arc_1 = subs_face.first();
            let arc_c_x = (arc_0.x + arc_1.x)/2.0;
            let arc_c_y = (arc_0.y + arc_1.y)/2.0;
            let arc_c_i = arc_c_x - arc_0.x;
            let arc_c_j = arc_c_y - arc_0.y;
            println!("G17 (set XY coordinate system for arc cut)");
            println!("G2 X{:0.3} Y{:0.3} I{:0.3} J{:0.3}",
                     arc_1.x, arc_1.y, arc_c_i, arc_c_j);
            // println!("G1 Z-0.2");
            // offset_faces.get(i).unwrap().print_gcode();
        }
    }
    pub fn print_params(&self) {
        println!("module: {}", self.module);
        println!("num_teeth: {}", self.num_teeth);
        println!("pressure_angle_degrees: {}", self.pressure_angle_degrees);
        println!("profile_shift: {}", self.profile_shift);
        println!("pressure_angle: {}", self.pressure_angle());
        println!("ref_diameter: {}", self.ref_diameter());
        println!("pitch_diameter: {}", self.pitch_diameter());
        println!("pitch_radius: {}", self.pitch_radius());
        println!("base_diameter: {}", self.base_diameter());
        println!("base_radius: {}", self.base_radius());
        println!("tip_diameter: {}", self.tip_diameter());
        println!("tip_radius: {}", self.tip_radius());
        println!("tip_pressure_angle: {}", self.tip_pressure_angle());
        println!("involute: {}", self.involute());
        println!("tip_involute: {}", self.tip_involute());
        println!("tip_thickness: {}", self.tip_thickness());
        println!("tip_radius: {}", self.tip_radius());
        println!("base_radius: {}", self.base_radius());
        println!("u_max: {}", self.u_max());
        println!("base thickness: {}", self.base_thickness());
        println!("base degrees: {}", self.base_thickness() * 180.0 / PI);
        println!("root width: {}", self.root_width());
    }
    pub fn pressure_angle(&self) -> f64 {
        self.pressure_angle_degrees * PI / 180.0
    }
    pub fn ref_diameter(&self) -> f64 {
        self.module * self.num_teeth
    }
    pub fn ref_radius(&self) -> f64 {
        self.ref_diameter() / 2.0
    }
    pub fn pitch_radius(&self) -> f64 {
        // center spacing increases by module (m) * shift (x)
        let v = self.module * self.profile_shift;
        self.ref_radius() + v
    }
    pub fn pitch_diameter(&self) -> f64 {
        // diameter is increased by 2x spacing increase
        self.pitch_radius() * 2.0
    }
    pub fn base_diameter(&self) -> f64 {
        self.ref_diameter() * self.pressure_angle().cos()
    }
    pub fn base_radius(&self) -> f64 {
        self.base_diameter() / 2.0
    }
    pub fn tip_radius(&self) -> f64 {
        let shift_ratio: f64 = 1.0 + self.profile_shift;
        let shift_amount: f64 = self.module * shift_ratio;
        self.ref_radius() + shift_amount
    }
    pub fn tip_diameter(&self) -> f64 {
        self.tip_radius() * 2.0
    }
    pub fn tip_pressure_angle(&self) -> f64 {
        (self.base_diameter() / self.tip_diameter()).acos()
    }
    pub fn involute(&self) -> f64 {
        self.pressure_angle().tan() - self.pressure_angle()
    }
    pub fn tip_involute(&self) -> f64 {
        self.tip_pressure_angle().tan() - self.tip_pressure_angle()
    }
    pub fn tip_thickness(&self) -> f64 {
        let half_tooth_angle = (PI / 2.0) / self.num_teeth;
        let shift_correct = (2.0 * self.profile_shift * self.pressure_angle().tan()) / self.num_teeth;
        let pitch_to_tip_angle = self.involute() - self.tip_involute();
        2.0 * (half_tooth_angle + shift_correct + pitch_to_tip_angle)
    }
    pub fn u_max(&self) -> f64 {
        let tip_r_sq: f64 = self.tip_radius().powi(2);
        let base_r_sq: f64 = self.base_radius().powi(2);
        ((tip_r_sq / base_r_sq) - 1.0).sqrt()
    }
    // included angle of the base of the tooth in radians
    pub fn base_thickness(&self) -> f64 {
        let umax = self.u_max();
        let base_r = self.base_radius();
        let tip_r = self.tip_radius();
        let base_x = base_r;
        let base_y = 0.0;
        let tip_x = self.tooth_profile_x(umax);
        let tip_y = self.tooth_profile_y(umax);
        let base_to_tip = ((base_x - tip_x).powi(2) + (base_y - tip_y).powi(2)).sqrt();
        let thick_cos = (base_r.powi(2) + tip_r.powi(2) - base_to_tip.powi(2))
            / (2.0 * base_r * tip_r);
        self.tip_thickness() + 2.0 * thick_cos.acos()
    }
    pub fn tooth_profile_x(&self, u: f64) -> f64 {
        self.base_radius() * (u.cos() + u * u.sin())
    }
    pub fn tooth_profile_y(&self, u: f64) -> f64 {
        self.base_radius() * (u.sin() - u * u.cos())
    }
    pub fn print_profile(&self, num_steps: u32) {
        let step_size = self.u_max() / num_steps as f64;
        for offset in 0..=num_steps {
            let u = offset as f64 * step_size;
            let x = self.tooth_profile_x(u);
            let y = self.tooth_profile_y(u);
            println!("{}\t{}\t{}\t{}", offset, u, x, y);
        }
    }
    // returns the space the mill has to fit into
    // Note: if this is too tight, you won't have room
    // for a rough pass before the final pass.
    pub fn root_width(&self) -> f64 {
        // let base_c = self.base_diameter() * PI;
        let tooth_base_angle = self.base_thickness();
        let total_tooth_angle = tooth_base_angle * self.num_teeth;
        let remaining_angle = (2.0 * PI) - total_tooth_angle;
        let root_angle = remaining_angle / self.num_teeth;
        let half_angle = root_angle / 2.0;
        let root_width = 2.0 * half_angle.sin() * self.base_radius();
        return root_width;
    }
    pub fn tooth_face(&self, num_steps: u32) -> ToothFace {
        let step_size = self.u_max() / num_steps as f64;
        let mut points: Vec<Point> = Vec::new();
        for offset in 0..=num_steps {
            let u = offset as f64 * step_size;
            let x = self.tooth_profile_x(u);
            let y = self.tooth_profile_y(u);
            let point: Point = Point{x, y};
            points.push(point);
        }
        ToothFace{points}
    }
    pub fn tooth_faces(&self, num_steps: u32) -> Vec<ToothFace> {
        let center: Point = Point{x:0.0, y:0.0};
        let mut faces: Vec<ToothFace> = Vec::new();
        let mut right: ToothFace = self.tooth_face(num_steps);
        let mut left: ToothFace = right.mirror_about_x()
            .rotate(&center, self.base_thickness());
        faces.push(right.copy());
        faces.push(left.copy());
        let tooth_angle: f64 = 2.0 * PI / self.num_teeth;
        for _tooth_index in 1..(self.num_teeth as i32) {
            right = right.rotate(&center, tooth_angle);
            left = left.rotate(&center, tooth_angle);
            faces.push(right.copy());
            faces.push(left.copy());
        }
        faces
    }
    pub fn mill_offset(&self,
                       num_steps: u32,
                       mill_d: f64,
                       add_margin: f64) -> Vec<ToothFace> {
        let final_faces: Vec<ToothFace> = self.tooth_faces(num_steps);
        let prev: &ToothFace = final_faces.get(final_faces.len() - 1).unwrap();
        let this: &ToothFace = final_faces.get(0).unwrap();
        let next: &ToothFace = final_faces.get(1).unwrap();
        let face: ToothFace =
            this.mill_offset(prev, next, mill_d, add_margin);
        // println!();
        // face.print_coords();
        // println!();
        let center: Point = Point{x:0.0, y:0.0};
        let mut faces: Vec<ToothFace> = Vec::new();
        let mut right: ToothFace = face;
        let mut left: ToothFace = right.mirror_about_x()
            .rotate(&center, self.base_thickness());
        faces.push(right.copy());
        faces.push(left.copy());
        let tooth_angle: f64 = 2.0 * PI / self.num_teeth;
        for _tooth_index in 1..(self.num_teeth as i32) {
            right = right.rotate(&center, tooth_angle);
            left = left.rotate(&center, tooth_angle);
            faces.push(right.copy());
            faces.push(left.copy());
        }
        faces
    }
}

impl PartialEq for Gear {
    fn eq(&self, other: &Self) -> bool {
        let smc: i64 = (self.module * PRECISION).round() as i64;
        let omc: i64 = (other.module * PRECISION).round() as i64;
        let szc: i64 = (self.num_teeth * PRECISION).round() as i64;
        let ozc: i64 = (other.num_teeth * PRECISION).round() as i64;
        let sac: i64 = (self.pressure_angle_degrees * PRECISION).round() as i64;
        let oac: i64 = (other.pressure_angle_degrees * PRECISION).round() as i64;
        let sxc: i64 = (self.profile_shift * PRECISION).round() as i64;
        let oxc: i64 = (other.profile_shift * PRECISION).round() as i64;
        smc == omc && szc == ozc && sac == oac && sxc == oxc
    }
}

#[derive(Debug)]
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
        let base_angle = (prev.last().y - self.first().y)
            .atan2(prev.last().x - self.first().x);
        points.push(self.first().translate(base_angle, total_offset));
        // 2. move all except first and last orthogonal to a
        // line from prior to next
        for x in 1..=(self.points.len() - 2) {
            let prio = self.points.get(x - 1).unwrap();
            let this = self.points.get(x).unwrap();
            let post = self.points.get(x + 1).unwrap();
            let angle = (post.y - prio.y).atan2(post.x - prio.x);
            let ortho = angle - PI/2.0;
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
    pub fn mirror_about_x(&self) -> ToothFace {
        let mut new_points: Vec<Point> = Vec::new();
        for offset in 0..self.points.len() {
            let index = self.points.len() - 1 - offset;
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
