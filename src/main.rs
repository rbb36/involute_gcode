use std::f64::consts::PI;

use involute_gcode::{
    geometry::{Point}, //, Line},
    involute::gear_params::GearParams,
    involute::tooth_face::ToothFace,
    canvas,
};

use involute_gcode::linear_interpolated_demo;

fn main() {
    do_demos();
}

fn do_demos() {
    if false {
        linear_interpolated_demo::do_demos();
        println!();
        gear_catalog();
        println!();
        canvas::draw_something("example.png");
        println!();
        demo_point_translation();
        println!();
        demo_face_translation();
    }
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
