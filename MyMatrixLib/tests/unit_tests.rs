////////////////////////////////////////////////////////////////////////////////////////////////////
///// Testing initialization methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 /////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod initializers {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 300;
    const MIN_SIZE: usize = 1;

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
