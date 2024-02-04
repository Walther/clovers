use clovers::wavelength::*;
use proptest::prelude::*;

proptest! {
  #[test]
  fn converts_all_wavelengths(lambda in SPECTRUM) {
      let _ = wavelength_into_xyz(lambda);
  }
}

proptest! {
  #[test]
  fn rotates_all_wavelengths(lambda in SPECTRUM) {
      let mut waves = rotate_wavelength(lambda);
      prop_assert_eq!(waves[0], lambda);
      waves.sort();
      let [a, b, c, d] = waves;
      let diff = a.abs_diff(b);
      prop_assert_eq!(diff, b.abs_diff(c));
      prop_assert_eq!(diff, c.abs_diff(d));
  }
}
