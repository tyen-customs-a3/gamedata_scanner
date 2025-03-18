use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use log::{debug, error, info};
use regex::Regex;
use lazy_static::lazy_static;
use serde_json;
use super::types::Report;
use super::error_handler::create_annotation;

/// Generate reports for the batch parsing results
pub fn generate_report(output_dir: &Path, report: &Report) -> io::Result<()> {
    // Generate JSON report
    let json_path = output_dir.join("report.json");
    save_json_report(report, &json_path)?;
    info!("JSON report saved to: {}", json_path.display());
    
    // Generate diagnostic report
    let diagnostic_path = output_dir.join("diagnostics.log");
    write_diagnostic_report(report, &diagnostic_path)?;
    info!("Diagnostic report saved to: {}", diagnostic_path.display());
    
    // Generate summary report
    let summary_path = output_dir.join("summary.txt");
    write_summary_report(report, &summary_path)?;
    info!("Summary report saved to: {}", summary_path.display());
    
    Ok(())
}

/// Save the report in JSON format
fn save_json_report(report: &Report, path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(report)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    fs::write(path, json)?;
    debug!("JSON report saved to: {}", path.display());
    
    Ok(())
}

/// Write a detailed diagnostic report
fn write_diagnostic_report(report: &Report, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    
    // Write header
    writeln!(file, "Batch Parser Diagnostic Report")?;
    writeln!(file, "Generated at: {}", report.timestamp)?;
    writeln!(file, "{}", "-".repeat(80))?;
    writeln!(file)?;
    
    // Write statistics
    writeln!(file, "Statistics:")?;
    writeln!(file, "  Total files processed: {}", report.stats.total_files)?;
    writeln!(file, "  Successfully processed: {}", report.stats.successful_files)?;
    writeln!(file, "  Failed with errors: {}", report.stats.failed_files)?;
    writeln!(file, "  Files with warnings: {}", report.stats.warning_files)?;
    writeln!(file, "  Timeouts: {}", report.stats.timeout_files)?;
    if report.stopped_early {
        writeln!(file, "  Note: Processing stopped early due to too many failures")?;
    }
    writeln!(file)?;
    
    // Write failures
    if !report.failures.is_empty() {
        writeln!(file, "Failures:")?;
        writeln!(file, "{}", "-".repeat(80))?;
        
        for failure in &report.failures {
            writeln!(file, "File: {}", failure.related_files.first().unwrap_or(&String::new()))?;
            writeln!(file, "Category: {}", failure.error_category)?;
            writeln!(file, "Severity: {}", failure.error_severity)?;
            writeln!(file, "Error: {}", failure.error_message)?;
            
            if let Some(line) = failure.error_line_number {
                writeln!(file, "Line: {}", line)?;
            }
            
            if let Some(context) = &failure.error_context {
                writeln!(file, "\nContext:")?;
                writeln!(file, "{}", context)?;
            }
            
            if !failure.diagnostics.is_empty() {
                writeln!(file, "\nDiagnostics:")?;
                for diagnostic in &failure.diagnostics {
                    writeln!(file, "  - {}", diagnostic)?;
                }
            }
            
            writeln!(file, "\nProcessing time: {}ms", failure.parse_duration_ms)?;
            writeln!(file, "{}", "-".repeat(80))?;
            writeln!(file)?;
        }
    }
    
    debug!("Diagnostic report saved to: {}", path.display());
    Ok(())
}

/// Write a summary report
fn write_summary_report(report: &Report, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    
    writeln!(file, "Batch Parser Summary Report")?;
    writeln!(file, "Generated at: {}", report.timestamp)?;
    writeln!(file, "{}", "-".repeat(80))?;
    writeln!(file)?;
    
    // Write statistics
    writeln!(file, "Processing Summary")?;
    writeln!(file, "  Total files: {}", report.stats.total_files)?;
    writeln!(file, "  Successful: {}", report.stats.successful_files)?;
    writeln!(file, "  Failed: {}", report.stats.failed_files)?;
    writeln!(file, "  Warnings: {}", report.stats.warning_files)?;
    writeln!(file, "  Timeouts: {}", report.stats.timeout_files)?;
    if report.stopped_early {
        writeln!(file, "  Note: Processing stopped early due to too many failures")?;
    }
    writeln!(file)?;
    
    // Write error categories summary
    if !report.failures.is_empty() {
        let mut categories = std::collections::HashMap::new();
        for failure in &report.failures {
            *categories.entry(failure.error_category.clone()).or_insert(0) += 1;
        }
        
        writeln!(file, "Error Categories")?;
        for (category, count) in categories {
            writeln!(file, "  {}: {}", category, count)?;
        }
        writeln!(file)?;
    }
    
    debug!("Summary report saved to: {}", path.display());
    Ok(())
}

/// Strip ANSI color codes from a string
fn strip_ansi_codes(input: &str) -> String {
    // This regex matches standard ANSI color and control codes
    lazy_static! {
        static ref ANSI_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    }
    ANSI_REGEX.replace_all(input, "").to_string()
} 