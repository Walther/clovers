let fs = require("fs");
let path = require("path");
let random_float = (min, max) => {
  return Math.random() * (max - min) + min;
};

let time_0 = 0.0;
let time_1 = 1.0;

let scene = {
  time_0,
  time_1,
  camera: {
    look_from: [478.0, 278.0, -600.0],
    look_at: [278.0, 278.0, 0.0],
    up: [0.0, 1.0, 0.0],
    vertical_fov: 40.0,
    aperture: 0.0,
    focus_distance: 10.0,
  },
  background_color: [0.0, 0.0, 0.0],
  objects: [],
  priority_objects: [],
};

let ground = {
  Lambertian: {
    albedo: {
      SolidColor: {
        color: [0.48, 0.83, 0.53],
      },
    },
  },
};

let boxes = [];
let boxes_per_side = 20;
let box_epsilon = 0.00001;
for (let i = 0; i < boxes_per_side; i++) {
  for (let j = 0; j < boxes_per_side; j++) {
    let w = 100.0;
    let x0 = -1000.0 + i * w;
    let z0 = -1000.0 + j * w;
    let y0 = 0.0;
    let x1 = x0 + w - box_epsilon;
    let y1 = random_float(10.0, 101.0);
    let z1 = z0 + w - box_epsilon;

    let box = {
      Boxy: {
        corner_0: [x0, y0, z0],
        corner_1: [x1, y1, z1],
        material: ground,
      },
    };

    boxes.push(box);
  }
}
scene.objects.push(...boxes);

let light = {
  Quad: {
    q: [123.0, 554.0, 147.0],
    u: [300.0, 0.0, 0.0],
    v: [0.0, 0.0, 265.0],
    material: {
      DiffuseLight: {
        emit: {
          SolidColor: {
            color: [7.0, 7.0, 7.0],
          },
        },
      },
    },
  },
};
scene.objects.push(light);
scene.priority_objects.push(light);

let moving_sphere = {
  MovingSphere: {
    center_0: [400.0, 400.0, 200.0],
    center_1: [420.0, 400.0, 200.0],
    time_0,
    time_1,
    radius: 50.0,
    material: {
      Lambertian: {
        albedo: {
          SolidColor: {
            color: [0.7, 0.3, 0.1],
          },
        },
      },
    },
  },
};
scene.objects.push(moving_sphere);

let glass_sphere = {
  Sphere: {
    center: [260.0, 150.0, 45.0],
    radius: 50.0,
    material: {
      Dielectric: {
        refractive_index: 1.5,
      },
    },
  },
};
scene.objects.push(glass_sphere);

let half_matte_metal_sphere = {
  Sphere: {
    center: [0.0, 150.0, 145.0],
    radius: 50.0,
    material: {
      Metal: {
        fuzz: 1.0,
        albedo: {
          SolidColor: {
            color: [0.8, 0.8, 0.9],
          },
        },
      },
    },
  },
};
scene.objects.push(half_matte_metal_sphere);

// subsurface scattering object, first part
let blue_sphere_glass = {
  Sphere: {
    center: [360.0, 150.0, 145.0],
    radius: 70.0,
    material: {
      Dielectric: {
        refractive_index: 1.5,
      },
    },
  },
};
scene.objects.push(blue_sphere_glass);

// subsurface scattering object, second part
let blue_sphere_smoke = {
  ConstantMedium: {
    density: 0.2,
    boundary: {
      ...blue_sphere_glass,
    },
    texture: {
      SolidColor: {
        color: [0.2, 0.4, 0.9],
      },
    },
  },
};
scene.objects.push(blue_sphere_smoke);

// Big mist for the whole scene
let mist = {
  ConstantMedium: {
    density: 0.0001,
    boundary: {
      Sphere: {
        center: [0.0, 0.0, 0.0],
        radius: 5000.0,
      },
    },
    texture: {
      SolidColor: {
        color: [1.0, 1.0, 1.0],
      },
    },
  },
};
scene.objects.push(mist);

let marble = {
  Sphere: {
    center: [220.0, 280.0, 300.0],
    radius: 80.0,
    material: {
      Lambertian: {
        // albedo: {
          // Originally NoiseTexture. Removed support for it.
          // NoiseTexture: {
          //   scale: 0.1,
          // },
        // },
      },
    },
  },
};
scene.objects.push(marble);

// Sphere-rasterized pseudo box
let spherebox = {
  ObjectList: [],
};
let num_spheres = 1000;
let white = {
  Lambertian: {
    albedo: {
      SolidColor: {
        color: [0.73, 0.73, 0.73],
      },
    },
  },
};
for (let i = 0; i < num_spheres; i++) {
  let sphere = {
    Sphere: {
      center: [
        random_float(0.0, 165.0),
        random_float(0.0, 165.0),
        random_float(0.0, 165.0),
      ],
      radius: 10.0,
      material: white,
    },
  };
  spherebox.ObjectList.push(sphere);
}
let rotated_spherebox = {
  RotateY: {
    angle: 15.0,
    object: spherebox,
  },
};
let translated_spherebox = {
  Translate: {
    offset: [-100.0, 270.0, 395.0],
    object: rotated_spherebox,
  },
};
scene.objects.push(translated_spherebox);

let json = JSON.stringify(scene, null, 2);
fs.writeFileSync(path.join(__dirname, "the_next_week.json"), json);
