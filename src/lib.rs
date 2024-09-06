mod matrix;
mod metrics;
mod vector;
pub use matrix::multiply;
pub use matrix::Matrix;
pub use metrics::{AmapMetrics, CmapMetrics};
pub use vector::{dot_product, Vector};
