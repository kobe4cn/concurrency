//metrics data structure

//basic function increase decrease snapshot

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics<T> {
    data: Arc<RwLock<HashMap<String, T>>>,
}
#[allow(clippy::new_without_default)]
impl<T> Metrics<T>
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy + Default + Sync,
{
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn increase(&self, key: &str, value: T) -> anyhow::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("lock failed {}", e))?;
        let counter = data.entry(key.to_string()).or_insert_with(|| T::default());
        *counter += value;
        Ok(())
    }
    pub fn decrease(&self, key: &str, value: T) -> anyhow::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!("lock failed {}", e))?;
        let counter = data.entry(key.to_string()).or_insert_with(|| T::default());
        *counter -= value;
        Ok(())
    }
    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, T>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}
