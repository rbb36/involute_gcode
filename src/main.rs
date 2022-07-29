use std::f64::consts::PI;

use involute_gcode::{
    geometry::{Point}, //, Line},
    involute::{Gear, ToothFace},
};

fn main() {
    demo_gear_mill();
    do_demos();
}

fn do_demos() {
    if false {
        demo_gear_mill_offset();
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
    // let faces: Vec<ToothFace> = gear.tooth_faces(10);
    let offset_faces: Vec<ToothFace> =
        gear.mill_offset(10, 2.0, 0.2);
    // for i in 0..faces.len() {
    //     faces.get(i).unwrap().print_coords();
    // }
    for i in 0..offset_faces.len() {
        println!("G1 Z-0.2");
        offset_faces.get(i).unwrap().print_gcode();
        println!("G1 Z5");
        println!("M00 (stop for inspection)");
        println!();
    }
}

fn demo_gear_mill_offset() {
    let gear: Gear = Gear{module:2.0,
                          num_teeth:10 as f64,
                          pressure_angle_degrees:20.0,
                          profile_shift:0.4};
    let faces: Vec<ToothFace> = gear.tooth_faces(10);
    let offset_faces: Vec<ToothFace> =
        gear.mill_offset(10, 2.0, 0.2);
    for i in 0..faces.len() {
        faces.get(i).unwrap().print_coords();
    }
    for i in 0..offset_faces.len() {
        offset_faces.get(i).unwrap().print_coords();
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