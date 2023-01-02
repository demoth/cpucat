pub fn resample(source: Vec<f32>, target_len: usize) -> Vec<f32> {
    assert!(target_len > 0);
    assert!(!source.is_empty());

    if source.len() == target_len {
        return source;
    }

    // how fast the cursor should move over the source vector
    // relative to the target vector
    // e.g. if target is 3 elements wide and the source is 4,
    // then the target vec is expected to be composed of
    // [the 0th element, the average of 1st and 2nd, the 3rd]
    // so the step is then 1.5.
    let step = (source.len() - 1) as f64 / (target_len - 1).max(1) as f64;
    let mut result = Vec::with_capacity(target_len);

    for i in 0..target_len {
        result.push(lerp(&source, i as f32 * step as f32));
    }

    result
}

/// Linearly interpolates points to the left and to the right of the position
/// 
/// e.g.
/// ```
/// # use cpucat::resample_tools::lerp;
/// let s = [1.0, 10.0, 20.0, 30.0];
/// // pos   0.0  1.0   2.0   3.0
/// 
/// // 0.9 ------^
/// assert_eq!(lerp(&s, 0.9), 9.1);
/// 
/// // 2.2 ---------------^
/// assert_eq!(lerp(&s, 2.2), 22.0);
/// ```
pub fn lerp(source: &[f32], positon: f32) -> f32 {
    let left_point = positon.floor() as usize;
    let left_value = source[left_point.clamp(0, source.len() - 1)];
    let left_fract = 1.0 - positon.fract();

    let right_point = positon.ceil() as usize;
    let right_value = source[right_point.clamp(0, source.len() - 1)];
    let right_fract = positon.fract();

    left_value * left_fract + right_value * right_fract
}

#[cfg(test)]
mod tests {

    mod interpolate {
        use crate::resample;
        #[test]
        fn simple_fract() {
            assert_eq!(resample(vec![0.0, 1.0], 5), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
        }

        #[test]
        fn complicated_frac() {
            assert_eq!(
                resample(vec![0.0, 1.0, 3.0, 2.0], 14),
                vec![
                    0.0, 0.23076923, 0.46153846, 0.6923077, 0.9230769, 1.3076923, 1.7692308,
                    2.2307692, 2.692308, 2.9230769, 2.6923077, 2.4615386, 2.2307692, 2.0
                ]
            );
        }

        #[test]
        fn large() {
            let large = resample(vec![0.0, 1.0], 5_000_000);
            assert_eq!(large.first(), Some(&0.0));
            assert_eq!(large.get(2_500_000), Some(&0.5000001));
            assert_eq!(large.last(), Some(&1.0));
        }

        #[test]
        fn one_to_many() {
            assert_eq!(
                resample(vec![42.0], 10),
                [42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0]
            )
        }
    }

    mod eq {
        use crate::resample;
        #[test]
        fn no_resampling() {
            assert_eq!(resample(vec![1.0, 2.0], 2), vec![1.0, 2.0]);
            assert_eq!(resample(vec![2.0], 1), vec![2.0]);
        }
    }

    mod downsample {
        use crate::resample;

        #[test]
        fn one_is_first() {
            assert_eq!(resample(vec![1.0, 3.0, 2.0], 1), vec![1.0]);
        }

        #[test]
        fn one_smaller() {
            assert_eq!(resample(vec![1.0, 2.0, 3.0, 4.0], 3), vec![1.0, 2.5, 4.0]);
        }

        #[test]
        fn large_overlap() {
            assert_eq!(resample(vec![1.0, 2.0, 3.0, 4.0, 5.0], 2), vec![1.0, 5.0]);
        }
    }
}
