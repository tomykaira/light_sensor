$fn = 100;
wallThickness = 1.4;
inner_wall_thickness = 1.4;
inner_wall_offset = 3;
inner_wall_height = 8;
hingeOuter = 7;
hingeInner = 4;
hingeInnerSlop = .4;
hingeFingerSlop = .4;
fingerLength = hingeOuter/1.65;
fingerSize = 6.5;
latchWidth = 20;
z = 0;

bat_w = 54;
bat_d = 50;
bat_h = 12.5;

margin = 2;
d_margin = 0.3;

bread_d = 45 + margin;
bread_w = 83.6 + margin;
bread_h = 22.2;
sensor_sensor_d = 11.9 + margin;
sensor_sensor_w = 21 + margin;
sensor_body_w = 27.8 + margin;
sensor_body_d = 13.9 + margin;
sensor_offset_w = wallThickness + 20; // 20 is arbitrary
sensor_offset_d = wallThickness + 20; // 20 is arbitrary
sensor_h = 5.9;

prop_dia = 7;
prop_offset = 5;

outer_wall = 2;
wall_to_servo_lower = 10;
sw_panel_h_max = 8.6;
sw_panel_h_min = 5.6;

wiring_space = 5;

width = bread_w + wallThickness * 2;
depth = 22.7 + bread_d + bat_d + inner_wall_thickness * 2 + wallThickness * 2 ;
inner_height = bread_h + sensor_h;
height = (inner_height + wallThickness * 2) / 2;


topFingerSize = fingerSize;
pos = -depth/2;
mount_d = 20;

bottom();
top();

module bottom() {
	union() {
		// main box and cutout
		difference() {
			translate([-width - fingerLength, -depth/2, 0]) {
				bottom_content();
			}

			// latch cutout
			translate([-width - fingerLength + (wallThickness/2), (-latchWidth/2) - (hingeFingerSlop/2), wallThickness]) {
				cube([wallThickness/2 + .1, latchWidth + hingeFingerSlop, height]);
			}


		}

		//latch cylinder
		difference() {
			translate([-width - fingerLength + (wallThickness/2), -latchWidth/2, height - 1]) {
				rotate([-90,0,0]) {
					cylinder(r = 1, h = latchWidth);
				}
			}
			// front wall wipe
			translate([-width - fingerLength - 5, -depth/2,0]) {
				cube([5,depth,height]);
			}
		}

		difference() {
			hull() {
				translate([0,-depth/2,height]) {
					rotate([-90,0,0]) {
						cylinder(r = hingeOuter/2, h = depth);
					}
				}
				translate([-fingerLength - .1, -depth/2,height - hingeOuter]){
					cube([.1,depth,hingeOuter]);
				}
				translate([-fingerLength, -depth/2,height-.1]){
					cube([fingerLength,depth,.1]);
				}
				translate([0, -depth/2,height]){
					rotate([0,45,0]) {
						cube([hingeOuter/2,depth,.01]);
					}
				}
			}
			// finger cutouts

			for  (i = [-depth/2 + fingerSize:fingerSize*2:depth/2]) {
				translate([-fingerLength,i - (fingerSize/2) - (hingeFingerSlop/2),0]) {
					cube([fingerLength*2,fingerSize + hingeFingerSlop,height*2]);
				}
			}
		}

		// center rod
		translate([0, -depth/2, height]) {
			rotate([-90,0,0]) {
				cylinder(r = hingeInner /2, h = depth);
			}
		}
	}
}

module top() {
	union() {
		top_content();

		//latch
		translate([width + fingerLength - wallThickness - 1.5, (-latchWidth/2), 0]) {
			cube([1.5, latchWidth, height - .5 + 4]);
		}
		translate([width + fingerLength - wallThickness, -latchWidth/2, height - .5 + 3]) {
			rotate([-90,0,0]) {
				cylinder(r = 1, h = latchWidth);
			}
		}

		difference() {
			hull() {
				translate([0,-depth/2,height]) {
					rotate([-90,0,0]) {
						cylinder(r = hingeOuter/2, h = depth);
					}
				}
				translate([fingerLength, -depth/2,height - hingeOuter - .5]){
					cube([.1,depth,hingeOuter - .5]);
				}
				translate([-fingerLength/2, -depth/2,height-.1]){
					cube([fingerLength,depth,.1]);
				}
				translate([0, -depth/2,height]){
					rotate([0,45,0]) {
						cube([hingeOuter/2,depth,.01]);
					}
				}
			}
			// finger cutouts
			for  (i = [-depth/2:fingerSize*2:depth/2 + fingerSize]) {
				translate([-fingerLength,i - (fingerSize/2) - (hingeFingerSlop/2),0]) {
					cube([fingerLength*2,fingerSize + hingeFingerSlop,height*2]);
				}
				if (depth/2 - i < (fingerSize * 1.5)) {
					translate([-fingerLength,i - (fingerSize/2) - (hingeFingerSlop/2),0]) {
						cube([fingerLength*2,depth,height*2]);
					}
				}
			}

			// center cutout
			translate([0, -depth/2, height]) {
				rotate([-90,0,0]) {
					cylinder(r = hingeInner /2 + hingeInnerSlop, h = depth);
				}
			}
		}
	}
}

