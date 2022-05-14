#![allow(dead_code)]

use std::iter::zip;

use fraction::Fraction;

/// A translation of Rich's `Edge` protocol. Defines an edge (such as a
/// `Layout`).
struct Edge {
    size: Option<u32>,
    ratio: u32,
    minimum_size: u32,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            size: None,
            ratio: 1,
            minimum_size: 1,
        }
    }
}

/// Trait to mark a struct as having an `Edge`.
trait HasEdge {
    fn protocol(&self) -> Edge;
}

impl HasEdge for Edge {
    fn protocol(&self) -> Self {
        Self {
            size: self.size,
            ratio: self.ratio,
            minimum_size: self.minimum_size,
        }
    }
}

/// Divide total space to satisfy `size`, `ratio`, and `minimum_size`
/// constraints.
///
/// The returned vector of integers should add up to total in most cases, unless
/// it is impossible to satisfy all the constraints. For instance, if there are
/// two edges with a minimum size of 20 each and `total` is 30 then the returned
/// list will be greater than total. In practice, this would mean that a Layout
/// object would clip the rows that would overflow the screen height.
///
/// # Arguments
///
/// * `total` - The total number of characters.
/// * `edges` - Edges within total space.
///
/// # Returns
///
/// A vector of number of characters for each edge.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
fn ratio_resolve<E: HasEdge>(total: u32, edges: &[E]) -> Vec<u32> {
    let mut sizes: Vec<Option<u32>> = edges.iter().map(|edge| edge.protocol().size).collect();

    // While any edges haven't been calculated.
    while sizes.iter().any(Option::is_none) {
        // Get flexible edges and index to map these back on to sizes list.
        let flexible_edges = zip(sizes.clone(), edges)
            .enumerate()
            .filter(|&(_index, (size, _edge))| size.is_none())
            .map(|(index, (_size, edge))| (index, edge));

        // Remaining space in total.
        let remaining: i32 = total as i32
            - ((sizes
                .clone()
                .into_iter()
                .map(|size| size.unwrap_or(0))
                .sum::<u32>()) as i32);

        if remaining <= 0 {
            // No room for flexible edges.
            return zip(sizes, edges)
                .map(|(size, edge)| {
                    if let Some(size) = size {
                        size
                    } else if edge.protocol().minimum_size != 0 {
                        edge.protocol().minimum_size
                    } else {
                        1
                    }
                })
                .collect();
        }

        // Calculate number of characters in a ratio portion.
        let portion = Fraction::new(
            remaining as u32,
            flexible_edges
                .clone()
                .map(|(_, edge)| {
                    if edge.protocol().ratio == 0 {
                        1
                    } else {
                        edge.protocol().ratio
                    }
                })
                .sum::<u32>(),
        );

        let mut valid = true;
        // If any edges will be less than their minimum, replace size with the minimum.
        for (index, edge) in flexible_edges.clone() {
            if (portion * Fraction::new(edge.protocol().ratio, 1u8))
                <= Fraction::new(edge.protocol().minimum_size, 1u8)
            {
                sizes[index] = Some(edge.protocol().minimum_size);
                // New fixed size will invalidate calculations, so we need to repeat the
                // process.
                valid = false;
                break;
            }
        }
        if valid {
            // Distribute flexible space and compensate for rounding error.
            // Since edge sizes can only be integers we need to add the
            // remainder to the following line.
            let mut remainder = Fraction::new(0u8, 1u8);
            for (index, edge) in flexible_edges {
                let size = (portion * Fraction::new(u64::from(edge.protocol().ratio), 1u8)
                    + remainder)
                    / Fraction::new(1u8, 1u8);

                remainder = (portion * Fraction::new(u64::from(edge.protocol().ratio), 1u8)
                    + remainder)
                    % Fraction::new(1u8, 1u8);

                sizes[index] = Some((*size.numer().unwrap() / *size.denom().unwrap()) as u32);
            }
        }
    }

    // Sizes now contains u32 only.
    sizes.into_iter().flatten().collect()
}

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
    #[case(100, &Vec::<Edge>::new(), vec![])]
    #[case(100, &vec![Edge { size: Some(100), ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }], vec![100, 1])]
    #[case(100, &vec![Edge { ratio: 1, ..Edge::default() }], vec![100])]
    #[case(100, &vec![Edge { ratio: 1, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }], vec![50, 50])]
    #[case(100, &vec![Edge { size: Some(20), ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }], vec![20, 40, 40])]
    #[case(100, &vec![Edge { size: Some(40), ..Edge::default() }, Edge { ratio: 2, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }], vec![40, 40, 20])]
    #[case(100, &vec![Edge { size: Some(40), ..Edge::default() }, Edge { ratio: 2, ..Edge::default() }, Edge { ratio: 1, minimum_size: 25, ..Edge::default() }], vec![40, 35, 25])]
    #[case(100, &vec![Edge { ratio: 1, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }], vec![33, 33, 34])]
    #[case(50, &vec![Edge { size: Some(30), ..Edge::default() }, Edge { ratio: 1, minimum_size: 10, ..Edge::default() }, Edge { size: Some(30), ..Edge::default() }], vec![30, 10, 30])]
    #[case(110, &vec![Edge { ratio: 1, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }, Edge { ratio: 1, ..Edge::default() }], vec![36, 37, 37])]
    #[case(50, &vec![Edge { size: Some(30), ..Edge::default() }, Edge { ratio: 1, minimum_size: 0, ..Edge::default() }, Edge { size: Some(30), ..Edge::default() }], vec![30, 1, 30])]
    fn test_ratio_resolve<E: HasEdge>(
        #[case] total: u32,
        #[case] edges: &[E],
        #[case] result: Vec<u32>,
    ) {
        assert_eq!(ratio_resolve(total, edges), result);
    }

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
