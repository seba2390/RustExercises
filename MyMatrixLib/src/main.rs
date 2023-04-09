use MyMatrixLib::Matrix;

fn main() {
    println!("Hello, world!");
    let mat = Matrix::<u32>::new(2,2);
    println!("==========");
    println!("{:?}", mat);
    println!("==========");
    mat.print();
    println!("==========");
    println!("{}", mat);
    println!("==========");
    let mat_2 = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
    println!("{}", mat_2);
    println!("==========");
    let nums: Vec<i32> = (1..=1).collect();
    println!("{:?}", nums);
    println!("==========");
    let rng_mat_1 = Matrix::<u8>::random_uniform(5,5,1,10);
    rng_mat_1.print();
    println!("==========");
    println!("row: {} is: {:?}", 2, rng_mat_1.view_row(2));
    println!("==========");
    let rng_mat_2 = Matrix::<f32>::random_uniform(5,5,-10.0,10.0);
    rng_mat_2.print();
    println!("==========");
    let row_2 = rng_mat_2.view_row(2);
    println!("row: {} is: {:?}", 2, row_2);
    println!("row abs max: {} is: {}", 2, MyMathLib::abs_max(row_2));
    println!("row arg abs max: {} is: {}", 2, MyMathLib::arg_abs_max(row_2));
    println!("==========");
    let rng_mat_3 = Matrix::<u32>::random_bernoulli(5,5,0.5);
    rng_mat_3.print();

}
