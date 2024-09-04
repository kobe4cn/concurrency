use std::ops::Deref;
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
    // pub fn len(&self) -> usize {
    //     self.data.len()
    // }
    // pub fn iter(&self) -> std::slice::Iter<T> {
    //     self.data.iter()
    // }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where
    T: std::ops::Mul<Output = T> + std::ops::AddAssign + Copy + Default,
{
    if a.len() != b.len() {
        anyhow::bail!("Incompatible vector dimensions");
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    Ok(result)
}
