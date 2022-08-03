use std::f64::consts::PI;

use involute_gcode::{
    geometry::{Point}, //, Line},
    involute::gear::Gear,
    involute::gear_params::GearParams,
    involute::tooth_face::ToothFace,
    canvas,
};

fn main() {
    do_demos();
}

fn do_demos() {
        demo_gear_mill_offset_coords();
        println!();
    if false {
        gear_catalog();
        println!();
        demo_gear_mill_offset_gcode();
        println!();
        canvas::draw_something("");
        println!();
        demo_gear_mill();
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
    let module:f64 = 3.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    gear.print_coords(10);
}

fn demo_gear_mill_offset_coords() {
    let module:f64 = 2.25;
    let num_teeth:f64 = 8 as f64;
    let pressure_angle_degrees:f64 = 10.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    let num_face_steps: u32 = 8;
    let mill_size:f64 = 3.0;
    let mill_offset:f64 = 0.1;
    let start_z:f64 = 0.0;
    let _final_depth:f64 = -2.95;
    let z_step:f64 = -0.2;
    gear.print_coords(num_face_steps);
    println!();
    gear.print_one_layer_coords(num_face_steps,
                                mill_size,
                                mill_offset,
                                start_z,
                                start_z + z_step);
}

fn demo_gear_mill_offset_gcode() {
    let module:f64 = 1.0;
    let num_teeth:f64 = 8 as f64;
    let pressure_angle_degrees:f64 = 10.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    let num_face_steps: u32 = 10;
    let mill_size:f64 = 1.0;
    let mill_offset:f64 = 0.05;
    let start_z:f64 = 0.0;
    let final_depth:f64 = -2.95;
    let z_step:f64 = -0.2;
    let mut tab_z:f64 = final_depth + 0.5;
    gear.print_traverse_to_start_gcode(num_face_steps,
                                       mill_size,
                                       mill_offset,
                                       start_z,
                                       final_depth,
                                       z_step);
    let mut prev_z = start_z;
    println!("F60");
    while prev_z > final_depth {
        let mut next_z = prev_z + z_step;
        if next_z < final_depth { next_z = final_depth; }
        gear.print_one_layer_slope_gcode(num_face_steps,
                                         mill_size,
                                         mill_offset,
                                         prev_z,
                                         next_z,
                                         tab_z);
        println!("G0 Z5");
        println!("M00 (stop for inspection)");
        println!("G0 Z1");
        println!();
        prev_z = next_z;
    }
    gear.print_one_layer_slope_gcode(num_face_steps,
                                     mill_size,
                                     mill_offset,
                                     final_depth,
                                     final_depth,
                                     tab_z);
    println!("G0 Z5");
    println!("M00 (stop for inspection)");
    println!("G0 Z1");
    println!();

    println!(";; LAST FULL TABS PASS ------------------------------");
    // smoothing pass with full tabs
    let num_face_steps:u32 = 20;
    let mill_offset:f64 = 0.0;
    let _start_z:f64 = -3.0;
    let final_depth:f64 = -3.0;
    let _z_step:f64 = -1.0;
    println!("F30");
    gear.print_one_layer_slope_gcode(num_face_steps,
                                     mill_size,
                                     mill_offset,
                                     final_depth,
                                     final_depth,
                                     tab_z);
    println!("G0 Z20");
    println!("M00 (stop for inspection)");
    println!("G0 Z1");
    println!();

    println!(";; FIRST TAB CUTTING PASS ------------------------------");
    // smoothing pass with 0.3mm tabs
    let num_face_steps:u32 = 20;
    let mill_offset:f64 = 0.0;
    let _start_z:f64 = -3.0;
    let final_depth:f64 = -3.0;
    let _z_step:f64 = -1.0;
    tab_z = final_depth + 0.3;
    println!("F30");
    gear.print_one_layer_slope_gcode(num_face_steps,
                                     mill_size,
                                     mill_offset,
                                     final_depth,
                                     final_depth,
                                     tab_z);
    println!("G0 Z20");
    println!("M00 (stop for inspection)");
    println!("G0 Z1");
    println!();

    // smoothing pass with 0.1mm tabs
    let num_face_steps:u32 = 20;
    let mill_offset:f64 = 0.0;
    let _start_z:f64 = -3.0;
    let final_depth:f64 = -3.0;
    let _z_step:f64 = -1.0;
    tab_z = final_depth + 0.1;
    println!("F30");
    gear.print_one_layer_slope_gcode(num_face_steps,
                                     mill_size,
                                     mill_offset,
                                     final_depth,
                                     final_depth,
                                     tab_z);
    println!("G0 Z20");
    println!("M00 (stop for inspection)");
    println!("G0 Z1");
    println!();

    // smoothing pass to remove tabs
    let num_face_steps:u32 = 20;
    let mill_offset:f64 = 0.0;
    let _start_z:f64 = -3.0;
    let final_depth:f64 = -3.0;
    let _z_step:f64 = -1.0;
    tab_z = final_depth + 0.0;
    println!("F30");
    gear.print_one_layer_slope_gcode(num_face_steps,
                                     mill_size,
                                     mill_offset,
                                     final_depth,
                                     final_depth,
                                     tab_z);
    println!("G0 Z20");
    println!("M00 (stop for inspection)");
    println!("G0 Z1");
    println!();
}

fn gear_catalog() {
    println!("angle\tmodule\tteeth\tprofile shift\troot width\ttip dia");
    //for a in [10.0, 14.5, 20.0] {
    for a in [10.0] {
        //for m in [0.5, 0.8, 1.0, 1.5, 2.0, 2.5, 3.0] {
        for m in [2.2, 2.25, 2.3] {
            for z in [8, 10, 12, 16, 20, 24, 30, 32, 36, 40, 48,
                      50, 56, 60, 64, 70, 72, 80, 88, 96] {
                let mut x = 0.0;
                if z == 8 { x = 0.4; }
                else if z == 10 { x = 0.3; }
                else if z == 12 { x = 0.2; }
                let params:GearParams =
                    GearParams{module:m, num_teeth:z as f64,
                               pressure_angle_degrees:a, profile_shift:x};
                println!("{}\t{}\t{}\t{}\t{:0.3}\t{:0.3}",
                         a, m, z, x, params.root_width(), params.tip_diameter());
            }
        }
    }
}

fn demo_toothface_mill_offset() {
    let module:f64 = 2.0;
    let num_teeth:f64 = 50 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.0;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
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
    let module:f64 = 2.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
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
    let mirrored: ToothFace = face.mirror_about_x_and_reverse_order();
    let rotated: ToothFace = mirrored.rotate(&center, angle);
    println!("{:?}", face);
    println!("{:?}", mirrored);
    println!("{:?}", rotated);
}

fn example_gear_1() {
    let module:f64 = 2.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    gear.params.print_params();
    gear.print_profile(20);
}

fn example_gear_2() {
    let module:f64 = 2.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    gear.params.print_params();
    gear.print_profile(20);
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
