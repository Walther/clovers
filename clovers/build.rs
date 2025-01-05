use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use phf_codegen::OrderedMap;

// TODO: learn enough macros to clean this file up a bit

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

    let fluorescents = include_str!("src/illuminants/CIE_illum_FLs_1nm.csv");
    let [fl1, fl2, fl3, fl4, fl5, fl6, fl7, fl8, fl9, fl10, fl11, fl12, fl3_1, fl3_2, fl3_3, fl3_4, fl3_5, fl3_6, fl3_7, fl3_8, fl3_9, fl3_10, fl3_11, fl3_12, fl3_13, fl3_14, fl3_15] =
        parse_fluorescents(fluorescents);
    write_illuminant(fl1, "FL1");
    write_illuminant(fl2, "FL2");
    write_illuminant(fl3, "FL3");
    write_illuminant(fl4, "FL4");
    write_illuminant(fl5, "FL5");
    write_illuminant(fl6, "FL6");
    write_illuminant(fl7, "FL7");
    write_illuminant(fl8, "FL8");
    write_illuminant(fl9, "FL9");
    write_illuminant(fl10, "FL10");
    write_illuminant(fl11, "FL11");
    write_illuminant(fl12, "FL12");
    write_illuminant(fl3_1, "FL3_1");
    write_illuminant(fl3_2, "FL3_2");
    write_illuminant(fl3_3, "FL3_3");
    write_illuminant(fl3_4, "FL3_4");
    write_illuminant(fl3_5, "FL3_5");
    write_illuminant(fl3_6, "FL3_6");
    write_illuminant(fl3_7, "FL3_7");
    write_illuminant(fl3_8, "FL3_8");
    write_illuminant(fl3_9, "FL3_9");
    write_illuminant(fl3_10, "FL3_10");
    write_illuminant(fl3_11, "FL3_11");
    write_illuminant(fl3_12, "FL3_12");
    write_illuminant(fl3_13, "FL3_13");
    write_illuminant(fl3_14, "FL3_14");
    write_illuminant(fl3_15, "FL3_15");
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
/// Standard illuminant `{output}`
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

fn floaty(f: &str) -> String {
    if f.contains('.') {
        f.to_string()
    } else {
        format!("{f}.0")
    }
}

fn parse_pairs(input: &str) -> OrderedMap<usize> {
    let mut builder: OrderedMap<usize> = OrderedMap::new();

    for line in input.lines() {
        let (wavelength, power) = line.split_once(',').unwrap();
        let wavelength: usize = wavelength.parse().unwrap();
        builder.entry(wavelength, &floaty(power));
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

fn parse_fluorescents(input: &str) -> [OrderedMap<usize>; 27] {
    let mut fl1: OrderedMap<usize> = OrderedMap::new();
    let mut fl2: OrderedMap<usize> = OrderedMap::new();
    let mut fl3: OrderedMap<usize> = OrderedMap::new();
    let mut fl4: OrderedMap<usize> = OrderedMap::new();
    let mut fl5: OrderedMap<usize> = OrderedMap::new();
    let mut fl6: OrderedMap<usize> = OrderedMap::new();
    let mut fl7: OrderedMap<usize> = OrderedMap::new();
    let mut fl8: OrderedMap<usize> = OrderedMap::new();
    let mut fl9: OrderedMap<usize> = OrderedMap::new();
    let mut fl10: OrderedMap<usize> = OrderedMap::new();
    let mut fl11: OrderedMap<usize> = OrderedMap::new();
    let mut fl12: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_1: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_2: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_3: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_4: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_5: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_6: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_7: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_8: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_9: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_10: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_11: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_12: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_13: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_14: OrderedMap<usize> = OrderedMap::new();
    let mut fl3_15: OrderedMap<usize> = OrderedMap::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let wavelength: usize = split[0].parse().unwrap();

        fl1.entry(wavelength, &floaty(split[1]));
        fl2.entry(wavelength, &floaty(split[2]));
        fl3.entry(wavelength, &floaty(split[3]));
        fl4.entry(wavelength, &floaty(split[4]));
        fl5.entry(wavelength, &floaty(split[5]));
        fl6.entry(wavelength, &floaty(split[6]));
        fl7.entry(wavelength, &floaty(split[7]));
        fl8.entry(wavelength, &floaty(split[8]));
        fl9.entry(wavelength, &floaty(split[9]));
        fl10.entry(wavelength, &floaty(split[10]));
        fl11.entry(wavelength, &floaty(split[11]));
        fl12.entry(wavelength, &floaty(split[12]));
        fl3_1.entry(wavelength, &floaty(split[13]));
        fl3_2.entry(wavelength, &floaty(split[14]));
        fl3_3.entry(wavelength, &floaty(split[15]));
        fl3_4.entry(wavelength, &floaty(split[16]));
        fl3_5.entry(wavelength, &floaty(split[17]));
        fl3_6.entry(wavelength, &floaty(split[18]));
        fl3_7.entry(wavelength, &floaty(split[19]));
        fl3_8.entry(wavelength, &floaty(split[20]));
        fl3_9.entry(wavelength, &floaty(split[21]));
        fl3_10.entry(wavelength, &floaty(split[22]));
        fl3_11.entry(wavelength, &floaty(split[23]));
        fl3_12.entry(wavelength, &floaty(split[24]));
        fl3_13.entry(wavelength, &floaty(split[25]));
        fl3_14.entry(wavelength, &floaty(split[26]));
        fl3_15.entry(wavelength, &floaty(split[27]));
    }

    [
        fl1, fl2, fl3, fl4, fl5, fl6, fl7, fl8, fl9, fl10, fl11, fl12, fl3_1, fl3_2, fl3_3, fl3_4,
        fl3_5, fl3_6, fl3_7, fl3_8, fl3_9, fl3_10, fl3_11, fl3_12, fl3_13, fl3_14, fl3_15,
    ]
}
