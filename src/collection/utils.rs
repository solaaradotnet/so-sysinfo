use anyhow::{Error, Result};
use libmacchina::{
    traits::GeneralReadout as _, traits::MemoryReadout as _, GeneralReadout, MemoryReadout,
};

use crate::args::VisualToggles;

lazy_static::lazy_static! {
    static ref SYSINFO_DATA: sysinfo::System = sysinfo::System::new_all();
    static ref OS_INFO_DATA: os_info::Info = os_info::get();
    static ref LIBMACCHINA_GENERAL_READOUT: GeneralReadout = libmacchina::GeneralReadout::new();
    static ref LIBMACCHINA_MEMORY_READOUT: MemoryReadout = libmacchina::MemoryReadout::new();
}

pub(crate) fn get_cpu() -> Result<String> {
    let cores = LIBMACCHINA_GENERAL_READOUT
        .cpu_cores()
        .map_err(|_| Error::msg("Failed to get CPU core count."))?;

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
    let current_process = SYSINFO_DATA
        .process(current_pid)
        .ok_or(Error::msg("Process with current pid does not exist"))?;

    let parent_pid = current_process
        .parent()
        .ok_or(Error::msg("Failed to get parent process."))?;
    let parent_process = SYSINFO_DATA
        .process(parent_pid)
        .expect("Process with parent pid does not exist");

    let shell = parent_process
        .name()
        .to_string_lossy()
        .trim()
        .to_lowercase();
    let shell = shell.strip_suffix(".exe").unwrap_or(&shell); // windows bad
    let shell = shell.strip_prefix('-').unwrap_or(shell); // login shells

    Ok(shell.to_string())
}

pub(crate) fn get_de() -> Result<String> {
    LIBMACCHINA_GENERAL_READOUT
        .desktop_environment()
        .map_err(|_| Error::msg("Failed to get desktop environment"))
}

pub(crate) fn get_wm() -> Result<String> {
    LIBMACCHINA_GENERAL_READOUT
        .window_manager()
        .map_err(|_| Error::msg("Failed to get window manager"))
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
