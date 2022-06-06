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
    look_from: [13.0, 2.0, 3.0],
    look_at: [0.0, 0.0, 0.0],
    up: [0.0, 1.0, 0.0],
    vertical_fov: 25.0,
    aperture: 0.0,
    focus_distance: 10.0,
  },
  background_color: [0.7, 0.7, 0.9],
  objects: [],
  priority_objects: [],
};

let ground_texture = {
  kind: "Lambertian",
  albedo: {
    SolidColor: {
      color: [0.5, 0.5, 0.5],
    },
  },
};

let ground_sphere = {
  kind: "Sphere",
  center: [0.0, -1000.0, 0.0],
  radius: 1000.0,
  material: ground_texture,
};
scene.objects.push(ground_sphere);

let glass_sphere = {
  kind: "Sphere",
  center: [0.0, 1.0, 0.0],
  radius: 1.0,
  material: {
    kind: "Dielectric",
    refractive_index: 1.5,
  },
};
scene.objects.push(glass_sphere);
scene.priority_objects.push(glass_sphere);

let lambertian_sphere = {
  kind: "Sphere",
  center: [-4.0, 1.0, 0.0],
  radius: 1.0,
  material: {
    kind: "Lambertian",
    albedo: {
      SolidColor: {
        color: [0.4, 0.2, 0.1],
      },
    },
  },
};
scene.objects.push(lambertian_sphere);

let metal_sphere = {
  kind: "Sphere",
  center: [4.0, 1.0, 0.0],
  radius: 1.0,
  material: {
    kind: "Metal",
    fuzz: 0.0,
    albedo: {
      SolidColor: {
        color: [0.7, 0.6, 0.5],
      },
    },
  },
};
scene.objects.push(metal_sphere);

for (let a = -11; a < 11; a++) {
  for (let b = -11; b < 11; b++) {
    let choose_mat = random_float(0.0, 1.0);
    let center_0 = [
      a + 0.9 * random_float(0.0, 1.0),
      0.2,
      b + 0.9 * random_float(0.0, 1.0),
    ];

    if (choose_mat < 0.8) {
      // diffuse
      let color = [
        random_float(0.0, 1.0),
        random_float(0.0, 1.0),
        random_float(0.0, 1.0),
      ];
      let texture = {
        SolidColor: {
          color,
        },
      };
      let sphere_material = {
        kind: "Lambertian",
        albedo: texture,
      };
      let center_1 = [
        center_0[0],
        center_0[1] + random_float(0.0, 0.5),
        center_0[2],
      ];

      let moving_sphere = {
        kind: "MovingSphere",
        center_0,
        center_1,
        time_0,
        time_1,
        radius: 0.2,
        material: sphere_material,
      };
      scene.objects.push(moving_sphere);
    } else if (choose_mat < 0.95) {
      // metal
      let color = [
        random_float(0.0, 1.0),
        random_float(0.0, 1.0),
        random_float(0.0, 1.0),
      ];
      let texture = {
        SolidColor: {
          color,
        },
      };
      let fuzz = random_float(0.0, 0.5);
      let sphere_material = {
        kind: "Metal",
        albedo: texture,
        fuzz,
      };
      let sphere = {
        kind: "Sphere",
        center: center_0,
        radius: 0.2,
        material: sphere_material,
      };
      scene.objects.push(sphere);
    } else {
      // glass
      let sphere_material = {
        kind: "Dielectric",
        refractive_index: 1.5,
      };
      let sphere = {
        kind: "Sphere",
        center: center_0,
        radius: 0.2,
        material: sphere_material,
      };
      scene.objects.push(sphere);
    }
  }
}

let json = JSON.stringify(scene);
fs.writeFileSync(path.join(__dirname, "one_weekend.json"), json);
