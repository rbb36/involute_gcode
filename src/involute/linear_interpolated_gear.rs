use std::f64::consts::PI;

use crate::geometry::Point;
use crate::involute::gear_params::GearParams;
use crate::involute::tooth_face::ToothFace;

const CLIMB_CUT:bool = true;

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub struct LinearInterpolatedGear {
    pub params: GearParams,
}

impl LinearInterpolatedGear {
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
    pub fn tooth_faces(&self, num_steps: u32) -> Vec<ToothFace> {
        let center: Point = Point{x:0.0, y:0.0};
        let mut faces: Vec<ToothFace> = Vec::new();
        let mut right: ToothFace = self.tooth_face(num_steps);
        let mut left: ToothFace = right.mirror_about_x_and_reverse_order()
            .rotate(&center, self.params.base_thickness());
        faces.push(right.copy());
        faces.push(left.copy());
        let tooth_angle: f64 = 2.0 * PI / self.params.num_teeth;
        for _tooth_index in 1..(self.params.num_teeth as i32) {
            right = right.rotate(&center, tooth_angle);
            left = left.rotate(&center, tooth_angle);
            faces.push(right.copy());
            faces.push(left.copy());
        }
        faces
    }
    // returns a cut path, a flag saying whether it was truncated at the root,
    // and the mininum radius to the center of the mill that it reached.
    pub fn mill_offset(&self,
                       num_steps: u32,
                       mill_d: f64,
                       add_margin: f64,
                       min_radius:f64) -> (Vec<ToothFace>, bool, f64) {
        let final_faces: Vec<ToothFace> = self.tooth_faces(num_steps);
        let prev: &ToothFace = final_faces.get(final_faces.len() - 1).unwrap();
        let this: &ToothFace = final_faces.get(0).unwrap();
        let next: &ToothFace = final_faces.get(1).unwrap();
        let face: ToothFace = this.mill_offset(prev, next, mill_d, add_margin);
        let center: Point = Point{x:0.0, y:0.0};
        let tooth_angle: f64 = 2.0 * PI / self.params.num_teeth;
        
        // trim points where the mill does not fit into the gap
        // between the teeth
        let inter_rotate:f64 = self.params.base_thickness() - tooth_angle;
        let inter_face: ToothFace =
            face.mirror_about_x().rotate(&center, inter_rotate);
        let mut clean_points:Vec<Point> = Vec::new();
        let mut is_truncated:bool = false;
        for i in 0..inter_face.points.len() {
            let point:&Point = face.points.get(i).unwrap();
            let radius:f64 = point.distance_to(&center);
            let face_x:f64 = point.x;
            let inter_x:f64 = inter_face.points.get(i).unwrap().x;
            if inter_x > face_x {
                is_truncated = true;
            } else if radius < min_radius {
                if i + 1 >= face.points.len() {
                  panic!("the mill - it does nothing");
                // } else {
                //     let next_point = face.points.get(i + 1).unwrap();
                //     let next_radius:f64 = next_point.distance_to(&center);
                //     if next_radius < min_radius {
                //         // let the next point handle it
                //     } else {
                //         let face_y = point.y;
                //         let next_x = next_point.x;
                //         let next_y = next_point.y;
                //         let ra_ra:f64 = (min_radius - radius)
                //             / (next_radius - radius);
                //         let cut_x:f64 = face_x + ra_ra * (next_x - face_x);
                //         let cut_y:f64 = face_y + ra_ra * (next_y - face_y);
                //     }
                }
            } else {
                clean_points.push(face.points.get(i).unwrap().copy());
            }
        }
        let min_point:&Point = &(clean_points.get(0).unwrap());
        let min_radius:f64 = min_point.distance_to(&center);
        let clean_face:ToothFace = ToothFace{points:clean_points};
        
        let mut faces: Vec<ToothFace> = Vec::new();
        let mut right: ToothFace = clean_face;
        let mut left: ToothFace = right.mirror_about_x_and_reverse_order()
            .rotate(&center, self.params.base_thickness());
        faces.push(right.copy());
        faces.push(left.copy());
        for _tooth_index in 1..(self.params.num_teeth as i32) {
            right = right.rotate(&center, tooth_angle);
            left = left.rotate(&center, tooth_angle);
            faces.push(right.copy());
            faces.push(left.copy());
        }
        (faces, is_truncated, min_radius)
    }
    pub fn print_coords(&self, num_face_steps:u32) {
        for face in self.tooth_faces(num_face_steps) {
            if CLIMB_CUT {
                face.mirror_about_x().print_coords();
            } else {
                face.print_coords();
            }
        }
    }
    pub fn print_traverse_to_start_gcode(&self,
                                         num_face_steps:u32,
                                         mill_size:f64,
                                         mill_offset:f64,
                                         start_z:f64,
                                         final_depth:f64,
                                         z_step:f64,
                                         min_radius:f64) -> (bool,f64) {
        let result: (Vec<ToothFace>, bool, f64) = 
            self.mill_offset(num_face_steps, mill_size, mill_offset, min_radius);
        let mut offset_faces:Vec<ToothFace> = result.0;
        let is_truncated:bool = result.1;
        let min_radius:f64 = result.2;
        if CLIMB_CUT {
            let mut mirrored_faces:Vec<ToothFace> = Vec::new();
            for face in offset_faces {
                let mirrored_face:ToothFace = face.mirror_about_x();
                mirrored_faces.push(mirrored_face);
            }
            offset_faces = mirrored_faces;
        }
        let face = offset_faces.get(0).unwrap();
        let traverse_z = start_z + 10.0;
        let line_up_x = face.first().x;
        let line_up_y = face.first().y;
        let line_up_z = start_z + 1.0;
        println!(";; -------------------------------------");
        println!(";; Involute Gear G-Code");
        println!(";; -------------------------------------");
        if is_truncated {
            println!(";; WARNING: TRUNCATED CUT");
            println!(";; This cut uses a mill that is too large to reach");
            println!(";; the root between the teeth. This will not work");
            println!(";; as a final pass.");
        }
        println!(";; -------------------------------------");
        println!(";; Module: m={:.3}", self.params.module);
        println!(";; Num Teeth: z={:.3}", self.params.num_teeth);
        println!(";; Pressure Angle: a={:.3}", self.params.pressure_angle_degrees);
        println!(";; Profile Shift: x={:.3}", self.params.profile_shift);
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
        (is_truncated, min_radius)
    }
    pub fn print_one_layer_slope_gcode(&self,
                                       num_face_steps: u32,
                                       mill_size: f64,
                                       mill_offset: f64,
                                       start_z: f64,
                                       end_z: f64,
                                       tab_z: f64,
                                       min_radius:f64) -> (bool,f64) {
        let result:(Vec<ToothFace>, bool, f64) =
            self.mill_offset(num_face_steps, mill_size, mill_offset, min_radius);
        let mut offset_faces:Vec<ToothFace> = result.0;
        let is_truncated:bool = result.1;
        let min_radius:f64 = result.2;
        if CLIMB_CUT {
            let mut mirrored_faces:Vec<ToothFace> = Vec::new();
            for face in offset_faces {
                let mirrored_face:ToothFace = face.mirror_about_x();
                mirrored_faces.push(mirrored_face);
            }
            offset_faces = mirrored_faces;
        }
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
            if prev_z < tab_z {
                println!("G1 Z{}", tab_z);
                // this_last_point = this_face.last();
                let target:&Point = next_face.first();
                println!("G1 X{:0.3} Y{:0.3}", target.x, target.y);
                println!("G1 Z{:0.3}", prev_z);
            }
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
            if CLIMB_CUT {
            println!("G3 X{:0.3} Y{:0.3} I{:0.3} J{:0.3}",
                     arc_1.x, arc_1.y, arc_c_i, arc_c_j);
            } else {
                println!("G2 X{:0.3} Y{:0.3} I{:0.3} J{:0.3}",
                         arc_1.x, arc_1.y, arc_c_i, arc_c_j);
            }
            // println!("G1 Z-0.2");
            // offset_faces.get(i).unwrap().print_gcode();
        }
        (is_truncated,min_radius)
    }
    pub fn print_one_layer_coords(&self,
                                  num_face_steps: u32,
                                  mill_size: f64,
                                  mill_offset: f64,
                                  start_z: f64,
                                  end_z: f64,
                                  min_radius: f64) -> (bool,f64) {
        let mill_offset:(Vec<ToothFace>, bool,f64) =
            self.mill_offset(num_face_steps, mill_size, mill_offset, min_radius);
        
        let mut offset_faces:Vec<ToothFace> = mill_offset.0;
        let is_truncated:bool = mill_offset.1;
        let min_radius:f64 = mill_offset.2;
            
        if CLIMB_CUT {
            let mut mirrored_faces:Vec<ToothFace> = Vec::new();
            for face in offset_faces {
                let mirrored_face:ToothFace = face.mirror_about_x();
                mirrored_faces.push(mirrored_face);
            }
            offset_faces = mirrored_faces;
        }
        let num_faces = offset_faces.len();
        let num_teeth = num_faces / 2;
        let total_dz = end_z - start_z;
        let _face_dz = total_dz / (num_faces as f64);
        let _prev_z = start_z;
        for this_tooth_i in 0..num_teeth {
            let this_face_i = this_tooth_i * 2;
            let next_face_i = (this_face_i + num_faces + 1) % num_faces;
            // let subs_face_i = (this_face_i + num_faces + 2) % num_faces;
            let this_face = offset_faces.get(this_face_i).unwrap();
            let next_face = offset_faces.get(next_face_i).unwrap();
            // let subs_face = offset_faces.get(subs_face_i).unwrap();
            this_face.print_coords();
            next_face.print_coords();
        }
        (is_truncated,min_radius)
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
