use MatrixLib::Matrix;

fn main() {
    println!("Hello, world!");
    let mat = Matrix::<u32>::new(2,2);
    println!("==========");
    println!("{:?}", mat);
    println!("==========");
    mat.print();
    println!("==========");
    println!("{}", mat);
}
