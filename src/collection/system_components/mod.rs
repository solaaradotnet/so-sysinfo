use anyhow::Error;
use libmacchina::{traits::GeneralReadout as _, traits::MemoryReadout as _};
use tracing::debug;

use crate::args::VisualToggles;

#[cfg(target_os = "macos")]
mod impl_macos;

#[cfg(target_os = "windows")]
mod impl_windows;

lazy_static::lazy_static! {
    static ref SYSINFO_DATA: sysinfo::System = sysinfo::System::new_all();
    static ref OS_INFO_DATA: os_info::Info = os_info::get();
    static ref LIBMACCHINA_GENERAL_READOUT: libmacchina::GeneralReadout = libmacchina::GeneralReadout::new();
    static ref LIBMACCHINA_MEMORY_READOUT: libmacchina::MemoryReadout = libmacchina::MemoryReadout::new();
}

pub(crate) trait SystemComponent {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error>;
}

pub(crate) struct Cpu;
pub(crate) struct SystemMemory;
pub(crate) struct Gpu;
pub(crate) struct BoardModel;
pub(crate) struct OperatingSystem;
pub(crate) struct CurrentShell;
pub(crate) struct TerminalEmulator;
pub(crate) struct WindowManager;
pub(crate) struct DesktopEnvironment;
pub(crate) struct Hostname;

impl SystemComponent for Cpu {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        // TODO: switch back to using libmacchina for this when the windows PRs get merged
        //       (PR url: https://github.com/Macchina-CLI/libmacchina/pull/145)
        let cores = num_cpus::get();

        let cpu_model = LIBMACCHINA_GENERAL_READOUT
            .cpu_model_name()
            .map_err(|_| Error::msg("Failed to get CPU model name."))?;

        Ok(vec![format!("{cores}x {cpu_model}")])
    }
}

impl SystemComponent for SystemMemory {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        let total_memory_in_kb = LIBMACCHINA_MEMORY_READOUT.total().unwrap();
        Ok(vec![human_bytes::human_bytes(
            (total_memory_in_kb * 1024) as f64,
        )])
    }
}

impl SystemComponent for Gpu {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        Err(Error::msg("unimplemented"))
    }
}

impl SystemComponent for BoardModel {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        Ok(vec![LIBMACCHINA_GENERAL_READOUT
            .machine()
            .unwrap_or("Generic".to_string())])
    }
}

impl SystemComponent for OperatingSystem {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        let os_name = LIBMACCHINA_GENERAL_READOUT
            .os_name()
            .or_else(|_| LIBMACCHINA_GENERAL_READOUT.distribution())
            .unwrap_or("Unknown".to_string());

        let arch = OS_INFO_DATA
            .architecture()
            .map_or_else(|| "".to_string(), |arch| format!(" ({arch})"));

        Ok(vec![format!("{os_name}{arch}")])
    }
}

impl SystemComponent for CurrentShell {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        let current_pid =
            sysinfo::get_current_pid().map_err(|_| Error::msg("Failed to get current PID."))?;
        debug!("current_pid: {current_pid}");
        let current_process = SYSINFO_DATA
            .process(current_pid)
            .ok_or(Error::msg("Process with current pid does not exist"))?;
        debug!("current_process: {current_process:?}");

        let mut parent_pid = current_process
            .parent()
            .ok_or(Error::msg("Failed to get parent process."))?;
        debug!("parent_pid: {parent_pid}");
        let mut parent_process = SYSINFO_DATA
            .process(parent_pid)
            .expect("Process with parent pid does not exist");
        debug!("parent_process: {parent_process:?}");

        let mut shell = parent_process
            .name()
            .to_string_lossy()
            .trim()
            .to_lowercase();
        debug!("shell: {shell}");

        shell = shell
            .strip_prefix('-')
            .unwrap_or(&shell)
            .strip_suffix(".exe")
            .unwrap_or(&shell)
            .to_string();
        debug!("shell(cleaned): {shell}");

