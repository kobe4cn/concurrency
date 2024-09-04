use concurrency::Matrix;
fn main() -> anyhow::Result<()> {
    let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
    let b = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    let c = a * b;
    println!("{}", c?);

    Ok(())
}
