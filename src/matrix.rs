#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: std::ops::Mul<Output = T>
        + std::ops::AddAssign
        + Copy
        + Default
        + std::fmt::Debug
        + std::fmt::Display
        + std::ops::Add<Output = T>,
{
    if a.cols != b.rows {
        anyhow::bail!("Incompatible matrix dimensions");
    }

    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }
    Ok(Matrix::new(a.rows, b.cols, data))
}

impl<T> Matrix<T> {
    fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: std::fmt::Display,
    //display a 2*3 matrix as {1 2 3, 4 5 6}, 3*2 matrix as {1 2, 3 4, 5 6}
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }

            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

// impl<T> std::fmt::Debug for Matrix<T>
// where
//     T: std::fmt::Debug,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "Matrix {{ rows: {}, cols: {}, data: {:?} }}",
//             self.rows, self.cols, self.data
//         )
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let c = multiply(&a, &b).unwrap();
        println!("{}", c);
        assert_eq!(format!("{}", c), "{22 28, 49 64}");
    }
}
