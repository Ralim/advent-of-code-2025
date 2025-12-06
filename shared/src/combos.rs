use itertools::{Itertools, repeat_n};

pub fn create_n_choice_sets_cloned<T>(set: &[T], num_elements: usize) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    //Create all combinations of operations for the number of elements
    let mut results = vec![];
    for combination in repeat_n(set.iter(), num_elements).multi_cartesian_product() {
        // println!("{:?}", combination);
        results.push(combination.into_iter().cloned().collect());
    }
    results
}

/// Create all combinations of length num_elements from all set items
/// i.e. [1,2,3] with num_elements = 2 -> [[1,1],[1,2],[1,3],[2,1],[2,2],[2,3],[3,1],[3,2],[3,3]]
pub fn create_n_choice_sets_ordered<T>(set: &[T], num_elements: usize) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    //Create all combinations of operations for the number of elements
    let mut results = vec![];
    for combination in repeat_n(set.iter(), num_elements).multi_cartesian_product() {
        // println!("{:?}", combination);
        results.push(combination.into_iter().cloned().collect());
    }
    results
}

/// Create all combinations of length num_elements from all set items, treating both numbers equal rank
/// i.e. [1,2,3] with num_elements = 2 -> [[1,1],[1,2],[1,3],[2,2],[2,3],[3,3]]
pub fn create_n_choice_sets_unordered<T>(set: &[T], num_elements: usize) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    set.iter()
        .combinations_with_replacement(num_elements)
        .map(|combination| combination.into_iter().cloned().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_n_choice_sets() {
        // Test with numbers
        let set = [1, 2, 3];
        let result = create_n_choice_sets_ordered(&set, 2);
        let expected = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![2, 1],
            vec![2, 2],
            vec![2, 3],
            vec![3, 1],
            vec![3, 2],
            vec![3, 3],
        ];
        assert_eq!(result, expected);

        // Test with single element
        let set = [5];
        let result = create_n_choice_sets_ordered(&set, 3);
        let expected = vec![vec![5, 5, 5]];
        assert_eq!(result, expected);

        // Test with zero elements
        let set = [1, 2];
        let result = create_n_choice_sets_ordered(&set, 0);
        let expected = vec![vec![]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_n_choice_sets_unordered() {
        // Test with numbers
        let set = [1, 2, 3];
        let result = create_n_choice_sets_unordered(&set, 2);
        let expected = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![2, 2],
            vec![2, 3],
            vec![3, 3],
        ];
        assert_eq!(result, expected);
    }
}
