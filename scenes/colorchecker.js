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
  materials: [],
};

// Big light for smooth lighting of the entire scene
let brightness = 2.5;
let lamp_material = {
  name: "big lamp",
  kind: "DiffuseLight",
  emit: {
    kind: "SolidColor",
    color: [brightness, brightness, brightness],
  },
};
scene.materials.push(lamp_material);
let light = {
  kind: "Quad",
  priority: true,
  q: [-100.0, 80.0, -100.0],
  u: [200.0, 0.0, 0.0],
  v: [0.0, 0.0, 200.0],
  material: "big lamp",
};
scene.objects.push(light);

const colors = [
  // Row 1: Natural colors
  [
    ["Dark skin", "#735244"],
    ["Light skin", "#c29682"],
    ["Blue sky", "#627a9d"],
    ["Foliage", "#576c43"],
    ["Blue flower", "#8580b1"],
    ["Bluish green", "#67bdaa"],
  ],
  // Row 2: Miscellaneous colors
  [
    ["Orange", "#d67e2c"],
    ["Purplish blue", "#505ba6"],
    ["Moderate red", "#c15a63"],
    ["Purple", "#5e3c6c"],
    ["Yellow green", "#9dbc40"],
    ["Orange yellow", "#e0a32e"],
  ],
  // Row 3: Primary and secondary colors
  [
    ["Blue", "#383d96"],
    ["Green", "#469449"],
    ["Red", "#af363c"],
    ["Yellow", "#e7c71f"],
    ["Magenta", "#bb5695"],
    ["Cyan", "#0885a1"],
  ],
  // Row 4: Grayscale colors
  [
    ["White", "#f3f3f2"],
    ["Neutral 8", "#c8c8c8"],
    ["Neutral 6.5", "#a0a0a0"],
    ["Neutral 5", "#7a7a7a"],
    ["Neutral 3.5", "#555555"],
    ["Black", "#343434"],
  ],
];

const width = 2.5;
const gap = 0.5;
const multiplier = width + gap;
const ystart = -0.5 * 4 * multiplier + 0.5 * gap;
const xstart = 0.5 * 4 * multiplier + 0.5 * gap;
colors.forEach((row, y) => {
  row.forEach(([name, hex], x) => {
    let material = {
      name,
      kind: "Lambertian",
      albedo: {
        kind: "SolidColor",
        color: {
          hex,
        },
      },
    };
    scene.materials.push(material);
    let quad = {
      kind: "Quad",
      // TODO: fix the camera setup, these coordinates are in weird order :|
      q: [ystart + y * multiplier, width, xstart + x * -multiplier],
      u: [width, 0.0, 0.0],
      v: [0.0, 0.0, width],
      material: name,
    };
    scene.objects.push(quad);
  });
});

let json = JSON.stringify(scene);
fs.writeFileSync(path.join(__dirname, "colorchecker.json"), json);
