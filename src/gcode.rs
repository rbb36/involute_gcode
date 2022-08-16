use crate::geometry::Arc;

pub fn get_gcode_for_arc(arc:&Arc) -> Vec<String> {
    let mut gcodes:Vec<String> = Vec::new();
    // gcodes.push(format!("G1 X{:.3} Y{:.3}", arc.start.x, arc.start.y));
    gcodes.push(format!("{} X{:.3} Y{:.3} I{:.3} J{:.3}", arc.gcode_g(),
                        arc.end().x, arc.end().y, arc.gcode_i(), arc.gcode_j()));
    gcodes
}
