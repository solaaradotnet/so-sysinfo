use anyhow::Error;

use crate::args::VisualToggles;

use super::{SystemComponent, WindowManager};

impl SystemComponent for WindowManager {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        // windows get_wm() stub...
        // TODO: do this better
        Ok(vec!["dwm".to_string()])
    }
}
impl SystemComponent for DesktopEnvironment {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
        // windows get_de() stub...
        // TODO: do this better
        Ok(vec!["Aero".to_string()])
    }
}
impl SystemComponent for TerminalEmulator {
    fn collect_info(visual_toggles: &VisualToggles) -> Result<Vec<String>, Error> {
        // windows get_terminal() stub...
        // TODO: do this better
        Ok(vec!["Unknown".to_string()])
    }
}
