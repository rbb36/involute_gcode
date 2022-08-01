use std::f64::consts::PI;

use raqote::*;

pub const BLACK:SolidSource = SolidSource {r:0x0, g:0x0, b:0x0, a:0xff};
pub const WHITE:SolidSource = SolidSource {r:0xff, g:0xff, b:0xff, a:0xff};

pub fn draw_something(out_path: &str) {
    let SOLID:StrokeStyle = StrokeStyle {
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
        &SOLID,
        &DrawOptions::new()
    );
    dt.write_png("example.png");

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
