//! This module defines a trait for printing benchmark reports.
mod json;
mod text;

pub use json::JsonReporter;
pub use text::TextReporter;
pub use json::build_serde_report;
pub use json::Report;

use crate::report::BenchReport;

/// A trait for reporting benchmark results.
pub trait BenchReporter {
    /// Print the report to the given writer.
    fn print(&self, w: &mut dyn std::io::Write, report: &BenchReport) -> anyhow::Result<()>;
}
