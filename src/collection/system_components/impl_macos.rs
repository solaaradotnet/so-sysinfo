use anyhow::Error;

use crate::args::VisualToggles;

use super::{SystemComponent, WindowManager, SYSINFO_DATA};

impl SystemComponent for WindowManager {
    fn collect_info(_: &VisualToggles) -> Result<Vec<String>, Error> {
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
                return Ok(vec![wm.to_string()]);
            }
        }

        Ok(vec!["Quartz Compositor".to_string()])
    }
}
