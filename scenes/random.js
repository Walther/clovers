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
      0.0,
      1.0,
      0.0
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
let light = {
  "FlipFace": {
    "object": {
      "XZRect": {
        "x0": -100.0,
        "x1": 100.0,
        "z0": -100.0,
        "z1": 100.0,
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

// Checkerboard ground
// The defaults should make this a unit square checkerboard
let ground = {
  "XZRect": {
    "x0": -1000.0,
    "x1": 1000.0,
    "z0": -1000.0,
    "z1": 1000.0,
    "k": 0.001,
    "material": {
      "Lambertian": {
        "albedo": {
          "SpatialChecker": {}
        }
      }
    }
  }
}
scene.objects.push(ground);



let num_spheres = 100;
for (let i = 0; i < num_spheres; i++) {
  // Size
  let radius = 0.1 + 3.0 * Math.random();

  // Color
  let color = [
    Math.random(),
    Math.random(),
    Math.random(),
  ];

  // Location
  let center = [
    // depth
    200.0 * (Math.random() - 0.5),
    // height: positive only
    radius + 5.0 * Math.random(),
    // width
    50.0 * (Math.random() - 0.5),
  ];

  // Default sphere
  let sphere = {
    "Sphere": {
      // TODO: fix the camera setup, these coordinates are in weird order :|
      center,
      radius
    }
  };

  // Material selector
  // let material = Math.round(1.0 * Math.random());
  let material = 0.0;

  switch (material) {
    case 0:
      // Dielectric
      sphere["Sphere"]["material"] = {
        "Dielectric": {
          // brighter color for the glass spheres
          color: [
            color[0] + 0.5,
            color[1] + 0.5,
            color[2] + 0.5,
          ],
          "refractive_index": 1.5
        },
      };
      break;

    // // Lambertian with checker
    // case 1:
    //   sphere["Sphere"]["material"] = {
    //     "Lambertian": {
    //       "albedo": {
    //         "SurfaceChecker": {
    //           "even": color,
    //           "odd": [color[0] / 2.0, color[1] / 2.0, color[2] / 2.0],
    //         }
    //       }
    //     }
    //   };
    //   break;

    // // Lambertian solid color
    // case 2:
    //   sphere["Sphere"]["material"] = {
    //     "Lambertian": {
    //       "albedo": {
    //         "SolidColor": {
    //           color
    //         }
    //       },
    //     },
    //   };
    //   break;

    // // Metal
    // case 3:
    //   sphere["Sphere"]["material"] = {
    //     "Metal": {
    //       "albedo": {
    //         "SolidColor": {
    //           color
    //         }
    //       },
    //       "fuzz": Math.random()
    //     },
    //   };
    //   break;



    // ConstantMedium
    // case 4:
    //   sphere = {
    //     // TODO: this is a weird override because ConstantMedium is an object, not a material by its own
    //     "ConstantMedium": {
    //       "boundary": {
    //         "Sphere": sphere["Sphere"]
    //       },
    //       "texture": {
    //         "SolidColor": {
    //           color
    //         }
    //       },
    //       "density": Math.random()
    //     }
    //   }
    //   break;
  }


  // Save the sphere to the objects list
  scene.objects.push(sphere);
}



let json = JSON.stringify(scene, null, 2);
fs.writeFileSync(path.join(__dirname, "random.json"), json);