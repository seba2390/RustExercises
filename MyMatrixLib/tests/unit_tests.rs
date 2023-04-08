////////////////////////////////////////////////////////////////////////////////////////////////////
///// Testing initialization methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 /////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod initializers {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 289; // Must have integer square root.
    const MIN_SIZE: usize = 1;

    fn test_matrix_new_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        for size in MIN_SIZE..MAX_SIZE {
            let new_mat = Matrix::<T>::new(size,size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(new_mat[(row_idx, col_idx)], T::default());
                }
            }
        }
    }

    #[test]
    fn test_matrix_new_creation() {
        test_matrix_new_creation_::<i8>();
        test_matrix_new_creation_::<i16>();
        test_matrix_new_creation_::<i32>();
        test_matrix_new_creation_::<i64>();
        test_matrix_new_creation_::<u8>();
        test_matrix_new_creation_::<u16>();
        test_matrix_new_creation_::<u32>();
        test_matrix_new_creation_::<u64>();
        test_matrix_new_creation_::<f32>();
        test_matrix_new_creation_::<f64>();
    }

    fn test_matrix_from_vec_creation_<T>()
        where
            T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::fmt::Display
            + num_traits::Zero
            + num_traits::One
            + num_traits::NumCast
            + std::fmt::Debug
            + PartialEq
            + Default
            + Clone
            + Copy,

    {
        'outer_loop: for root_size in MIN_SIZE..=(MAX_SIZE as f64).sqrt() as usize {
            let size = root_size * root_size;

            let mut vector: Vec<T> = vec![T::default(); size];
            for val in 0..size {
                // Doing this to stop at the point where val becomes to big to be represented by type T
                if let Some(num) = num_traits::NumCast::from(val+1).and_then(|n:T| n.into()) {
                    vector[val] = num;
                } else {
                    break 'outer_loop;
                }
            }

            let from_vec_mat = Matrix::<T>::from_vec(root_size, root_size, vector);
            let mut counter: T = T::one();
            for row_idx in 0..root_size {
                for col_idx in 0..root_size {
                    assert_eq!(from_vec_mat[(row_idx, col_idx)], counter);
                    counter = counter + T::one();
                }
            }
        }
    }

    #[test]
    fn test_matrix_from_vec_creation() {
        test_matrix_from_vec_creation_::<i8>();
        test_matrix_from_vec_creation_::<i16>();
        test_matrix_from_vec_creation_::<i32>();
        test_matrix_from_vec_creation_::<i64>();
        test_matrix_from_vec_creation_::<u8>();
        test_matrix_from_vec_creation_::<u16>();
        test_matrix_from_vec_creation_::<u32>();
        test_matrix_from_vec_creation_::<u64>();
        test_matrix_from_vec_creation_::<f32>();
        test_matrix_from_vec_creation_::<f64>();
    }


    fn test_matrix_ones_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        for size in MIN_SIZE..MAX_SIZE {
            let ones_mat = Matrix::<T>::ones(size,size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(ones_mat[(row_idx, col_idx)], T::one());
                }
            }
        }
    }

    #[test]
    fn test_matrix_ones_creation() {
        test_matrix_ones_creation_::<i8>();
        test_matrix_ones_creation_::<i16>();
        test_matrix_ones_creation_::<i32>();
        test_matrix_ones_creation_::<i64>();
        test_matrix_ones_creation_::<u8>();
        test_matrix_ones_creation_::<u16>();
        test_matrix_ones_creation_::<u32>();
        test_matrix_ones_creation_::<u64>();
        test_matrix_ones_creation_::<f32>();
        test_matrix_ones_creation_::<f64>();
    }

    fn test_matrix_zeros_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        for size in MIN_SIZE..MAX_SIZE {
            let zeros_mat = Matrix::<T>::zeros(size,size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(zeros_mat[(row_idx, col_idx)], T::zero());
                }
            }
        }
    }

    #[test]
    fn test_matrix_zeros_creation() {
        test_matrix_zeros_creation_::<i8>();
        test_matrix_zeros_creation_::<i16>();
        test_matrix_zeros_creation_::<i32>();
        test_matrix_zeros_creation_::<i64>();
        test_matrix_zeros_creation_::<u8>();
        test_matrix_zeros_creation_::<u16>();
        test_matrix_zeros_creation_::<u32>();
        test_matrix_zeros_creation_::<u64>();
        test_matrix_zeros_creation_::<f32>();
        test_matrix_zeros_creation_::<f64>();
    }

    fn test_matrix_identity_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        for size in MIN_SIZE..MAX_SIZE {
            let identity_mat = Matrix::<T>::identity(size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    if col_idx == row_idx {
                        assert_eq!(identity_mat[(row_idx, col_idx)], T::one());
                    }
                    else {
                        assert_eq!(identity_mat[(row_idx, col_idx)], T::zero());
                    }
                }
            }
        }
    }

    #[test]
    fn test_matrix_identity_creation() {
        test_matrix_identity_creation_::<i8>();
        test_matrix_identity_creation_::<i16>();
        test_matrix_identity_creation_::<i32>();
        test_matrix_identity_creation_::<i64>();
        test_matrix_identity_creation_::<u8>();
        test_matrix_identity_creation_::<u16>();
        test_matrix_identity_creation_::<u32>();
        test_matrix_identity_creation_::<u64>();
        test_matrix_identity_creation_::<f32>();
        test_matrix_identity_creation_::<f64>();
    }
}
