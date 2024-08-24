use anyhow::{Error, Result};
use libmacchina::{traits::GeneralReadout as _, GeneralReadout};

lazy_static::lazy_static! {
    static ref SYSINFO_DATA: sysinfo::System = sysinfo::System::new_all();
    static ref OS_INFO_DATA: os_info::Info = os_info::get();
    static ref LIBMACCHINA_READOUT: GeneralReadout = libmacchina::GeneralReadout::new();
}

pub(crate) fn get_system_memory() -> String {
    human_bytes::human_bytes(SYSINFO_DATA.total_memory() as f64)
}

pub(crate) fn get_cpu() -> Result<String> {
    let cores = LIBMACCHINA_READOUT
        .cpu_cores()
        .map_err(|_| Error::msg("Failed to get CPU core count."))?;

    let cpu_model = LIBMACCHINA_READOUT
        .cpu_model_name()
        .map_err(|_| Error::msg("Failed to get CPU model name."))?;

    Ok(format!("{cores}x {cpu_model}"))
}

pub(crate) fn get_os() -> Result<String> {
    let os_name = LIBMACCHINA_READOUT
        .os_name()
        .or_else(|_| LIBMACCHINA_READOUT.distribution())
        .unwrap_or("Unknown".to_string());

    let arch = OS_INFO_DATA
        .architecture()
        .map_or_else(|| "".to_string(), |arch| format!(" ({arch})"));

    Ok(format!("{os_name}{arch}"))
}

pub(crate) fn get_model() -> String {
    LIBMACCHINA_READOUT
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
    LIBMACCHINA_READOUT
        .desktop_environment()
        //.map_err(|_| Error::msg("Failed to get desktop environment")) TODO: do this better
        .or(Ok("N/A".to_string()))
}

pub(crate) fn get_wm() -> Result<String> {
    LIBMACCHINA_READOUT
        .window_manager()
        //.map_err(|_| Error::msg("Failed to get window manager")) TODO: do this better
        .or(Ok("N/A".to_string()))
}

pub(crate) fn get_terminal() -> Result<String> {
    LIBMACCHINA_READOUT
        .terminal()
        //.map_err(|_| Error::msg("Failed to get terminal application")) TODO: do this better
        .or(Ok("N/A".to_string()))
}

pub(crate) fn get_hostname() -> Result<String> {
    LIBMACCHINA_READOUT
        .hostname()
        .map_err(|_| Error::msg("Failed to get hostname."))
}
