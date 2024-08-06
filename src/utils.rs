use anyhow::{Error, Result};
use libmacchina::{traits::GeneralReadout as _, GeneralReadout};
use sysinfo::get_current_pid;

pub fn get_system_memory(system_info: &sysinfo::System) -> String {
    human_bytes::human_bytes(system_info.total_memory() as f64)
}

pub fn get_cpu(readout: &GeneralReadout) -> Result<String> {
    let cores = readout
        .cpu_cores()
        .map_err(|_| Error::msg("Failed to get CPU core count."))?;

    let cpu_model = readout
        .cpu_model_name()
        .map_err(|_| Error::msg("Failed to get CPU model name."))?;

    Ok(format!("{cores}x {cpu_model}"))
}

pub fn get_os(readout: &GeneralReadout, os_info: &os_info::Info) -> Result<String> {
    let os_name = readout
        .os_name()
        .or_else(|_| readout.distribution())
        .unwrap_or("Unknown".to_string());

    let arch = os_info
        .architecture()
        .map_or_else(|| "".to_string(), |arch| format!(" ({arch})"));

    Ok(format!("{os_name}{arch}"))
}

pub fn get_model(readout: &GeneralReadout) -> String {
    readout.machine().unwrap_or("Generic".to_string())
}

pub fn get_shell(system_info: &sysinfo::System) -> Result<String> {
    let current_pid = get_current_pid().map_err(|_| Error::msg("Failed to get current PID."))?;
    let current_process = system_info
        .process(current_pid)
        .ok_or(Error::msg("Process with current pid does not exist"))?;

    let parent_pid = current_process
        .parent()
        .ok_or(Error::msg("Failed to get parent process."))?;
    let parent_process = system_info
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

pub fn get_de(readout: &GeneralReadout) -> Result<String> {
    readout
        .desktop_environment()
        //.map_err(|_| Error::msg("Failed to get desktop environment")) TODO: do this better
        .or(Ok("N/A".to_string()))
}

pub fn get_wm(readout: &GeneralReadout) -> Result<String> {
    readout
        .window_manager()
        //.map_err(|_| Error::msg("Failed to get window manager")) TODO: do this better
        .or(Ok("N/A".to_string()))
}

pub fn get_terminal(readout: &GeneralReadout) -> Result<String> {
    readout
        .terminal()
        //.map_err(|_| Error::msg("Failed to get terminal application")) TODO: do this better
        .or(Ok("N/A".to_string()))
}
