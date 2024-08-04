use libmacchina::GeneralReadout;
use std::collections::HashMap;
use sysinfo::get_current_pid;

use gethostname::gethostname;
pub fn get_hostname() -> String {
    let hostname = gethostname();
    let potentially_fqdn = hostname.to_string_lossy();

    let hostname = potentially_fqdn.split('.').take(1).collect::<String>();

    hostname
}

pub fn get_system_memory(system_info: &sysinfo::System) -> String {
    human_bytes::human_bytes(system_info.total_memory() as f64)
}

pub fn get_cpu() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();

    let cores = general_readout
        .cpu_cores()
        .expect("Failed to get desktop environment");

    let cpu_model = general_readout
        .cpu_model_name()
        .expect("Failed to get desktop environment");

    format!("{cores}x {cpu_model}")
}

pub fn get_os(os_info: &os_info::Info) -> String {
    format!(
        "{} {} ({})",
        os_info.os_type(),
        os_info.version(),
        os_info.architecture().unwrap_or_default()
    )
}

pub fn get_model() -> String {
    crate::model_impl::get_model()
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

pub fn get_de() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();

    general_readout
        .desktop_environment()
        .expect("Failed to get desktop environment")
}

pub fn get_wm() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();

    general_readout
        .window_manager()
        .expect("Failed to get desktop environment")
}

pub fn get_terminal() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();

    general_readout
        .terminal()
        .expect("Failed to get desktop environment")
}
