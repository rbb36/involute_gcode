use std::f64::consts::PI;

use crate::involute::Gear;

// const ORIGIN: Point = Point{x:0.0, y:0.0};
const PRECISION: f64 = 1000.0;

#[cfg(test)]
mod gear_tests {
    use super::*; // needed to access crate imports
    #[test]
    fn _gear_tests_are_running() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn equal() {
        let base: Gear = Gear{ module:1.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        let gsm: Gear = Gear{ module:1.00000000001,
                              num_teeth: 18.0,
                              pressure_angle_degrees:20.0,
                              profile_shift:0.0 };
        let gdm: Gear = Gear{ module:1.1,
                              num_teeth: 18.0,
                              pressure_angle_degrees:20.0,
                              profile_shift:0.0 };
        let gsz: Gear = Gear{ module:1.0,
                              num_teeth:18.00000000001,
                              pressure_angle_degrees:20.0,
                              profile_shift:0.0 };
        let gdz: Gear = Gear{ module:1.0,
                              num_teeth:18.1,
                              pressure_angle_degrees:20.0,
                              profile_shift:0.0 };
        let gsa: Gear = Gear{ module:1.0,
                              num_teeth:18.0,
                              pressure_angle_degrees:20.000000000001,
                              profile_shift:0.0 };
        let gda: Gear = Gear{ module:1.0,
                              num_teeth:18.0,
                              pressure_angle_degrees:20.1,
                              profile_shift:0.0 };
        let gsx: Gear = Gear{ module:1.0,
                              num_teeth:18.0,
                              pressure_angle_degrees:20.0,
                              profile_shift:0.00000000000001 };
        let gdx: Gear = Gear{ module:1.0,
                              num_teeth:18.0,
                              pressure_angle_degrees:20.0,
                              profile_shift:0.1 };
        assert_eq!(base, gsm);
        assert_eq!(base, gsz);
        assert_eq!(base, gsa);
        assert_eq!(base, gsx);
        assert_ne!(base, gdm);
        assert_ne!(base, gdz);
        assert_ne!(base, gda);
        assert_ne!(base, gdx);
    }
    #[test]
    fn pressure_angle() {
        let base: Gear = Gear{ module:1.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:90.0,
                               profile_shift:0.0 };
        assert_eq!(PI / 2.0, base.pressure_angle());
    }
    #[test]
    fn ref_diameter() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        assert_eq!(36.0, base.ref_diameter());
    }
    #[test]
    fn pitch_radius() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:10.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.5 };
        assert_eq!(11.0, base.pitch_radius());
    }
    #[test]
    fn pitch_diameter() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:10.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.5 };
        assert_eq!(22.0, base.pitch_diameter());
    }
    #[test]
    fn base_diameter() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        let actual: f64 = base.base_diameter();
        let expected: f64 = 2.0 * 18.0 * base.pressure_angle().cos();
        assert_eq!(expected, actual);
    }
    #[test]
    fn base_radius() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        let actual: f64 = base.base_radius();
        let expected: f64 = base.base_diameter() / 2.0;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn tip_radius() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        let shift_ratio: f64 = 1.0 + base.profile_shift;
        let shift_amount: f64 = base.module * shift_ratio;
        let expected: f64 = base.ref_radius() + shift_amount;
        let actual: f64 = base.tip_radius();
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn tip_diameter() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        assert_eq!(base.tip_radius() * 2.0, base.tip_diameter());
    }
    #[test]
    pub fn tip_pressure_angle() {
        let base: Gear = Gear{ module:2.0,
                               num_teeth:18.0,
                               pressure_angle_degrees:20.0,
                               profile_shift:0.0 };
        let expected: f64 =
            (base.base_diameter() / base.tip_diameter()).acos();
        assert_eq!(expected, base.tip_pressure_angle());
    }
    #[test]
    pub fn involute() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 0.01490438;
        let actual = base.involute();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn tip_involute() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 0.068;
        let actual = base.tip_involute();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn shifted_tip_involute() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 10.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.4 };
        let expected: f64 = 0.178;
        let actual = base.tip_involute();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn tip_thickness() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 0.068;
        let actual = base.tip_thickness();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn shifted_tip_thickness() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 10.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.4 };
        let expected: f64 = 0.045;
        let actual = base.tip_thickness();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn u_max() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 0.631;
        let actual = base.u_max();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn shifted_u_max() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 10.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.4 };
        let expected: f64 = 0.925;
        let actual = base.u_max();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn base_thickness() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 0.204;
        let actual = base.base_thickness();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn shifted_base_thickness() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 10.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.4 };
        let expected: f64 = 0.402;
        let actual = base.base_thickness();
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn tooth_profile_x() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 9.977;
        let umax: f64 = base.u_max();
        let actual = base.tooth_profile_x(umax);
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn shifted_tooth_profile_x() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 10.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.4 };
        let expected: f64 = 6.298;
        let umax: f64 = base.u_max();
        let actual = base.tooth_profile_x(umax);
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn tooth_profile_y() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 18.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.0 };
        let expected: f64 = 0.680;
        let umax: f64 = base.u_max();
        let actual = base.tooth_profile_y(umax);
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
    #[test]
    pub fn shifted_tooth_profile_y() {
        let base: Gear = Gear{ module: 1.0,
                               num_teeth: 10.0,
                               pressure_angle_degrees: 20.0,
                               profile_shift: 0.4 };
        let expected: f64 = 1.136;
        let umax: f64 = base.u_max();
        let actual = base.tooth_profile_y(umax);
        assert_eq!((PRECISION * expected).round(),
                   (PRECISION * actual).round());
    }
}
