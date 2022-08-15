use crate::geometry::Arc;

pub fn get_gcode_for_arc(arc:&Arc) -> Vec<String> {
    let mut gcodes:Vec<String> = Vec::new();
    // gcodes.push(format!("G1 X{:.3} Y{:.3}", arc.start.x, arc.start.y));
    gcodes.push(format!("G2 X{:.3} Y{:.3} I{:.3} J{:.3}",
                        arc.end.x, arc.end.y, arc.gcode_j(), arc.gcode_i()));
    gcodes
}