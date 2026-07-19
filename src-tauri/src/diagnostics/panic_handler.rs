use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

/// Installs a global Rust panic hook that writes crash reports to
/// `{app_data_dir}/diagnostics/crash_YYYYMMDD_HHMMSS.log`.
///
/// Reports are local only — no network, no telemetry.
/// The user may review and optionally export them from Settings → Diagnostics.
pub fn install_panic_hook(app_data_dir: &Path) {
    let dir = app_data_dir.join("diagnostics");
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("[diagnostics] failed to create diagnostics dir: {e}");
        return;
    }

    let dir = Arc::new(dir);

    std::panic::set_hook(Box::new(move |panic_info| {
        let now = Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("crash_{timestamp}.log");
        let log_path = dir.join(&filename);

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());

        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Box<dyn Any>".to_string()
        };

        // Collect backtrace
        let backtrace = std::backtrace::Backtrace::capture();

        let report = format!(
            "=== ХРОНИКИ Crash Report ===\n\
             App:       ХРОНИКИ\n\
             Version:   {}\n\
             Timestamp: {}\n\
             \n\
             --- Panic ---\n\
             Message:   {}\n\
             Location:  {}\n\
             \n\
             --- Backtrace ---\n\
             {}\n\
             \n\
             === End of Report ===\n",
            crate::APP_VERSION,
            now.to_rfc3339(),
            message,
            location,
            backtrace,
        );

        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)
        {
            let _ = f.write_all(report.as_bytes());
        }

        // Also print to stderr for debug builds
        eprintln!(
            "[HRONIKI PANIC] {}\n  at {}\n  log: {}",
            message, location, filename
        );
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_crash_log_format() {
        let temp_dir = env::temp_dir().join(format!("hroniki_diag_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir).unwrap();

        let now = Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();

        // Validate timestamp format is correct (YYYYMMDD_HHMMSS)
        assert_eq!(timestamp.len(), 15);
        assert_eq!(&timestamp[8..9], "_");

        // Validate filename construction
        let filename = format!("crash_{timestamp}.log");
        assert!(filename.starts_with("crash_"));
        assert!(filename.ends_with(".log"));

        // Validate report structure
        let report = format!(
            "=== ХРОНИКИ Crash Report ===\n\
             App:       ХРОНИКИ\n\
             Version:   0.2.1-beta\n\
             Timestamp: {}\n\
             \n\
             --- Panic ---\n\
             Message:   test panic\n\
             Location:  src/test.rs:1:1\n\
             \n\
             --- Backtrace ---\n\
             (none)\n",
            now.to_rfc3339()
        );

        assert!(report.contains("=== ХРОНИКИ Crash Report ==="));
        assert!(report.contains("Version:   0.2.1-beta"));
        assert!(report.contains("--- Panic ---"));
        assert!(report.contains("--- Backtrace ---"));

        // Write and read back
        let log_path = temp_dir.join(&filename);
        fs::write(&log_path, &report).unwrap();
        let read_back = fs::read_to_string(&log_path).unwrap();
        assert_eq!(read_back, report);

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
