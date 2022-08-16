use crate::geometry;
use crate::geometry::{Arc, Circle, Point};
use crate::involute::tooth_face::ToothFace;

pub fn get_arc(p1:&Point, p2:&Point, p3:&Point) -> Arc {
    let circle:Circle = geometry::circle_from_points(p1, p2, p3);
    let start_angle = circle.center.angle_to(p1);
    let end_angle = circle.center.angle_to(p3);
    let included_angle = end_angle - start_angle;
    Arc{circle, start_angle, included_angle}
}

pub fn get_tooth_face_arcs(face:&ToothFace) -> Vec<Arc> {
    let mut arcs:Vec<Arc> = Vec::new();

    for i in 0..(face.points.len() / 2) {
        let p1:&Point = face.points.get(i * 2).unwrap();
        let p2:&Point = face.points.get(i * 2 + 1).unwrap();
        let p3:&Point = face.points.get(i * 2 + 2).unwrap();
        let arc:Arc = get_arc(p1, p2, p3);
        arcs.push(arc);
    }

    arcs
}

pub fn get_tooth_face_gcode(face:&ToothFace) -> Vec<String> {
    let mut gcodes:Vec<String> = Vec::new();

    let x = face.first().x;
    let y = face.first().y;
    gcodes.push(format!("G1 X{:.3} Y{:.3}", x, y));
    for i in 0..(face.points.len() / 2) {
        println!("{}", (i * 2));
        let p1:&Point = face.points.get(i * 2).unwrap();
        let p2:&Point = face.points.get(i * 2 + 1).unwrap();
        let p3:&Point = face.points.get(i * 2 + 2).unwrap();
        let arc:Arc = get_arc(p1, p2, p3);
        // let circ:Circle = geometry::circle_from_points(p1, p2, p3);
        // let arc:Arc = Arc{circle:circ.copy(), start:p1.copy(), end:p2.copy()};
        // println!("{:?}", circ);
        let i = arc.gcode_i();
        let j = arc.gcode_j();
        gcodes.push(format!("G2 X{:.3} Y{:.3} I{:.3} J{:.3}", p3.x, p3.y, i, j));
    }
    // for point in &face.points {
    //     gcodes.push(format!("{:?}", point));
    // }
    // gcodes.push("g0x0y0z0".to_string());

    gcodes
}
