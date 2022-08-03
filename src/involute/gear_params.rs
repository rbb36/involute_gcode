use std::f64::consts::PI;

use crate::geometry::{Line, Point, PRECISION};

#[derive(Debug)]
#[allow(dead_code)]
pub struct GearParams {
    pub module: f64, // (aka: m)
    pub num_teeth: f64, // (aka: z)
    pub pressure_angle_degrees: f64, // (aka: a)
    pub profile_shift: f64, // (aka: x, roughly: (z8 =~ .4, z16+ = 0))
}
impl GearParams {
    pub fn print_params(&self) {
        println!("module: {}", self.module);
        println!("num_teeth: {}", self.num_teeth);
        println!("pressure_angle_degrees: {}", self.pressure_angle_degrees);
        println!("profile_shift: {}", self.profile_shift);
        println!("pressure_angle: {}", self.pressure_angle());
        println!("ref_diameter: {}", self.ref_diameter());
        println!("pitch_diameter: {}", self.pitch_diameter());
        println!("pitch_radius: {}", self.pitch_radius());
        println!("base_diameter: {}", self.base_diameter());
        println!("base_radius: {}", self.base_radius());
        println!("tip_diameter: {}", self.tip_diameter());
        println!("tip_radius: {}", self.tip_radius());
        println!("tip_pressure_angle: {}", self.tip_pressure_angle());
        println!("involute: {}", self.involute());
        println!("tip_involute: {}", self.tip_involute());
        println!("tip_thickness: {}", self.tip_thickness());
        println!("tip_radius: {}", self.tip_radius());
        println!("base_radius: {}", self.base_radius());
        println!("u_max: {}", self.u_max());
        println!("base thickness: {}", self.base_thickness());
        println!("base degrees: {}", self.base_thickness() * 180.0 / PI);
        println!("root width: {}", self.root_width());
    }
    pub fn pressure_angle(&self) -> f64 {
        self.pressure_angle_degrees * PI / 180.0
    }
    pub fn ref_diameter(&self) -> f64 {
        self.module * self.num_teeth
    }
    pub fn ref_radius(&self) -> f64 {
        self.ref_diameter() / 2.0
    }
    pub fn pitch_radius(&self) -> f64 {
        // center spacing increases by module (m) * shift (x)
        let v = self.module * self.profile_shift;
        self.ref_radius() + v
    }
    pub fn pitch_diameter(&self) -> f64 {
        // diameter is increased by 2x spacing increase
        self.pitch_radius() * 2.0
    }
    pub fn base_diameter(&self) -> f64 {
        self.ref_diameter() * self.pressure_angle().cos()
    }
    pub fn base_radius(&self) -> f64 {
        self.base_diameter() / 2.0
    }
    pub fn tip_radius(&self) -> f64 {
        let shift_ratio: f64 = 1.0 + self.profile_shift;
        let shift_amount: f64 = self.module * shift_ratio;
        self.ref_radius() + shift_amount
    }
    pub fn tip_diameter(&self) -> f64 {
        self.tip_radius() * 2.0
    }
    pub fn tip_pressure_angle(&self) -> f64 {
        (self.base_diameter() / self.tip_diameter()).acos()
    }
    pub fn involute(&self) -> f64 {
        self.pressure_angle().tan() - self.pressure_angle()
    }
    pub fn tip_involute(&self) -> f64 {
        self.tip_pressure_angle().tan() - self.tip_pressure_angle()
    }
    pub fn tip_thickness(&self) -> f64 {
        let half_tooth_angle = (PI / 2.0) / self.num_teeth;
        let shift_correct = (2.0 * self.profile_shift * self.pressure_angle().tan()) / self.num_teeth;
        let pitch_to_tip_angle = self.involute() - self.tip_involute();
        2.0 * (half_tooth_angle + shift_correct + pitch_to_tip_angle)
    }
    pub fn u_max(&self) -> f64 {
        let tip_r_sq: f64 = self.tip_radius().powi(2);
        let base_r_sq: f64 = self.base_radius().powi(2);
        ((tip_r_sq / base_r_sq) - 1.0).sqrt()
    }
    pub fn tooth_profile_x(&self, u: f64) -> f64 {
        self.base_radius() * (u.cos() + u * u.sin())
    }
    pub fn tooth_profile_y(&self, u: f64) -> f64 {
        self.base_radius() * (u.sin() - u * u.cos())
    }
    // included angle of the base of the tooth in radians
    pub fn base_thickness(&self) -> f64 {
        let umax = self.u_max();
        let base_r = self.base_radius();
        let tip_r = self.tip_radius();
        let base_x = base_r;
        let base_y = 0.0;
        let tip_x = self.tooth_profile_x(umax);
        let tip_y = self.tooth_profile_y(umax);
        let base_to_tip = ((base_x - tip_x).powi(2) + (base_y - tip_y).powi(2)).sqrt();
        let thick_cos = (base_r.powi(2) + tip_r.powi(2) - base_to_tip.powi(2))
            / (2.0 * base_r * tip_r);
        self.tip_thickness() + 2.0 * thick_cos.acos()
    }
    // returns the space the mill has to fit into
    // Note: if this is too tight, you won't have room
    // for a rough pass before the final pass.
    pub fn root_width(&self) -> f64 {
        // let base_c = self.base_diameter() * PI;
        let tooth_base_angle = self.base_thickness();
        let total_tooth_angle = tooth_base_angle * self.num_teeth;
        let remaining_angle = (2.0 * PI) - total_tooth_angle;
        let root_angle = remaining_angle / self.num_teeth;
        let half_angle = root_angle / 2.0;
        let root_width = 2.0 * half_angle.sin() * self.base_radius();
        return root_width;
    }
}


impl PartialEq for GearParams {
    fn eq(&self, other: &Self) -> bool {
        let smc: i64 = (self.module * PRECISION).round() as i64;
        let omc: i64 = (other.module * PRECISION).round() as i64;
        let szc: i64 = (self.num_teeth * PRECISION).round() as i64;
        let ozc: i64 = (other.num_teeth * PRECISION).round() as i64;
        let sac: i64 = (self.pressure_angle_degrees * PRECISION).round() as i64;
        let oac: i64 = (other.pressure_angle_degrees * PRECISION).round() as i64;
        let sxc: i64 = (self.profile_shift * PRECISION).round() as i64;
        let oxc: i64 = (other.profile_shift * PRECISION).round() as i64;
        smc == omc && szc == ozc && sac == oac && sxc == oxc
    }
}
