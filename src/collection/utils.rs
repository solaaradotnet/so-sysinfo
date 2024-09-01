use anyhow::{Error, Result};
use libmacchina::{
    traits::GeneralReadout as _, traits::MemoryReadout as _, GeneralReadout, MemoryReadout,
};
use tracing::debug;

use crate::args::VisualToggles;

lazy_static::lazy_static! {
    static ref SYSINFO_DATA: sysinfo::System = sysinfo::System::new_all();
    static ref OS_INFO_DATA: os_info::Info = os_info::get();
    static ref LIBMACCHINA_GENERAL_READOUT: GeneralReadout = libmacchina::GeneralReadout::new();
    static ref LIBMACCHINA_MEMORY_READOUT: MemoryReadout = libmacchina::MemoryReadout::new();
}

pub(crate) fn get_cpu() -> Result<String> {
    // TODO: switch back to using libmacchina for this when the windows PRs get merged
    //       (PR url: https://github.com/Macchina-CLI/libmacchina/pull/145)
    let cores = num_cpus::get();

    let cpu_model = LIBMACCHINA_GENERAL_READOUT
        .cpu_model_name()
        .map_err(|_| Error::msg("Failed to get CPU model name."))?;

    Ok(format!("{cores}x {cpu_model}"))
}

pub(crate) fn get_system_memory() -> String {
    let total_memory_in_kb = LIBMACCHINA_MEMORY_READOUT.total().unwrap();
    human_bytes::human_bytes((total_memory_in_kb * 1024) as f64)
}

pub(crate) fn get_os() -> Result<String> {
    let os_name = LIBMACCHINA_GENERAL_READOUT
        .os_name()
        .or_else(|_| LIBMACCHINA_GENERAL_READOUT.distribution())
        .unwrap_or("Unknown".to_string());

    let arch = OS_INFO_DATA
        .architecture()
        .map_or_else(|| "".to_string(), |arch| format!(" ({arch})"));

    Ok(format!("{os_name}{arch}"))
}

pub(crate) fn get_model() -> String {
    LIBMACCHINA_GENERAL_READOUT
        .machine()
        .unwrap_or("Generic".to_string())
}

pub(crate) fn get_shell() -> Result<String> {
    let current_pid =
        sysinfo::get_current_pid().map_err(|_| Error::msg("Failed to get current PID."))?;
    debug!("current_pid: {current_pid}");
    let current_process = SYSINFO_DATA
        .process(current_pid)
        .ok_or(Error::msg("Process with current pid does not exist"))?;
    debug!("current_process: {current_process:?}");

    let parent_pid = current_process
        .parent()
        .ok_or(Error::msg("Failed to get parent process."))?;
    debug!("parent_pid: {parent_pid}");
    let parent_process = SYSINFO_DATA
        .process(parent_pid)
        .expect("Process with parent pid does not exist");
    debug!("parent_process: {parent_process:?}");

    let shell = parent_process
        .name()
        .to_string_lossy()
        .trim()
        .to_lowercase();
    debug!("shell (pass 1): {shell}");
    let shell = shell.strip_suffix(".exe").unwrap_or(&shell); // windows bad
    debug!("shell (pass 2): {shell}");
    let mut shell = shell.strip_prefix('-').unwrap_or(shell).to_string(); // login shells
    debug!("shell (pass 3): {shell}");

    // recursively get parent shell process if needed
    while matches!(shell.as_ref(), "cargo") {
        let parent_pid = parent_process
            .parent()
            .ok_or(Error::msg("Failed to get parent process."))?;
        debug!("parent_pid: {parent_pid}");
        let parent_process = SYSINFO_DATA
            .process(parent_pid)
            .expect("Process with parent pid does not exist");
        debug!("parent_process: {parent_process:?}");

        let deeper_shell = parent_process
            .name()
            .to_string_lossy()
            .trim()
            .to_lowercase();
        debug!("shell (pass 1): {shell}");
        let deeper_shell = deeper_shell.strip_suffix(".exe").unwrap_or(&deeper_shell); // windows bad
        debug!("shell (pass 2): {shell}");
        shell = deeper_shell
            .strip_prefix('-')
            .unwrap_or(deeper_shell)
            .to_string(); // login shells
        debug!("shell (pass 3): {shell}");
    }

    Ok(shell.to_string())
}

pub(crate) fn get_de() -> Result<String> {
    LIBMACCHINA_GENERAL_READOUT
        .desktop_environment()
        .map_err(|_| Error::msg("Failed to get desktop environment"))
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn get_wm() -> Result<String> {
    LIBMACCHINA_GENERAL_READOUT
        .window_manager()
        .map_err(|_| Error::msg("Failed to get window manager"))
}
#[cfg(target_os = "macos")]
pub(crate) fn get_wm() -> Result<String> {
    use std::ffi::OsStr;
    for wm in [
        "chunkwm",
        "kwm",
        "yabai",
        "Amethyst",
        "Spectacle",
        "Rectangle",
    ] {
        if SYSINFO_DATA
            .processes_by_exact_name(OsStr::new(wm))
            .next()
            .is_some()
        {
            return Ok(wm.to_string());
        }
    }

    Ok("Quartz Compositor".to_string())
}

pub(crate) fn get_terminal(visual_toggles: &VisualToggles) -> Result<String> {
    if visual_toggles.hide_terminal_version {
        std::env::remove_var("TERM_PROGRAM_VERSION")
    }
    LIBMACCHINA_GENERAL_READOUT
        .terminal()
        .map_err(|_| Error::msg("Failed to get terminal application"))
}

pub(crate) fn get_hostname() -> Result<String> {
    LIBMACCHINA_GENERAL_READOUT
        .hostname()
        .map_err(|_| Error::msg("Failed to get hostname."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use testresult::TestResult;
    use tracing::Level;
    use tracing_test::traced_test;

    #[test]
    fn test_get_cpu() -> TestResult {
        let _ = get_cpu()?;
        Ok(())
    }

    #[test]
    fn test_get_system_memory() {
        let info = get_system_memory();
        assert!(!info.is_empty())
    }

    #[test]
    fn test_get_os() -> TestResult {
        let _ = get_os()?;
        Ok(())
    }

    #[test]
    fn test_get_model() {
        let info = get_model();
        assert!(!info.is_empty())
    }

    #[traced_test]
    #[test]
    fn test_get_shell() -> TestResult {
        let _ = get_shell()?;
        Ok(())
    }

    #[test]
    fn test_get_de() -> TestResult {
        let _ = get_de()?;
        Ok(())
    }

    #[test]
    fn test_get_wm() -> TestResult {
        let _ = get_wm()?;
        Ok(())
    }

    #[test]
    fn test_get_terminal_with_version() -> TestResult {
        let _ = get_terminal(&VisualToggles {
            hide_terminal_version: true,
        })?;

        Ok(())
    }

    #[test]
    fn test_get_terminal_without_version() -> TestResult {
        let _ = get_terminal(&VisualToggles {
            hide_terminal_version: false,
        })?;

        Ok(())
    }

    #[test]
    fn test_get_hostname() -> TestResult {
        let _ = get_hostname()?;
        Ok(())
    }
}
