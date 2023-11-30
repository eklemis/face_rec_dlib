pub fn compute_average(features: &[Vec<f64>]) -> Vec<f64> {
    let len = features.len();
    let mut sum = vec![0.0; features[0].len()];

    for feature in features {
        for (i, val) in feature.iter().enumerate() {
            sum[i] += val;
        }
    }

    sum.iter().map(|&x| x / len as f64).collect()
}

pub fn compute_median(features: &[Vec<f64>]) -> Vec<f64> {
    let len = features.len();
    let mut median = vec![vec![]; features[0].len()];

    for feature in features {
        for (i, val) in feature.iter().enumerate() {
            median[i].push(*val);
        }
    }

    median
        .iter_mut()
        .map(|vals| {
            vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if len % 2 == 0 {
                (vals[len / 2 - 1] + vals[len / 2]) / 2.0
            } else {
                vals[len / 2]
            }
        })
        .collect()
}
