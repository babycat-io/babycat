#![cfg(feature = "rayon")]

use ndarray::prelude::*;

use ndarray::Zip;

const M: usize = 1024 * 10;
const N: usize = 100;

#[test]
fn test_zip_1() {
    let mut a = Array2::<f64>::zeros((M, N));

    Zip::from(&mut a).par_apply(|x| *x = x.exp());
}

#[test]
fn test_zip_index_1() {
    let mut a = Array2::default((10, 10));

    Zip::indexed(&mut a).par_apply(|i, x| {
        *x = i;
    });

    for (i, elt) in a.indexed_iter() {
        assert_eq!(*elt, i);
    }
}

#[test]
fn test_zip_index_2() {
    let mut a = Array2::default((M, N));

    Zip::indexed(&mut a).par_apply(|i, x| {
        *x = i;
    });

    for (i, elt) in a.indexed_iter() {
        assert_eq!(*elt, i);
    }
}

#[test]
fn test_zip_index_3() {
    let mut a = Array::default((1, 2, 1, 2, 3));

    Zip::indexed(&mut a).par_apply(|i, x| {
        *x = i;
    });

    for (i, elt) in a.indexed_iter() {
        assert_eq!(*elt, i);
    }
}

#[test]
fn test_zip_index_4() {
    let mut a = Array2::zeros((M, N));
    let mut b = Array2::zeros((M, N));

    Zip::indexed(&mut a).and(&mut b).par_apply(|(i, j), x, y| {
        *x = i;
        *y = j;
    });

    for ((i, _), elt) in a.indexed_iter() {
        assert_eq!(*elt, i);
    }
    for ((_, j), elt) in b.indexed_iter() {
        assert_eq!(*elt, j);
    }
}

#[test]
#[cfg(feature = "approx")]
fn test_zip_collect() {
    use approx::assert_abs_diff_eq;

    // test Zip::apply_collect and that it preserves c/f layout.

    let b = Array::from_shape_fn((M, N), |(i, j)| 1. / (i + 2 * j + 1) as f32);
    let c = Array::from_shape_fn((M, N), |(i, j)| f32::ln((1 + i + j) as f32));

    {
        let a = Zip::from(&b).and(&c).par_apply_collect(|x, y| x + y);

        assert_abs_diff_eq!(a, &b + &c, epsilon = 1e-6);
        assert_eq!(a.strides(), b.strides());
    }

    {
        let b = b.t();
        let c = c.t();

        let a = Zip::from(&b).and(&c).par_apply_collect(|x, y| x + y);

        assert_abs_diff_eq!(a, &b + &c, epsilon = 1e-6);
        assert_eq!(a.strides(), b.strides());
    }

}

#[test]
#[cfg(feature = "approx")]
fn test_zip_small_collect() {
    use approx::assert_abs_diff_eq;

    for m in 0..32 {
        for n in 0..16 {
            let dim = (m, n);
            let b = Array::from_shape_fn(dim, |(i, j)| 1. / (i + 2 * j + 1) as f32);
            let c = Array::from_shape_fn(dim, |(i, j)| f32::ln((1 + i + j) as f32));

            {
                let a = Zip::from(&b).and(&c).par_apply_collect(|x, y| x + y);

                assert_abs_diff_eq!(a, &b + &c, epsilon = 1e-6);
                assert_eq!(a.strides(), b.strides());
            }
        }
    }
}

#[test]
#[cfg(feature = "approx")]
fn test_zip_assign_into() {
    use approx::assert_abs_diff_eq;

    let mut a = Array::<f32, _>::zeros((M, N));
    let b = Array::from_shape_fn((M, N), |(i, j)| 1. / (i + 2 * j + 1) as f32);
    let c = Array::from_shape_fn((M, N), |(i, j)| f32::ln((1 + i + j) as f32));

    Zip::from(&b).and(&c).par_apply_assign_into(&mut a, |x, y| x + y);

    assert_abs_diff_eq!(a, &b + &c, epsilon = 1e-6);
}
