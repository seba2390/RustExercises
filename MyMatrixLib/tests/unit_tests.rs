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


////////////////////////////////////////////////////////////////////////////////////////////////////
///// Testing RNG methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 /////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod prng_initializers {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 100; // Must have integer square root.
    const MIN_SIZE: usize = 1;
    const F64_RANGE_SCALER: f64 = 0.5;
    const F32_RANGE_SCALER: f32 = 0.5;

    #[test]
    fn test_matrix_uniform_rng_creation()
    {
        for size in MIN_SIZE..MAX_SIZE {

            let rng_mat_1 =  Matrix::<i8 >::random_uniform(size, size, i8::MIN,  i8::MAX);
            let rng_mat_2 =  Matrix::<i16>::random_uniform(size, size, i16::MIN, i16::MAX);
            let rng_mat_3 =  Matrix::<i32>::random_uniform(size, size, i32::MIN, i32::MAX);
            let rng_mat_4 =  Matrix::<i64>::random_uniform(size, size, i64::MIN, i64::MAX);
            let rng_mat_5 =  Matrix::<u8 >::random_uniform(size, size, u8::MIN,  u8::MAX);
            let rng_mat_6 =  Matrix::<u16>::random_uniform(size, size, u16::MIN, u16::MAX);
            let rng_mat_7 =  Matrix::<u32>::random_uniform(size, size, u32::MIN, u32::MAX);
            let rng_mat_8 =  Matrix::<u64>::random_uniform(size, size, u64::MIN, u64::MAX);
            let rng_mat_9 =  Matrix::<f32>::random_uniform(size, size, f32::MIN*F32_RANGE_SCALER , f32::MAX*F32_RANGE_SCALER);
            let rng_mat_10 = Matrix::<f64 >::random_uniform(size, size, f64::MIN*F64_RANGE_SCALER, f64::MAX*F64_RANGE_SCALER);

            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert!(rng_mat_1[(row_idx, col_idx)]  >= i8::MIN);
                    assert!(rng_mat_1[(row_idx, col_idx)]  <  i8::MAX);

                    assert!(rng_mat_2[(row_idx, col_idx)]  >= i16::MIN);
                    assert!(rng_mat_2[(row_idx, col_idx)]  <  i16::MAX);

                    assert!(rng_mat_3[(row_idx, col_idx)]  >= i32::MIN);
                    assert!(rng_mat_3[(row_idx, col_idx)]  <  i32::MAX);

                    assert!(rng_mat_4[(row_idx, col_idx)]  >= i64::MIN);
                    assert!(rng_mat_4[(row_idx, col_idx)]  <  i64::MAX);

                    assert!(rng_mat_5[(row_idx, col_idx)]  >= u8::MIN);
                    assert!(rng_mat_5[(row_idx, col_idx)]  <  u8::MAX);

                    assert!(rng_mat_6[(row_idx, col_idx)]  >= u16::MIN);
                    assert!(rng_mat_6[(row_idx, col_idx)]  <  u16::MAX);

                    assert!(rng_mat_7[(row_idx, col_idx)]  >= u32::MIN);
                    assert!(rng_mat_7[(row_idx, col_idx)]  <  u32::MAX);

                    assert!(rng_mat_8[(row_idx, col_idx)]  >= u64::MIN);
                    assert!(rng_mat_8[(row_idx, col_idx)]  <  u64::MAX);

                    assert!(rng_mat_9[(row_idx, col_idx)]  >= f32::MIN*F32_RANGE_SCALER);
                    assert!(rng_mat_9[(row_idx, col_idx)]  <  f32::MAX*F32_RANGE_SCALER);

                    assert!(rng_mat_10[(row_idx, col_idx)]  >= f64::MIN*F64_RANGE_SCALER);
                    assert!(rng_mat_10[(row_idx, col_idx)]  <  f64::MAX*F64_RANGE_SCALER);
                }
            }
        }
    }


    fn test_matrix_bernoulli_rng_creation_<T>()
        where
            T:
            rand::distributions::uniform::SampleUniform +
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialOrd +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        const PROBABILITY: f64 = 0.5;
        for size in MIN_SIZE..MAX_SIZE {
            let rng_mat = Matrix::<T>::random_bernoulli(size,size,PROBABILITY);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert!(rng_mat[(row_idx, col_idx)] == T::zero() ||
                            rng_mat[(row_idx, col_idx)] == T::one());
                }
            }
        }
    }

    #[test]
    fn test_matrix_bernoulli_rng_creation() {
        test_matrix_bernoulli_rng_creation_::<i8>();
        test_matrix_bernoulli_rng_creation_::<i16>();
        test_matrix_bernoulli_rng_creation_::<i32>();
        test_matrix_bernoulli_rng_creation_::<i64>();
        test_matrix_bernoulli_rng_creation_::<u8>();
        test_matrix_bernoulli_rng_creation_::<u16>();
        test_matrix_bernoulli_rng_creation_::<u32>();
        test_matrix_bernoulli_rng_creation_::<u64>();
        test_matrix_bernoulli_rng_creation_::<f32>();
        test_matrix_bernoulli_rng_creation_::<f64>();
    }
}





