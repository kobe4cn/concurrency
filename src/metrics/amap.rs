use core::fmt;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn increase(&self, key: impl AsRef<str>, value: i64) -> anyhow::Result<()> {
        let key = key.as_ref();
        if let Some(counter) = self.data.get(key) {
            counter.fetch_add(value, std::sync::atomic::Ordering::Relaxed);
        } else {
            return Err(anyhow::anyhow!("key not found"));
        }
        Ok(())
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(
                f,
                "{}: {}\n",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
