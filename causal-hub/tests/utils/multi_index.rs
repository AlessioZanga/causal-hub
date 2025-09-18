#[cfg(test)]
mod tests {
    use causal_hub::utils::MI;
    use ndarray::prelude::*;

    #[test]
    fn multi_index_ravel() {
        // Set dimensions.
        let dim = array![2, 3, 4];
        // Create a multi index.
        let multi_index = MI::new(dim.clone());

        // Assert the shape.
        assert_eq!(multi_index.shape(), &dim);

        // Assert the ravel multi index explicitly.
        assert_eq!(multi_index.ravel([0, 0, 0]), 0);
        assert_eq!(multi_index.ravel([0, 0, 1]), 1);
        assert_eq!(multi_index.ravel([0, 0, 2]), 2);
        assert_eq!(multi_index.ravel([0, 0, 3]), 3);
        assert_eq!(multi_index.ravel([0, 1, 0]), 4);
        assert_eq!(multi_index.ravel([0, 1, 1]), 5);
        assert_eq!(multi_index.ravel([1, 0, 0]), 12);

        // Assert the ravel multi index implicitly.
        for i in 0..dim[0] {
            for j in 0..dim[1] {
                for k in 0..dim[2] {
                    let ravel_index = multi_index.ravel([i, j, k]);
                    let expected_index = i * dim[1] * dim[2] + j * dim[2] + k;
                    assert_eq!(ravel_index, expected_index);
                }
            }
        }
    }

    #[test]
    fn multi_index_unravel() {
        // Set dimensions.
        let dim = array![2, 3, 4];
        // Create a multi index.
        let multi_index = MI::new(dim.clone());

        // Assert the unravel multi index explicitly.
        assert_eq!(multi_index.unravel(0), vec![0, 0, 0]);
        assert_eq!(multi_index.unravel(1), vec![0, 0, 1]);
        assert_eq!(multi_index.unravel(2), vec![0, 0, 2]);
        assert_eq!(multi_index.unravel(3), vec![0, 0, 3]);
        assert_eq!(multi_index.unravel(4), vec![0, 1, 0]);
        assert_eq!(multi_index.unravel(5), vec![0, 1, 1]);
        assert_eq!(multi_index.unravel(12), vec![1, 0, 0]);

        // Assert the unravel multi index implicitly.
        for i in 0..dim[0] {
            for j in 0..dim[1] {
                for k in 0..dim[2] {
                    let ravel_index = i * dim[1] * dim[2] + j * dim[2] + k;
                    let multi_index_value = multi_index.unravel(ravel_index);
                    assert_eq!(multi_index_value, vec![i, j, k]);
                }
            }
        }
    }
}
