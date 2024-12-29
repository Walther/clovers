use clovers::{spectrum::*, wavelength::SPECTRUM};
use palette::{white_point::E, Xyz};
use proptest::prelude::*;

proptest! {
  #[test]
  fn converts_all_wavelengths_black(lambda in SPECTRUM) {
    let xyz: Xyz<E> = Xyz::new(0.0, 0.0, 0.0);
      let _ = spectral_power(xyz, lambda);
  }
}

proptest! {
  #[test]
  fn converts_all_wavelengths_grey(lambda in SPECTRUM) {
    let xyz: Xyz<E> = Xyz::new(0.5, 0.5, 0.5);
      let _ = spectral_power(xyz, lambda);
  }
}

proptest! {
  #[test]
  fn converts_all_wavelengths_white(lambda in SPECTRUM) {
    let xyz: Xyz<E> = Xyz::new(1.0, 1.0, 1.0);
      let _ = spectral_power(xyz, lambda);
  }
}

// TODO: add more comprehensive tests for varying Xyz
