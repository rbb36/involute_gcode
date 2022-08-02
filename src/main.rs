use std::f64::consts::PI;

use involute_gcode::{
    geometry::{Point}, //, Line},
    involute::{Gear, ToothFace},
    canvas,
};

fn main() {
    // canvas::draw_something("");
    do_demos();
}

fn do_demos() {
    // demo_gear_mill();
    demo_gear_mill_offset();
    println!();
    if false {
        gear_catalog();
        println!();
        demo_toothface_mill_offset();
        println!();
        demo_gear_face_01();
        println!();
        demo_point_translation();
        println!();
        demo_face_translation();
        println!();
        example_gear_1();
        println!();
        example_gear_2();
    }
}

fn demo_gear_mill() {
    let gear: Gear = Gear{module:3.0,
                          num_teeth:10 as f64,
                          pressure_angle_degrees:20.0,
                          profile_shift:0.4};
    gear.print_coords(10);
}

fn demo_gear_mill_offset() {
    let gear: Gear = Gear{module:1.0,
                          num_teeth:8 as f64,
                          pressure_angle_degrees:10.0,
                          profile_shift:0.4};
    let num_face_steps: u32 = 10;
    let mill_size:f64 = 1.0;
    let mill_offset:f64 = 0.05;
    let start_z:f64 = 0.0;
    let final_depth:f64 = -3.0;
    let z_step:f64 = -0.2;
    gear.print_traverse_to_start_gcode(num_face_steps,
                                       mill_size,
                                       mill_offset,
                                       start_z,
                                       final_depth,
                                       z_step);
    let mut prev_z = start_z;
    for i in 0..((final_depth / z_step).round() as u32) {
        let mut next_z = prev_z + z_step;
        if next_z < final_depth { next_z = final_depth; }
        gear.print_one_layer_slope_gcode(num_face_steps,
                                         mill_size,
                                         mill_offset,
                                         prev_z,
                                         next_z);
        println!("G0 Z20");
        println!("M00 (stop for inspection");
        println!("G0 Z1");
        println!();
        prev_z = next_z;
    }
    // let faces: Vec<ToothFace> = gear.tooth_faces(10);
    // let offset_faces: Vec<ToothFace> =
    //     gear.mill_offset(10, 2.0, 0.2);
    // for i in 0..faces.len() {
    //     faces.get(i).unwrap().print_coords();
    // }
    // for i in 0..offset_faces.len() {
    //     offset_faces.get(i).unwrap().print_coords();
    // }
}

fn gear_catalog() {
    println!("module\tteeth\tprofile shift\troot width\ttip dia");
    for a in [10.0, 14.5, 20.0] {
        for m in [0.5, 0.8, 1.0, 1.5, 2.0, 2.5, 3.0] {
            for z in [8, 10, 12, 16, 20, 24, 30, 32, 36, 40, 48,
                      50, 56, 60, 64, 70, 72, 80, 88, 96] {
                let mut x = 0.0;
                if z == 8 { x = 0.4; }
                else if z == 10 { x = 0.3; }
                else if z == 12 { x = 0.2; }
                let gear = Gear{module:m, num_teeth:z as f64,
                                pressure_angle_degrees:a, profile_shift:x};
                println!("{}\t{}\t{}\t{}\t{:0.3}\t{}",
                         a, m, z, x, gear.root_width(), gear.tip_diameter());
            }
        }
    }
}

fn demo_toothface_mill_offset() {
    let gear: Gear = Gear{module:2.0,
                          num_teeth:50 as f64,
                          pressure_angle_degrees:20.0,
                          profile_shift:0.0};
    let faces: Vec<ToothFace> = gear.tooth_faces(20);
    let last: &ToothFace = faces.get(faces.len() - 1).unwrap();
    let zeroth: &ToothFace = faces.get(0).unwrap();
    let first: &ToothFace = faces.get(1).unwrap();
    let face: ToothFace = zeroth.mill_offset(last, first, 2.0, 0.2);
    face.print_coords();
    println!();
    zeroth.print_coords();
    println!();
    first.print_coords();
}

fn demo_gear_face_01() {
    // let gear: Gear = Gear{module:2.0,
    //                       num_teeth:num_teeth as f64,
    //                       pressure_angle_degrees:20.0,
    //                       profile_shift:0.4};
    let gear: Gear = Gear{module:2.0,
                          num_teeth:50 as f64,
                          pressure_angle_degrees:20.0,
                          profile_shift:0.0};
    let faces: Vec<ToothFace> = gear.tooth_faces(20);
    for face_index in 0..faces.len() {
        faces.get(face_index).unwrap().print_coords();
    }
}

fn demo_face_translation() {
    let p1: Point = Point{x:1.0, y:0.0};
    let p2: Point = Point{x:2.0, y:0.5};
    let p3: Point = Point{x:3.0, y:1.5};
    let center: Point = Point{x:0.0, y:0.0};
    let angle: f64 = PI / 2.0;
    let points: Vec<Point> = Vec::from([p1, p2, p3]);
    let face: ToothFace = ToothFace{points:points};
    let mirrored: ToothFace = face.mirror_about_x();
    let rotated: ToothFace = mirrored.rotate(&center, angle);
    println!("{:?}", face);
    println!("{:?}", mirrored);
    println!("{:?}", rotated);
}

fn example_gear_1() {
    let gear: Gear = Gear{module:2.0,
                          num_teeth:10.0,
                          pressure_angle_degrees:20.0,
                          profile_shift:0.4};
    gear.print_params();
    gear.print_profile(20);
}

fn example_gear_2() {
    let gear2: Gear = Gear{module: 2.0,
                           num_teeth: 20.0,
                           pressure_angle_degrees: 20.0,
                           profile_shift: 0.0};
    gear2.print_params();
    gear2.print_profile(20);
}

fn demo_point_translation() {
    let point: Point = Point{x:1.0, y:0.5};
    let center: Point = Point{x:0.0, y:0.0};
    println!("point {:?}", point);
    println!("center {:?}", center);
    println!("mirror {:?}", point.mirror_about_x());
    println!("rotate {:?}", point.rotate(&center, 7.5 * PI));
    // let trans: Point = point.mirror_about_x()
    //     .rotate(&center, PI / 4.0);
    // println!("{:?}", trans);
}
