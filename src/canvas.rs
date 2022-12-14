use std::f64::consts::PI;

use crate::{
    geometry::{Arc, Point},
    involute::arc_interpolate,
    involute::tooth_face::ToothFace,
};

use raqote::{
    PathBuilder,
    SolidSource,
    LineCap,
    LineJoin,
    StrokeStyle,
    Source,
    DrawOptions,
    DrawTarget,
};

pub const BLACK:SolidSource = SolidSource {r:0x0, g:0x0, b:0x0, a:0xff};
pub const RED:SolidSource = SolidSource {r:0xff, g:0x0, b:0x0, a:0xff};
pub const WHITE:SolidSource = SolidSource {r:0xff, g:0xff, b:0xff, a:0xff};
pub const BLUE:SolidSource = SolidSource {r:0x00, g:0x00, b:0xff, a:0xff};

pub const ORIGIN:Point = Point{x:0.0, y:0.0};

pub fn solid_stroke(width:f64) -> StrokeStyle {
    StrokeStyle {
        cap: LineCap::Square, join: LineJoin::Round,
        width: width as f32, miter_limit: 1.0,
        dash_array: vec![1.0], dash_offset: 0.0,
    }
}

pub struct OffsetScaler {
    pub offset: Point,
    pub scale: f64
}
impl OffsetScaler {
    // offset first, then scale
    pub fn fix(&self, point:&Point) -> Point {
        Point{x:(point.x + self.offset.x) * self.scale,
              y:(point.y + self.offset.y) * self.scale}
    }
}

pub struct Fixer {
    pub os:OffsetScaler,
    pub width:i32,
    pub height:i32,
}
impl Fixer {
    pub fn fix(&self, point:&Point) -> Point {
        let os_point = self.os.fix(point);
        let x = os_point.x + self.width as f64 / 2.0;
        let y = -1.0 * os_point.y + self.height as f64 / 2.0;
        // println!("{:?} os{:?} {:.3} {:.3}", point, os_point, x, y);
        Point{x, y}
    }
}

pub fn make_fixer(offset:&Point, scale:f64, width:i32, height:i32) -> Fixer {
    let os:OffsetScaler = OffsetScaler{offset:offset.copy(), scale};
    Fixer{os, width, height}
}

pub fn get_svg(first_face:&ToothFace,
               second_face:&ToothFace,
               fixer:&Fixer,
               num_teeth:u8) -> String {
    let mut svg:String = String::new();
    for i in 0..num_teeth {
        let angle = (-1.0 * i as f64 / num_teeth as f64) * 2.0 * PI;
        // println!("{:.3}", angle);
        let f1:ToothFace = first_face.rotate(&ORIGIN, angle);
        let f2:ToothFace = second_face.rotate(&ORIGIN, angle);
        let start_x:f64 = fixer.fix(f1.first()).x;
        let start_y:f64 = fixer.fix(f1.first()).y;
        if i > 0 {
            svg.push_str(format!("   L{:.3} {:.3}", start_x, start_y).as_str());
        } else {
            svg.push_str(format!("   M{:.3} {:.3}", start_x, start_y).as_str());
        }
        svg.push_str("\n");
        svg.push_str(get_svg_for_tooth(&f1, &f2, fixer).as_str());
    }
    // let m = format!("M{:.3} {:.3}", start.x, start.y);
    // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
    //
    // format!("<path d=\"{}{}\" stroke=\"black\" stroke-width=\"1\" fill-opacity=\"0.0\" />", m, a)
    svg
}

pub fn get_svg_for_tooth(first_face:&ToothFace,
                         second_face:&ToothFace,
                         fixer:&Fixer) -> String {
    let first_arcs:Vec<Arc> = arc_interpolate::get_tooth_face_arcs(&first_face);
    let root_arc:Arc = first_face.root_arc_to(&second_face);
    let second_arcs:Vec<Arc> = arc_interpolate::get_tooth_face_arcs(&second_face);
    let mut svg:String = String::new();
    for arc in first_arcs {
        svg.push_str(&get_svg_for_arc(&arc, fixer));
        svg.push_str("\n");
    }
    svg.push_str(&get_svg_for_arc(&root_arc, fixer));
    svg.push_str("\n");
    for arc in second_arcs {
        svg.push_str(&get_svg_for_arc(&arc, fixer));
        svg.push_str("\n");
    }
    return svg;
}
pub fn get_svg_for_arc(arc:&Arc, fixer:&Fixer) -> String {
    let mut sweep_flag:u8 = 0;
    println!("{:.3}", arc.included_angle);
    if arc.included_angle < 0.0 { sweep_flag = 1; }
    let mut large_arc_flag:u8 = 0;
    if arc.included_angle.abs() > PI { large_arc_flag = 1; }
    // let start = fixer.fix(&arc.start());
    let end = fixer.fix(&arc.end());
    let scale = fixer.os.scale;
    format!("    A{:.3} {:.3} {} {} {} {:.3} {:.3}",
            arc.circle.radius * scale, arc.circle.radius * scale,
            0, large_arc_flag, sweep_flag,
            end.x, end.y)
}

