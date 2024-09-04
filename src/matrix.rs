use std::{sync::mpsc, thread};

use crate::{dot_product, Vector};

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

const NUM_THREADS: usize = 4;

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}
pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}
impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: std::ops::Mul<Output = T>
        + std::ops::AddAssign
        + Copy
        + Default
        + Send
        + 'static
        + std::fmt::Debug
        + std::fmt::Display
        + std::ops::Add<Output = T>,
{
    if a.cols != b.rows {
        anyhow::bail!("Incompatible matrix dimensions");
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Error: {:?}", e);
                    };
                }
                Ok::<(), anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    //generate 4 threads which will receive the msg and do dot product

    let matrix_len = a.rows * b.cols;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();
            let col_ = Vector::new(col_data);
            let idx = i * b.cols + j;
            let input = MsgInput::new(idx, row, col_);
            let (tx, rx) = oneshot::channel::<MsgOutput<T>>();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error: {:?}", e);
            };
            receivers.push(rx);
            // let ret=rx.recv()?;
            // data[i * b.cols + j] += dot_product(row, col_)?;
        }
    }

    for rx in receivers {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
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

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
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
