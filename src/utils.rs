use libmacchina::{traits::GeneralReadout as _, GeneralReadout};
use sysinfo::get_current_pid;

pub fn get_system_memory(system_info: &sysinfo::System) -> String {
    human_bytes::human_bytes(system_info.total_memory() as f64)
}

pub fn get_cpu(readout: &GeneralReadout) -> String {
    let cores = readout.cpu_cores().expect("Failed to get cpu cores");

    let cpu_model = readout.cpu_model_name().expect("Failed to get cpu model");

    format!("{cores}x {cpu_model}")
}

pub fn get_os(readout: &GeneralReadout, os_info: &os_info::Info) -> String {
    readout.os_name().unwrap() + " " + os_info.architecture().unwrap_or_default()
}

pub fn get_model(readout: &GeneralReadout) -> String {
    readout.machine().expect("Couldn't get machine info")
}

pub fn get_shell(system_info: &sysinfo::System) -> String {
    let process = system_info
        .process(get_current_pid().unwrap())
        .expect("Process with current pid does not exist");
    let parent = system_info
        .process(process.parent().unwrap())
        .expect("Process with parent pid does not exist");
    let shell = parent.name().to_string_lossy().trim().to_lowercase();
    let shell = shell.strip_suffix(".exe").unwrap_or(&shell); // windows bad
    let shell = shell.strip_prefix('-').unwrap_or(shell); // login shells
    shell.to_string()
}

pub fn get_de(readout: &GeneralReadout) -> String {
    readout
        .desktop_environment()
        .expect("Failed to get desktop environment")
}

pub fn get_wm(readout: &GeneralReadout) -> String {
    readout
        .window_manager()
        .expect("Failed to get window manager")
}

pub fn get_terminal(readout: &GeneralReadout) -> String {
    readout
        .terminal()
        .expect("Failed to get terminal application")
}
