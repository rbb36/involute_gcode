use std::f64::consts::PI;

use involute_gcode::{
    gcode,
    geometry,
    geometry::{Arc, Point}, //, Line},
    involute::arc_interpolate,
    involute::gear::Gear,
    involute::gear_params::GearParams,
    involute::tooth_face::ToothFace,
    canvas,
};

use involute_gcode::linear_interpolated_demo;

fn main() {
    do_demos();
}

fn do_demos() {
    eight_tooth();
    if false {
        arc_demo();
        println!();
        demo_circle_from_points();
        println!();
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

fn eight_tooth() {
    let line_width = 2.0;
    let width = 1000 as i32;
    let height = 1000 as i32;
    
    let offset:Point = Point{x:0.0, y:0.0};
    let scale:f64 = 10.0;

    let module:f64 = 2.25;
    let num_teeth:f64 = 40 as f64;
    let pressure_angle_degrees:f64 = 10.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    let first_face: ToothFace = gear.tooth_face(10).reverse_order();
    let angle = gear.params.base_thickness() - (2.0 * PI / num_teeth);
    let second_face: ToothFace = first_face
        .mirror_about_x_and_reverse_order()
        .rotate(&Point{x:0.0,y:0.0}, angle);

    let mut dt:raqote::DrawTarget = raqote::DrawTarget::new(width, height);
    canvas::white(&mut dt, width, height);

    draw(&first_face, &second_face, line_width,
         &mut dt, width, height, &offset, scale);
}

fn draw(first_face:&ToothFace,
        second_face:&ToothFace,
        line_width:f64,
        dt:&mut raqote::DrawTarget,
        width:i32,
        height:i32,
        offset:&Point,
        scale:f64) {
    let root_arc:Arc = first_face.root_arc_to(&second_face);
    let root_arcs:Vec<Arc> = vec![root_arc];//Vec{root_arc};
    
    println!("G1 X{:.3} Y{:.3}", first_face.first().x, first_face.first().y);
    let arcs:Vec<Arc> = arc_interpolate::get_tooth_face_arcs(&first_face);
    for arc in &arcs {
        println!("{:?}", gcode::get_gcode_for_arc(&arc));
    }
    canvas::draw_arcs(dt, &arcs, &offset, scale, line_width, width, height);

    for arc in &root_arcs {
        println!("{:?}", gcode::get_gcode_for_arc(&arc));
    }
    canvas::draw_arcs(dt, &root_arcs, &offset, scale, line_width, width, height);
    
    let arcs:Vec<Arc> = arc_interpolate::get_tooth_face_arcs(&second_face);
    for arc in &arcs {
        println!("{:?}", gcode::get_gcode_for_arc(&arc));
    }
    canvas::draw_arcs(dt, &arcs, &offset, scale, line_width, width, height);

    dt.write_png("draw_arcs.png").unwrap();
}

fn arc_demo() {
    let module:f64 = 2.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: Gear = Gear{params};
    let face: ToothFace = gear.tooth_face(50);

    let arcs:Vec<Arc> = arc_interpolate::get_tooth_face_arcs(&face);
    for arc in &arcs {
        // println!("{:?}", arc);
        println!("{:?}", gcode::get_gcode_for_arc(&arc));
    }
    
    let mut dt:raqote::DrawTarget = raqote::DrawTarget::new(1000, 1000);
    let offset:Point = Point{x:-8.0, y:1.0};
    let scale:f64 = 200.0;
    canvas::draw_arcs(&mut dt, &arcs, &offset, scale, 4.0, 1000, 1000);

    let face: ToothFace = gear.tooth_face(4);
    let arcs:Vec<Arc> = arc_interpolate::get_tooth_face_arcs(&face);
    let offset:Point = Point{x:-8.0, y:1.0};
    let scale:f64 = 200.0;
    canvas::draw_arcs(&mut dt, &arcs, &offset, scale, 2.0, 1000, 1000);

    dt.write_png("draw_arcs.png").unwrap();
}

fn demo_circle_from_points() {
    println!("{:?}", geometry::circle_from_points(&Point{x:5.0, y:0.0},
                                                  &Point{x:0.0, y:5.0},
                                                  &Point{x:3.0, y:4.0}));
    
    println!("{:?}", geometry::circle_from_points(&Point{x:2.5, y:0.0},
                                                  &Point{x:1.5, y:2.0},
                                                  &Point{x:0.0, y:2.5}));
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