// dimensions:
// http://www.arduinoos.com/2016/02/sg90-servo-part-1/
module servo() {
	union() {
		cube([22.5, 22.7, 11.8]);
		translate([-4.7, 15.9, 0]) {
			cube([4.7, 2.5, 11.8]);
		}
		translate([22.5, 15.9, 0]) {
			cube([4.7, 2.5, 11.8]);
		}
		translate([0, 22.7, 0]) {
			cube([5.9 + 8.8, 4.0, 11.8]);
		}
		translate([5.9 - 4.6/2, 22.7+4.0, 11.8/2 - 4.6/2]) {
			cube([4.6, 3.2, 4.6]);
		}
		// cable
		translate([- 10, 4.5 - 1.2 * 0.5 / 2, 0]) {
			// w=10 can be any value, * 1.5 is for space
			cube([10, 1.2 * 1.5, 11.8]);
		}
	}
}

servo_w_offset = width / 2 - 5.9;
servo_h_offset = depth - 22.7 - wallThickness;
module bottom_content() {
	difference() {
		union() {
			cube([width, wallThickness, height]);
			translate([0, depth - wallThickness, 0])
				cube([width, wallThickness, height]);

			cube([wallThickness, depth, height]);
			translate([width - wallThickness, 0, 0])
				cube([wallThickness, depth, height]);

			difference () {
				cube([width, depth, wallThickness]);
				translate([0, -depth/1.41, 0])
					mesh([0 : width / mesh_pitch * 3], [0 : depth / mesh_pitch * 3]);
			}

			// content area
			translate([wallThickness, wallThickness, wallThickness]) {
				union() {
					// separators
					translate([(width - bat_w) / 2 + inner_wall_thickness / 2, inner_wall_thickness, 0]) {
						separator(bat_w, bat_d);
					}
					translate([(width - bread_w) / 2 + inner_wall_thickness / 2, inner_wall_thickness * 2 + bat_d, 0]) {
						separator(bread_w, bread_d);
					}
					translate([servo_w_offset, servo_h_offset - margin / 2, 0]) {
						separator(22.5, 22.7);
					}
				}
			}

			// TOOD verify X position
			translate([(11.55), depth, 0]) {
				cube([13.25, mount_d ,height]);
			}
			translate([((11.55 + 13.25 + 43)), depth, 0]) {
				cube([13.25, mount_d ,height]);
			}
		}
		// servo window
		translate([servo_w_offset, servo_h_offset, wallThickness]) {
			union() {
				servo();
				translate([0, 22.7, 0]) {
					cube([5.9 + 8.8, 4.0, 30]); // extend height
				}

			}
		}
	}
}


module separator(w, d) {
	translate([-inner_wall_offset, -inner_wall_offset, 0]) {
		union() {
			translate([0, inner_wall_offset, 0]) {
				cube([inner_wall_thickness, d - inner_wall_offset * 2, inner_wall_height]);
			}
			translate([w + inner_wall_thickness, inner_wall_offset, 0]) {
				cube([inner_wall_thickness, d - inner_wall_offset * 2, inner_wall_height]);
			}
			translate([inner_wall_offset, 0, 0]) {
				cube([w - inner_wall_offset * 2, inner_wall_thickness,, inner_wall_height]);
			}
			translate([inner_wall_offset, d + inner_wall_thickness, 0]) {
				cube([w - inner_wall_offset * 2, inner_wall_thickness, inner_wall_height]);
			}
		}
	}
}

mesh_pitch = 5;
mesh_hole_size = 3;
module mesh(x_range, y_range) {
	for ( i = y_range )
	{
		for (j = x_range ) {
			rotate(45, [0, 0, 1])
				translate([j * mesh_pitch, i * mesh_pitch, 0])
				cube(mesh_hole_size, mesh_hole_size, wallThickness);
		}
	}
}

module top_content() {
	union() {
		difference() {

			translate([fingerLength, -depth/2, 0]) {
				cube([width,depth,height - .5]);
			}

			translate([fingerLength + wallThickness, -depth/2 + wallThickness, wallThickness]) {
				cube([width - (wallThickness * 2), depth - (wallThickness * 2), height]);
			}
			translate([sensor_offset_w, sensor_offset_d, 0]) {
				cube([sensor_sensor_w, sensor_sensor_d, wallThickness]);
			}
		}
	}
}
