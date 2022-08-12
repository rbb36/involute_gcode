use crate::{
    involute::linear_interpolated_gear,
    involute::linear_interpolated_gear::LinearInterpolatedGear,
    involute::gear_params::GearParams,
    involute::tooth_face::ToothFace,
};

pub fn do_demos() {
    if false {
        demo_gear_mill_offset_coords();
        println!();
        demo_gear_mill_offset_gcode();
        println!();
        demo_gear_mill();
        println!();
        demo_toothface_mill_offset();
        println!();
        demo_gear_face_01();
        println!();
        example_gear_1();
        println!();
        example_gear_2();
        println!();
    }
}

fn demo_gear_mill() {
    let module:f64 = 3.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
    gear.print_coords(10);
}

fn demo_gear_mill_offset_coords() {
    let module:f64 = 2.25; // 2.25 x 8 = 3mm mill
    let num_teeth:f64 = 40 as f64;
    let pressure_angle_degrees:f64 = 10.0;
    let profile_shift:f64 = 0.0;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
    let num_face_steps: u32 = 8;
    let mill_size:f64 = 0.0;
    let mill_offset:f64 = 0.0;
    let start_z:f64 = 0.0;
    let _final_depth:f64 = -2.95;
    let z_step:f64 = -0.2;
    let min_radius:f64 = 0.0;
    gear.print_coords(num_face_steps);
    println!();
    gear.print_one_layer_coords(num_face_steps,
                                mill_size,
                                mill_offset,
                                start_z,
                                start_z + z_step,
                                min_radius);
}

pub struct GcodeParams {
    pub num_face_steps: u32,
    pub mill_size:f64,
    pub mill_offset:f64,
    pub start_z:f64,
    pub final_z:f64,
    pub z_step:f64,
    pub tab_z:f64,
}

pub fn gcode_header(gear:&LinearInterpolatedGear,
                    gcode_params:&GcodeParams,
                    min_radius:f64) {
    gear.print_traverse_to_start_gcode(gcode_params.num_face_steps,
                                       gcode_params.mill_size,
                                       gcode_params.mill_offset,
                                       gcode_params.start_z,
                                       gcode_params.final_z + 0.05,
                                       gcode_params.z_step,
                                       min_radius);
}

fn demo_gear_mill_offset_gcode() {
    let module:f64 = 1.0;
    let num_teeth:f64 = 8 as f64;
    let pressure_angle_degrees:f64 = 10.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
    let num_face_steps: u32 = 10;
    let mill_size:f64 = 1.0;
    let mill_offset:f64 = 0.05;
    let start_z:f64 = 0.0;
    let final_depth:f64 = -2.95;
    let z_step:f64 = -0.2;
    let mut tab_z:f64 = final_depth + 0.5;
    let min_radius:f64 = 0.0;
    let gcode_params:GcodeParams = GcodeParams{
        num_face_steps, mill_size, mill_offset, start_z,
        z_step, final_z:-2.95, tab_z:-2.95,
    };
    gcode_header(&gear, &gcode_params, min_radius);
    // gear.print_traverse_to_start_gcode(num_face_steps,
    //                                    mill_size,
    //                                    mill_offset,
    //                                    start_z,
    //                                    final_depth,
    //                                    z_step,
    //                                    min_radius);
    let mut prev_z = start_z;
    println!("F60");
    let mut min_observed_radius:f64 = gear.params.tip_diameter();
    while prev_z > final_depth {
        let mut next_z = prev_z + z_step;
        if next_z < final_depth { next_z = final_depth; }
        let result:(bool,f64) = gear.print_one_layer_slope_gcode(num_face_steps,
                                                                mill_size,
                                                                mill_offset,
                                                                prev_z,
                                                                next_z,
                                                                tab_z,
                                                                min_radius);
        println!("G0 Z5");
        println!("M00 (stop for inspection)");
        println!("G0 Z1");
        println!();
        prev_z = next_z;
        if min_observed_radius > result.1 { min_observed_radius = result.1; }
    }
    gear.print_one_layer_slope_gcode(num_face_steps,
                                     mill_size,
                                     mill_offset,
                                     final_depth,
                                     final_depth,
                                     tab_z,
                                     min_radius);
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
                                     tab_z,
                                     min_radius);
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
                                     tab_z,
                                     min_radius);
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
                                     tab_z,
                                     min_radius);
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
                                     tab_z,
                                     min_radius);
    println!("G0 Z20");
    println!("M00 (stop for inspection)");
    println!("G0 Z1");
    println!();
}

fn demo_toothface_mill_offset() {
    let module:f64 = 2.0;
    let num_teeth:f64 = 50 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.0;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
    let faces: Vec<ToothFace> = gear.tooth_faces(20);
    let last: &ToothFace = faces.get(faces.len() - 1).unwrap();
    let zeroth: &ToothFace = faces.get(0).unwrap();
    let first: &ToothFace = faces.get(1).unwrap();
    let face: ToothFace = linear_interpolated_gear::mill_offset_tooth_face(zeroth, last, first, 2.0, 0.2);
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
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
    let faces: Vec<ToothFace> = gear.tooth_faces(20);
    for face_index in 0..faces.len() {
        faces.get(face_index).unwrap().print_coords();
    }
}

fn example_gear_1() {
    let module:f64 = 2.0;
    let num_teeth:f64 = 10 as f64;
    let pressure_angle_degrees:f64 = 20.0;
    let profile_shift:f64 = 0.4;
    let params:GearParams =
        GearParams{module, num_teeth, pressure_angle_degrees, profile_shift};
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
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
    let gear: LinearInterpolatedGear = LinearInterpolatedGear{params};
    gear.params.print_params();
    gear.print_profile(20);
}
