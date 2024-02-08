//! Hand-converted from `spectrum_grid.h` in the supplemental material from [Physically Meaningful Rendering using Tristimulus Colours](https://doi.org/10.1111/cgf.12676)

#![allow(clippy::pedantic)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use super::spectra_xyz_5nm_380_780_097::*;

/*
 * Evaluate the spectrum for xyz at the given wavelength.
 */
pub(super) fn spectrum_xyz_to_p(lambda: f64, xyz: [f64; 3]) -> f64 {
    assert!(lambda >= spectrum_sample_min);
    assert!(lambda <= spectrum_sample_max);
    let mut xyY: [f64; 3] = [0.0, 0.0, 0.0];
    let mut uv: [f64; 2] = [0.0, 0.0];

    let norm: f64 = 1.0 / (xyz[0] + xyz[1] + xyz[2]);
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(norm < f64::MAX) {
        return 0.0;
    }
    // convert to xy chromaticities
    xyY[0] = xyz[0] * norm;
    xyY[1] = xyz[1] * norm;
    xyY[2] = xyz[1];

    // rotate to align with grid
    spectrum_xy_to_uv([xyY[0], xyY[1]], &mut uv);

    if uv[0] < 0.0
        || uv[0] >= spectrum_grid_width_f
        || uv[1] < 0.0
        || uv[1] >= spectrum_grid_height_f
    {
        return 0.0;
    }

    let uvi: [usize; 2] = [uv[0] as usize, uv[1] as usize];
    assert!(uvi[0] < spectrum_grid_width);
    assert!(uvi[1] < spectrum_grid_height);

    let cell_idx: usize = uvi[0] + spectrum_grid_width * uvi[1];
    assert!(cell_idx < spectrum_grid_width * spectrum_grid_height);
    // assert!(cell_idx >= 0);

    let spectrum_grid_cell_t {
        inside,
        num_points,
        idx,
    } = spectrum_grid[cell_idx];
    let num = num_points;

    // TODO: can this alloc be removed?

    // get linearly interpolated spectral power for the corner vertices:
    let mut p = std::iter::repeat(0.0).take(num).collect::<Vec<_>>();
    // this clamping is only necessary if lambda is not sure to be >= spectrum_sample_min and <= spectrum_sample_max:
    let sb: f64 = //fminf(spectrum_num_samples-1e-4, fmaxf(0.0,
        (lambda - spectrum_sample_min)/(spectrum_sample_max-spectrum_sample_min) * (spectrum_num_samples_f-1.0); //));
    assert!(sb >= 0.0);
    assert!(sb <= spectrum_num_samples_f);

    let sb0: usize = sb as usize;
    let sb1: usize = if sb0 + 1 < spectrum_num_samples {
        sb0 + 1
    } else {
        spectrum_num_samples - 1
    };
    let sbf: f64 = sb - sb0 as f64;
    for i in 0..num {
        let index = idx[i];
        assert!(index >= 0);
        let index = index as usize;
        assert!(sb0 < spectrum_num_samples);
        assert!(sb1 < spectrum_num_samples);
        let spectrum = spectrum_data_points[index].spectrum;
        p[i] = spectrum[sb0] * (1.0 - sbf) + spectrum[sb1] * sbf;
    }

    let mut interpolated_p: f64 = 0.0;

    if inside == 1 {
        // fast path for normal inner quads:
        uv[0] -= uvi[0] as f64;
        uv[1] -= uvi[1] as f64;

        assert!(uv[0] >= 0.0 && uv[0] <= 1.0);
        assert!(uv[1] >= 0.0 && uv[1] <= 1.0);

        // the layout of the vertices in the quad is:
        //  2  3
        //  0  1
        interpolated_p = p[0] * (1.0 - uv[0]) * (1.0 - uv[1])
            + p[2] * (1.0 - uv[0]) * uv[1]
            + p[3] * uv[0] * uv[1]
            + p[1] * uv[0] * (1.0 - uv[1]);
    } else {
        // need to go through triangulation :(
        // we get the indices in such an order that they form a triangle fan around idx[0].
        // compute barycentric coordinates of our xy* point for all triangles in the fan:
        let ex: f64 = uv[0] - spectrum_data_points[idx[0] as usize].uv[0];
        let ey: f64 = uv[1] - spectrum_data_points[idx[0] as usize].uv[1];
        let mut e0x: f64 = spectrum_data_points[idx[1] as usize].uv[0]
            - spectrum_data_points[idx[0] as usize].uv[0];
        let mut e0y: f64 = spectrum_data_points[idx[1] as usize].uv[1]
            - spectrum_data_points[idx[0] as usize].uv[1];
        let mut uu: f64 = e0x * ey - ex * e0y;
        for i in 0..(num - 1) {
            let e1x: f64;
            let e1y: f64;
            if i == num - 2 {
                // close the circle
                e1x = spectrum_data_points[idx[1] as usize].uv[0]
                    - spectrum_data_points[idx[0] as usize].uv[0];
                e1y = spectrum_data_points[idx[1] as usize].uv[1]
                    - spectrum_data_points[idx[0] as usize].uv[1];
            } else {
                e1x = spectrum_data_points[idx[i + 2] as usize].uv[0]
                    - spectrum_data_points[idx[0] as usize].uv[0];
                e1y = spectrum_data_points[idx[i + 2] as usize].uv[1]
                    - spectrum_data_points[idx[0] as usize].uv[1];
            }
            let vv: f64 = ex * e1y - e1x * ey;

            // TODO: with some sign magic, this division could be deferred to the last iteration!
            let area: f64 = e0x * e1y - e1x * e0y;
            // normalise
            let u: f64 = uu / area;
            let v: f64 = vv / area;
            let w: f64 = 1.0 - u - v;
            // outside spectral locus (quantized version at least) or outside grid
            if u < 0.0 || v < 0.0 || w < 0.0 {
                uu = -vv;
                e0x = e1x;
                e0y = e1y;
                continue;
            }

            // This seems to be the triangle we've been looking for.
            interpolated_p = p[0] * w + p[i + 1] * v + p[if i == num - 2 { 1 } else { i + 2 }] * u;
            break;
        }
    }

    // now we have a spectrum which corresponds to the xy chromaticities of the input. need to scale according to the
    // input brightness X+Y+Z now:
    interpolated_p / norm
}
