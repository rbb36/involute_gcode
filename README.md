# involute_gcode
Rust code for generating CNC mill G-code to cut involute gears.

To see a G-code sample:

$ cargo run

The example G-code is for a module 3 metric gear with a 20 degree pressure angle, 10 teeth, and 0.4 profile shift. The G-code has an offset of 1.2 millimeters from the final face, intended for a rough pass with a 2mm end mill.

To see the first cut, look at doc/first_test_cut.jpg
