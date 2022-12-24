pub fn resample(source: Vec<f32>, expected_len: usize) -> Vec<f32> {
    assert!(expected_len > 0);
    assert!(!source.is_empty());

    match source.len().cmp(&expected_len) {
        // No work required
        std::cmp::Ordering::Equal => source,

        // Interpolate
        std::cmp::Ordering::Less => {
            let mut result = Vec::with_capacity(expected_len);
            let step = (source.len() - 1) as f64 / (expected_len - 1) as f64;
            let mut source_index: f64 = 0.0;
            for _i in 0..expected_len {
                let left_index = source_index.floor() as usize;
                let lerp_percent = source_index.fract() as f32;
                let value = source[left_index] * (1.0 - lerp_percent)
                    + source.get(left_index + 1).copied().unwrap_or(0.0) * lerp_percent;
                source_index += step;
                result.push(value);
            }
            result
        }

        // Downsample
        std::cmp::Ordering::Greater => {
            if expected_len == 1 {
                // single element being the average
                vec![source.iter().copied().sum::<f32>() / source.len() as f32]
            } else {
                let mut result = Vec::with_capacity(expected_len);
                let step = source.len() as f64 / expected_len as f64;
                assert!(step > 1.0);
                let mut source_index: f64 = 0.0;
                for _i in 0..expected_len {
                    let left_index = source_index.floor() as usize;
                    let left_fract = 1.0 - source_index.fract() as f32;
                    source_index += step;
                    let mut right_index = source_index.floor() as usize;
                    let mut right_fract = source_index.fract() as f32;
                    if right_index == source.len() {
                        right_index -= 1;
                        right_fract = 1.0;
                    }

                    let mut values = source[left_index..=right_index].to_vec();
                    *values.first_mut().unwrap() *= left_fract;
                    *values.last_mut().unwrap() *= right_fract;

                    let value: f32 = values.iter().copied().sum::<f32>()
                        / ((values.len() - 2) as f32 + left_fract + right_fract);
                    result.push(value);
                }

                result
            }
        }
    }
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
                    0.0, 0.23076923, 0.46153846, 0.6923077, 0.9230769, 1.3076923, 1.7692307,
                    2.2307694, 2.692308, 2.923077, 2.6923077, 2.4615383, 2.2307692, 2.0
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
                vec![42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0, 42.0]
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
        fn average() {
            assert_eq!(resample(vec![1.0, 3.0, 2.0], 1), vec![2.0]);
        }

        #[test]
        fn one_smaller() {
            assert_eq!(
                resample(vec![1.0, 3.0, 2.0, 10.0], 3),
                vec![1.5, 2.5, 8.000001]
            );
        }
        #[test]
        fn two_to_one() {
            assert_eq!(resample(vec![1.0, 10.0], 1), vec![5.5]);
        }
    }
}
