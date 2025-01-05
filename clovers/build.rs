use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use phf_codegen::OrderedMap;

fn main() {
    let d50 = include_str!("src/illuminants/CIE_std_illum_D50.csv");
    let d50 = parse_pairs(d50);
    write_illuminant(d50, "D50");
    let d65 = include_str!("src/illuminants/CIE_std_illum_D65.csv");
    let d65 = parse_pairs(d65);
    write_illuminant(d65, "D65");

    let leds = include_str!("src/illuminants/CIE_illum_LEDs_1nm.csv");
    let [led_b1, led_b2, led_b3, led_b4, led_b5, led_bh1, led_rgb1, led_v1, led_v2] =
        parse_leds(leds);

    write_illuminant(led_b1, "LED_B1");
    write_illuminant(led_b2, "LED_B2");
    write_illuminant(led_b3, "LED_B3");
    write_illuminant(led_b4, "LED_B4");
    write_illuminant(led_b5, "LED_B5");
    write_illuminant(led_bh1, "LED_BH1");
    write_illuminant(led_rgb1, "LED_RGB1");
    write_illuminant(led_v1, "LED_V1");
    write_illuminant(led_v2, "LED_V2");
}

fn write_illuminant(builder: OrderedMap<usize>, output: &str) {
    let outpath = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("{output}.rs"));
    let mut outfile = BufWriter::new(File::create(&outpath).unwrap());
    let data = builder.build();
    write!(
        &mut outfile,
        "static {output}: phf::OrderedMap<Wavelength, Float> = {data}"
    )
    .unwrap();
    writeln!(&mut outfile, ";").unwrap();
    write!(
        &mut outfile,
        r#"#
[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-derive", derive(serde::Serialize, serde::Deserialize))]
/// Standard illuminant {output}
pub struct {output} {{
    #[cfg_attr(feature = "serde-derive", serde(default = "default_intensity"))]
    intensity: Float,
}}

impl TextureTrait for {output} {{
    #[must_use]
    fn color(&self, _ray: &Ray, wavelength: Wavelength, _hit_record: &HitRecord) -> Float {{
        let color = match {output}.get(&wavelength) {{
            Some(&p) => p * self.intensity,
            None => 0.0,
        }};
        color.clamp(0.0, 1.0)
    }}

    #[must_use]
    fn emit(
        &self,
        _ray: &Ray,
        wavelength: Wavelength,
        _hit_record: &HitRecord,
    ) -> Float {{
        match {output}.get(&wavelength) {{
            Some(&p) => p * self.intensity,
            None => 0.0,
        }}
    }}
}}
"#
    )
    .unwrap()
}

fn parse_pairs(input: &str) -> OrderedMap<usize> {
    let mut builder: OrderedMap<usize> = OrderedMap::new();

    for line in input.lines() {
        let (wavelength, power) = line.split_once(',').unwrap();
        let wavelength: usize = wavelength.parse().unwrap();
        // Ensure the numbers are floaty
        if power.contains('.') {
            builder.entry(wavelength, power);
        } else {
            builder.entry(wavelength, &format!("{power}.0"));
        }
    }
    builder
}

fn parse_leds(input: &str) -> [OrderedMap<usize>; 9] {
    let mut led_b1: OrderedMap<usize> = OrderedMap::new();
    let mut led_b2: OrderedMap<usize> = OrderedMap::new();
    let mut led_b3: OrderedMap<usize> = OrderedMap::new();
    let mut led_b4: OrderedMap<usize> = OrderedMap::new();
    let mut led_b5: OrderedMap<usize> = OrderedMap::new();
    let mut led_bh1: OrderedMap<usize> = OrderedMap::new();
    let mut led_rgb1: OrderedMap<usize> = OrderedMap::new();
    let mut led_v1: OrderedMap<usize> = OrderedMap::new();
    let mut led_v2: OrderedMap<usize> = OrderedMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let wavelength: usize = split[0].parse().unwrap();
        led_b1.entry(wavelength, split[1]);
        led_b2.entry(wavelength, split[2]);
        led_b3.entry(wavelength, split[3]);
        led_b4.entry(wavelength, split[4]);
        led_b5.entry(wavelength, split[5]);
        led_bh1.entry(wavelength, split[6]);
        led_rgb1.entry(wavelength, split[7]);
        led_v1.entry(wavelength, split[8]);
        led_v2.entry(wavelength, split[9]);
    }

    [
        led_b1, led_b2, led_b3, led_b4, led_b5, led_bh1, led_rgb1, led_v1, led_v2,
    ]
}
