let fs = require('fs');
let path = require('path');

let scene = {
  "time_0": 0.0,
  "time_1": 1.0,
  "camera": {
    "look_from": [
      // Look from side, and slightly above centerline
      30.0,
      10.0,
      0.0
    ],
    "look_at": [
      0.0,
      0.0,
      0.0
    ],
    // Tilt for the famous look
    "up": [
      -0.25,
      1.0,
      -0.25
    ],
    "vertical_fov": 40.0,
    "aperture": 0.0,
    "focus_distance": 10.0
  },
  "background_color": [
    0.0,
    0.0,
    0.0
  ],
  "objects": [],
  "priority_objects": [],
}

// Big light for smooth lighting
// Offset to the left 
let light = {
  "FlipFace": {
    "object": {
      "XZRect": {
        "x0": -100.0,
        "x1": 100.0,
        "z0": -100.0,
        "z1": 0.0,
        "k": 80.0,
        "material": {
          "DiffuseLight": {
            "emit": {
              "SolidColor": {
                "color": [
                  2.5,
                  2.5,
                  2.5
                ]
              }
            }
          }
        }
      }
    }
  }
};

scene.objects.push(light);
scene.priority_objects.push(light);


let radius = 8.0;

let sphere = {
  "Sphere": {
    // TODO: fix the camera setup, these coordinates are in weird order :|
    "center": [0.0, 0.0, 0.0],
    radius,
    "material": {
      "Lambertian": {
        "albedo": {
          "SurfaceChecker": {
            "even": [1.0, 0.0, 0.0],
            "odd": [1.0, 1.0, 1.0],
            "density": 12.0
          }
        }
      }
    }
  }
}
scene.objects.push(sphere);


let json = JSON.stringify(scene, null, 2);
fs.writeFileSync(path.join(__dirname, "boing.json"), json);