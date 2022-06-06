let fs = require("fs");
let path = require("path");

// Camera staring down from above, centered around origin
let scene = {
  time_0: 0.0,
  time_1: 1.0,
  camera: {
    look_from: [
      // TODO: fix the bad camera behavior with zero...
      1e-6, 30.0, 0.0,
    ],
    look_at: [0.0, 0.0, 0.0],
    up: [0.0, 1.0, 0.0],
    vertical_fov: 40.0,
    aperture: 0.0,
    focus_distance: 10.0,
  },
  background_color: [0.0, 0.0, 0.0],
  objects: [],
  priority_objects: [],
};

// Big light for smooth lighting of the entire scene
let light = {
  kind: "Quad",
  q: [-100.0, 80.0, -100.0],
  u: [200.0, 0.0, 0.0],
  v: [0.0, 0.0, 200.0],
  material: {
    DiffuseLight: {
      emit: {
        SolidColor: {
          color: [3.0, 3.0, 3.0],
        },
      },
    },
  },
};

scene.objects.push(light);
scene.priority_objects.push(light);

// Checkerboard ground
// The defaults should make this a unit square checkerboard
let ground = {
  kind: "Quad",
  q: [-10.0, 0.001, -10.0],
  u: [20.0, 0.0, 0.0],
  v: [0.0, 0.0, 20.0],
  material: {
    Lambertian: {
      albedo: {
        SpatialChecker: {},
      },
    },
  },
};

scene.objects.push(ground);

let radius = 1.5;
let height = radius;
for (var y = -2; y <= 2; y += 1) {
  for (var x = -2; x <= 2; x += 1) {
    // Tint the color a bit based on coord
    let color = [0.2 + 0.1 * x, 0.2 + 0.1 * y, 0.2 + 0.1 * height];
    // Default sphere
    let sphere = {
      kind: "Sphere",
      // TODO: fix the camera setup, these coordinates are in weird order :|
      center: [y * 3.0, radius * 1.0, x * 3.0],
      radius: radius,
    };
    // First row: Lambertian with checker
    if (y == -2) {
      sphere["material"] = {
        Lambertian: {
          albedo: {
            SurfaceChecker: {
              even: color,
              odd: [color[0] / 2.0, color[1] / 2.0, color[2] / 2.0],
            },
          },
        },
      };
    }
    // Second row: Lambertian solid color
    else if (y == -1) {
      sphere["material"] = {
        Lambertian: {
          albedo: {
            SolidColor: {
              color,
            },
          },
        },
      };
    }
    // Third row: Metal
    else if (y == 0) {
      sphere["material"] = {
        Metal: {
          albedo: {
            SolidColor: {
              color,
            },
          },
          // Start with no fuzz, increase based on x. Dodge the negative index.
          fuzz: 0.0 + 0.1 * (2 + x),
        },
      };
    }
    // Fourth row: Dielectric
    else if (y == 1) {
      sphere["material"] = {
        Dielectric: {
          // brighter color for the glass spheres
          color: [color[0] + 0.5, color[1] + 0.5, color[2] + 0.5],
          refractive_index: 1.5,
        },
      };
    }
    // Fifth row: ConstantMedium
    else if (y == 2) {
      sphere = {
        // TODO: this is a weird override because ConstantMedium is an object, not a material by its own
        kind: "ConstantMedium",
        boundary: {
          ...sphere,
        },
        texture: {
          SolidColor: {
            color,
          },
        },
        // Start with high density, lower it
        density: 1.0 - 0.2 * (2 + x),
      };
    }
    // Default back to Lambertian with color tinting
    else {
      sphere["material"] = {
        Lambertian: {
          albedo: {
            SolidColor: {
              color,
            },
          },
        },
      };
    }
    // Save the sphere to the objects list
    scene.objects.push(sphere);
  }
}

let json = JSON.stringify(scene);
fs.writeFileSync(path.join(__dirname, "debug_scene.json"), json);
