use std::f64::consts::PI;

use crate::geometry::{Arc, Point};

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
        println!("{:?} os{:?} {:.3} {:.3}", point, os_point, x, y);
        Point{x, y}
    }
}


pub fn draw_arcs(dt:&mut DrawTarget, arcs:&Vec<Arc>, offset:&Point, scale:f64, line_width:f64, width:i32, height:i32) {
    let mut white:bool = true;
    let os:OffsetScaler = OffsetScaler{offset:offset.copy(), scale};
    let fixer:Fixer = Fixer{os, width, height};
    let mut index:u32 = 0;
    for arc in arcs {
        println!("{:?}", arc);
        let mut pb = PathBuilder::new();
        // let start = fixer.fix(&arc.start);
        // let end = fixer.fix(&arc.end);
        // let center = fixer.fix(&arc.circle.center);
        // center x, center y, radius, start angle, included angle
        println!("fixing center");
        let arc_c:Point = fixer.fix(&arc.circle.center);
        let radius:f32 = (arc.circle.radius * fixer.os.scale) as f32;
        println!("arc: {:.3} {:.3} {:.3} {:.3} {:.3}",
                 arc_c.x, arc_c.y, radius,
                 arc.start_angle, arc.included_angle);
        // pb.move_to(1000.0, 1000.0);
        pb.arc(arc_c.x as f32, arc_c.y as f32, radius as f32,
               (-1.0 * arc.start_angle) as f32,
               (-1.0 * arc.included_angle) as f32);
        // pb.line_to(1000.0, 1000.0);
        // pb.line_to(arc_c.x as f32, arc_c.y as f32);
        pb.close();
        let path = pb.finish();
        let val:u32 = index * 128 / (arcs.len() as u32);
        let mut color: SolidSource =
            SolidSource {r:val as u8, g:val as u8, b:0x66 as u8, a:0xff};
        // if line_width < 3.0 { color = BLUE; }
        if ! white {
            color = RED;
            // if line_width < 3.0 { color = BLACK; }
        }
        dt.stroke(&path, &Source::Solid(color),
                  &solid_stroke(line_width), &DrawOptions::new());
        white = ! white;
        index += 1;
    }
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

pub fn draw_arc(dt:&mut DrawTarget) {
    let solid_stroke:StrokeStyle = StrokeStyle {
        cap: LineCap::Round, join: LineJoin::Round,
        width: 4.0, miter_limit: 4.0,
        dash_array: vec![1.0], dash_offset: 0.0,
    };
    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 0.0);
    pb.move_to(160.0, 190.0);
    // center x, center y, radius, start angle, included angle
    pb.arc(160.0, 190.0, 180.0, -0.25 * PI as f32, -0.5 * PI as f32);
    let path = pb.finish();
    dt.push_clip(&path);
    dt.stroke(&path, &Source::Solid(WHITE), &solid_stroke, &DrawOptions::new());
}

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
