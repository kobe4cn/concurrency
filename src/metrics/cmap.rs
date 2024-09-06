//metrics data structure

//basic function increase decrease snapshot

use core::fmt;
use std::sync::Arc;

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct CmapMetrics<T> {
    data: Arc<DashMap<String, T>>,
}
#[allow(clippy::new_without_default)]
impl<T> CmapMetrics<T>
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + Default + Sync,
{
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()),
        }
    }
    pub fn increase(&self, key: &str, value: T) -> anyhow::Result<()> {
        // let mut data = self
        //     .data
        //     .
        //     .map_err(|e| anyhow::anyhow!("lock failed {}", e))?;
        let mut counter = self
            .data
            .entry(key.to_string())
            .or_insert_with(|| T::default());
        *counter += value;
        Ok(())
    }
    pub fn decrease(&self, key: &str, value: T) -> anyhow::Result<()> {
        // let mut data = self
        //     .data
        //     .write()
        //     .map_err(|e| anyhow::anyhow!("lock failed {}", e))?;
        let mut counter = self
            .data
            .entry(key.to_string())
            .or_insert_with(|| T::default());
        *counter -= value;
        Ok(())
    }
    // pub fn snapshot(&self) -> anyhow::Result<HashMap<String, T>> {
    //     Ok(self
    //         .data
    //         .read()
    //         .map_err(|e| anyhow::anyhow!(e.to_string()))?
    //         .clone())
    // }
}
impl<T> fmt::Display for CmapMetrics<T>
where
    T: fmt::Display + std::ops::AddAssign + std::ops::SubAssign + Copy + Default + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let data = self.data;
        for entry in self.data.iter() {
            writeln!(f, "{}: {}\n", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
