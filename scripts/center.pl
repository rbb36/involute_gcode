#!/usr/bin/env perl

print "F30\n";
print "G0 Z10\n";
print "G0 X0 Y0\n";
print "G0 Z1\n";
print "G1 Z0.2\n";

# 2.5mm r = 0.5mm mill + 2.0mm offset
print "G1 X2 Y0\n";
foreach my $i (0..12) {
    my $z = 0.2 - $i * 2 / 10;
    print "G1 Z".$z."\n";
    print "G3 X0 Y-2 I-2 J0\n";
    print "G1 Z".($z - .05)."\n";
    print "G3 X-2 Y0 I0 J2\n";
    print "G1 Z".($z - .1)."\n";
    print "G3 X0 Y2 I2 J0\n";
    print "G1 Z".($z - .15)."\n";
    print "G3 X2 Y0 I0 J-2\n";
}
print "G3 X0 Y-2 I-2 J0\n";
print "G3 X-2 Y0 I0 J2\n";
print "G3 X0 Y2 I2 J0\n";
print "G3 X2 Y0 I0 J-2\n";

# 3.5mm d = 1.75mm r = 0.5mm mill + 1.25mm offset
# Drill d = 3.5mm = .1378"
