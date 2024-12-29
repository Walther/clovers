use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let d50 = include_str!("src/illuminants/CIE_std_illum_D50.csv");
    write_illuminant_simple(d50, "D50");
    let d65 = include_str!("src/illuminants/CIE_std_illum_D65.csv");
    write_illuminant_simple(d65, "D65");
}

fn write_illuminant_simple(input: &str, output: &str) {
    let mut builder: phf_codegen::OrderedMap<usize> = phf_codegen::OrderedMap::new();
    parse_pairs(input, &mut builder);

    let outpath = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("{output}.rs"));
    let mut outfile = BufWriter::new(File::create(&outpath).unwrap());
    let data = builder.build();
    writeln!(&mut outfile, "#[allow(clippy::unreadable_literal)]").unwrap();
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
    fn color(&self, _hit_record: &HitRecord) -> Xyz<E> {{
        // FIXME:
        Xyz::new(1.0, 1.0, 1.0)
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

fn parse_pairs(input: &str, builder: &mut phf_codegen::OrderedMap<usize>) {
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
}
