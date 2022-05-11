#![allow(dead_code)]

use std::iter::zip;

/// Divide a integer total in to parts based on ratios.
///
/// # Arguments
///
/// * `total` - The integer total to divide.
/// * `ratios` - A vector of integer ratios.
/// * `maximums` - A vector of maximum values for each slot.
/// * `values` - A vector of values.
///
/// # Returns
///
/// A vector of integers guaranteed to sum to total.
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn ratio_reduce(total: i32, ratios: &[i32], maximums: &[i32], values: Vec<i32>) -> Vec<i32> {
    let ratios = zip(ratios, maximums).map(|(ratio, max)| if max == &0 { &0 } else { ratio });

    let mut total_ratio: i32 = ratios.clone().sum();
    if total_ratio == 0 {
        return values;
    }

    let mut total_remaining = total;
    let mut result: Vec<i32> = vec![];
    for ((ratio, maximum), value) in ratios.zip(maximums).zip(values) {
        if ratio != &0 && total_ratio > 0 {
            let distributed = (*maximum as f32)
                .min((ratio * total_remaining) as f32 / total_ratio as f32)
                .round() as i32;
            result.push(value - distributed);
            total_remaining -= distributed;
            total_ratio -= ratio;
        } else {
            result.push(value);
        }
    }

    result
}

/// Distribute an integer total in to parts based on ratios.
///
/// # Arguments
///
/// * `total` - The total to divide.
/// * `ratios` - A vector of integer ratios.
/// * `minimums` - Optional vector of minimum values for each slot.
///
/// # Returns
///
/// A vector of integers gauranteed to sum to total.
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
fn ratio_distribute(total: i32, ratios: Vec<i32>, minimums: Option<Vec<i32>>) -> Vec<i32> {
    let ratios = if let Some(minimums) = minimums.clone() {
        zip(ratios, minimums)
            .map(|(ratio, min)| if min == 0 { 0 } else { ratio })
            .collect()
    } else {
        ratios
    };

    let mut total_ratio: i32 = ratios.iter().sum();
    let mut total_remaining = total;
    let mut distributed_total: Vec<i32> = vec![];

    let minimums = minimums.unwrap_or_else(|| vec![0; ratios.len()]);

    for (ratio, minimum) in zip(ratios, minimums) {
        let distributed = if total_ratio > 0 {
            (minimum as f32).max(((ratio * total_remaining) as f32 / total_ratio as f32).ceil())
                as i32
        } else {
            total_remaining
        };

        distributed_total.push(distributed);
        total_ratio -= ratio;
        total_remaining -= distributed;
    }

    distributed_total
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(20, &vec![2, 4], &vec![20, 20], vec![5, 5], vec![-2, -8])]
    #[case(20, &vec![2, 4], &vec![1, 1], vec![5, 5], vec![4, 4])]
    #[case(20, &vec![2, 4], &vec![1, 1], vec![2, 2], vec![1, 1])]
    #[case(3, &vec![2, 4], &vec![3, 3], vec![2, 2], vec![1, 0])]
    #[case(3, &vec![2, 4], &vec![3, 3], vec![0, 0], vec![-1, -2])]
    #[case(3, &vec![0, 0], &vec![3, 3], vec![4, 4], vec![4, 4])]
    #[case(3, &vec![5, -6], &vec![3, 3], vec![5, 5], vec![5, 5])]
    fn test_ratio_reduce(
        #[case] total: i32,
        #[case] ratios: &[i32],
        #[case] maximums: &[i32],
        #[case] values: Vec<i32>,
        #[case] result: Vec<i32>,
    ) {
        assert_eq!(ratio_reduce(total, ratios, maximums, values), result);
    }

    #[rstest]
    #[case(10, vec![1], None, vec![10])]
    #[case(10, vec![1, 1], None, vec![5, 5])]
    #[case(12, vec![1, 3], None, vec![3, 9])]
    #[case(0, vec![1, 3], None, vec![0, 0])]
    #[case(0, vec![1, 3], Some(vec![1, 1]), vec![1, 1])]
    #[case(10, vec![1, 0], None, vec![10, 0])]
    fn test_ratio_distribute(
        #[case] total: i32,
        #[case] ratios: Vec<i32>,
        #[case] minimums: Option<Vec<i32>>,
        #[case] result: Vec<i32>,
    ) {
        assert_eq!(ratio_distribute(total, ratios, minimums), result);
    }
}