        // recursively get parent shell process if needed
        while matches!(shell.as_ref(), "cargo") {
            parent_pid = parent_process
                .parent()
                .ok_or(Error::msg("Failed to get parent process."))?;
            debug!("parent_pid: {parent_pid}");
            parent_process = SYSINFO_DATA
                .process(parent_pid)
                .expect("Process with parent pid does not exist");
            debug!("parent_process: {parent_process:?}");

            shell = parent_process
                .name()
                .to_string_lossy()
                .trim()
                .to_lowercase();
            debug!("shell: {shell}");

            shell = shell
                .strip_prefix('-')
                .unwrap_or(&shell)
                .strip_suffix(".exe")
                .unwrap_or(&shell)
                .to_string();
            debug!("shell(cleaned): {shell}");
        }

        Ok(vec![shell.to_string()])
    }
}

#[cfg(not(target_os = "windows"))]
impl SystemComponent for TerminalEmulator {
    fn collect_info(visual_toggles: &VisualToggles) -> Result<Vec<String>, Error> {
        if visual_toggles.hide_terminal_version {
            std::env::remove_var("TERM_PROGRAM_VERSION")
        }
        Ok(vec![LIBMACCHINA_GENERAL_READOUT.terminal().map_err(
            |_| Error::msg("Failed to get terminal application"),
        )?])
    }
}

#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "macos"))]
impl SystemComponent for WindowManager {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        Ok(vec![LIBMACCHINA_GENERAL_READOUT
            .window_manager()
            .map_err(|_| Error::msg("Failed to get window manager"))?])
    }
}

#[cfg(not(target_os = "windows"))]
impl SystemComponent for DesktopEnvironment {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        Ok(vec![LIBMACCHINA_GENERAL_READOUT
            .desktop_environment()
            .map_err(|_| {
                Error::msg("Failed to get desktop environment")
            })?])
    }
}

impl SystemComponent for Hostname {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        Ok(vec![LIBMACCHINA_GENERAL_READOUT
            .hostname()
            .map_err(|_| Error::msg("Failed to get hostname."))?])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testresult::TestResult;
    use tracing_test::traced_test;

    #[test]
    fn test_get_cpu() -> TestResult {
        let info = Cpu::collect_info(&VisualToggles::default())?;
        assert!(!info.is_empty());
        Ok(())
    }

    #[test]
    fn test_get_system_memory() -> TestResult {
        let info = SystemMemory::collect_info(&VisualToggles::default())?;
        assert!(!info.is_empty());
        Ok(())
    }

    #[test]
    fn test_get_os() -> TestResult {
        let info = OperatingSystem::collect_info(&VisualToggles::default())?;
        assert!(!info.is_empty());
        Ok(())
    }

    #[test]
    fn test_get_model() -> TestResult {
        let _info = BoardModel::collect_info(&VisualToggles::default())?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_shell() -> TestResult {
        let _info = CurrentShell::collect_info(&VisualToggles::default())?;
        Ok(())
    }

    #[test]
    fn test_get_de() -> TestResult {
        let _info = DesktopEnvironment::collect_info(&VisualToggles::default())?;
        Ok(())
    }

    #[test]
    fn test_get_wm() -> TestResult {
        let _info = WindowManager::collect_info(&VisualToggles::default())?;
        Ok(())
    }

    #[test]
    fn test_get_terminal_with_version() -> TestResult {
        let _info = TerminalEmulator::collect_info(&VisualToggles {
            hide_terminal_version: true,
        })?;

        Ok(())
    }

    #[test]
    fn test_get_terminal_without_version() -> TestResult {
        let _info = TerminalEmulator::collect_info(&VisualToggles {
            hide_terminal_version: false,
        })?;

        Ok(())
    }

    #[test]
    fn test_get_hostname() -> TestResult {
        let _info = Hostname::collect_info(&VisualToggles::default())?;
        Ok(())
    }
}