pub fn draw_arcs(dt:&mut DrawTarget, arcs:&Vec<Arc>, fixer:&Fixer, line_width:f64) {
    let mut white:bool = true;
    let mut index:u32 = 0;
    for arc in arcs {
        let val:u32 = index * 128 / (arcs.len() as u32);
        draw_arc(dt, arc, fixer, line_width, white, val);
        white = ! white;
        index += 1;
    }
}

pub fn draw_arc(dt:&mut DrawTarget, arc:&Arc, fixer:&Fixer, line_width:f64, white:bool, val:u32) {
    let mut pb = PathBuilder::new();
    let arc_c:Point = fixer.fix(&arc.circle.center);
    let radius:f32 = (arc.circle.radius * fixer.os.scale) as f32;
    // println!("{:.3} {:.3} {:.3}", arc_c.x, arc_c.y, radius);
    pb.arc(arc_c.x as f32, arc_c.y as f32, radius as f32,
           (-1.0 * arc.start_angle) as f32,
           (-1.0 * arc.included_angle) as f32);
    pb.close();
    let path = pb.finish();
    let mut color: SolidSource =
        SolidSource {r:val as u8, g:val as u8, b:0x66 as u8, a:0xff};
    if ! white { color = RED; }
    dt.stroke(&path, &Source::Solid(color),
              &solid_stroke(line_width), &DrawOptions::new());
}

pub fn white(dt:&mut DrawTarget, width:i32, height:i32) {
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    pb.line_to(width as f32, 0.0);
    pb.line_to(width as f32, height as f32);
    pb.line_to(0.0, height as f32);
    pb.line_to(0.0, 0.0);
    pb.close();
    let path = pb.finish();
    dt.fill(&path, &Source::Solid(WHITE), &DrawOptions::new());
}

// pub fn draw_arc(dt:&mut DrawTarget) {
//     let solid_stroke:StrokeStyle = StrokeStyle {
//         cap: LineCap::Round, join: LineJoin::Round,
//         width: 4.0, miter_limit: 4.0,
//         dash_array: vec![1.0], dash_offset: 0.0,
//     };
//     let mut pb = PathBuilder::new();
//     pb.move_to(0.0, 0.0);
//     pb.move_to(160.0, 190.0);
//     // center x, center y, radius, start angle, included angle
//     pb.arc(160.0, 190.0, 180.0, -0.25 * PI as f32, -0.5 * PI as f32);
//     let path = pb.finish();
//     dt.push_clip(&path);
//     dt.stroke(&path, &Source::Solid(WHITE), &solid_stroke, &DrawOptions::new());
// }

pub fn draw_something(out_path: &str) {
    let solid_stroke:StrokeStyle = StrokeStyle {
        cap: LineCap::Round, join: LineJoin::Round,
        width: 4.0, miter_limit: 4.0,
        dash_array: vec![1.0], dash_offset: 0.0,
    };

    let mut dt = DrawTarget::new(400, 400);

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    pb.line_to(399.0, 0.0);
    // pb.line_to(399.0, 399.0);
    pb.line_to(340.0, 190.0);
    pb.arc(160.0, 190.0, 180.0, 0.0, 2.0 * PI as f32);
    pb.close();
    let path = pb.finish();
    dt.push_clip(&path);
    
    dt.stroke(
        &path,
        &Source::Solid(WHITE),
        &solid_stroke,
        &DrawOptions::new()
    );
    dt.write_png(out_path).unwrap();

    // let mut pb = PathBuilder::new();
    // pb.move_to(100., 10.);
    // pb.cubic_to(150., 40., 175., 0., 200., 10.);
    // pb.quad_to(120., 100., 80., 200.);
    // pb.quad_to(150., 180., 300., 300.);
    // pb.close();
    // let path = pb.finish();

    // let gradient = Source::new_radial_gradient(
    //     Gradient {
    //         stops: vec![
    //             GradientStop {
    //                 position: 0.2,
    //                 color: Color::new(0xff, 0, 0xff, 0),
    //             },
    //             GradientStop {
    //                 position: 0.8,
    //                 color: Color::new(0xff, 0xff, 0xff, 0xff),
    //             },
    //             GradientStop {
    //                 position: 1.,
    //                 color: Color::new(0xff, 0xff, 0, 0xff),
    //             },
    //         ],
    //     },
    //     Point::new(150., 150.),
    //     128.,
    //     Spread::Pad,
    // );
    // dt.fill(&path, &gradient, &DrawOptions::new());

    // let mut pb = PathBuilder::new();
    // pb.move_to(100., 100.);
    // pb.line_to(300., 300.);
    // pb.line_to(200., 300.);
    // let path = pb.finish();
}
