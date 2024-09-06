use anyhow::Result;
use std::{cmp::max, collections::HashMap, fmt::Display};
use strum::IntoEnumIterator;
use tui_nodes::Connection;

use crate::args::VisualToggles;

pub(crate) mod system_components;
use system_components::{
    BoardModel, Cpu, CurrentShell, DesktopEnvironment, Gpu, OperatingSystem, SystemComponent,
    SystemMemory, TerminalEmulator, WindowManager,
};

#[derive(strum::EnumIter, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum SystemComponentKind {
    Cpu,
    SystemMemory,
    Gpu,
    BoardModel,
    OperatingSystem,
    CurrentShell,
    TerminalEmulator,
    WindowManager,
    DesktopEnvironment,
}

impl Display for SystemComponentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "[ {} ]",
            match self {
                SystemComponentKind::Cpu => "CPU",
                SystemComponentKind::SystemMemory => "RAM",
                SystemComponentKind::Gpu => "GPU",
                SystemComponentKind::OperatingSystem => "OS",
                SystemComponentKind::BoardModel => "Model",
                SystemComponentKind::CurrentShell => "Shell",
                SystemComponentKind::TerminalEmulator => "Terminal",
                SystemComponentKind::DesktopEnvironment => "DE",
                SystemComponentKind::WindowManager => "WM",
            }
        )
    }
}

type ComponentLinks = &'static [SystemComponentKind];

impl SystemComponentKind {
    pub fn title(&self) -> &'static str {
        match self {
            SystemComponentKind::Cpu => "[ CPU ]",
            SystemComponentKind::SystemMemory => "[ RAM ]",
            SystemComponentKind::Gpu => "[ GPU ]",
            SystemComponentKind::BoardModel => "[ Model ]",
            SystemComponentKind::OperatingSystem => "[ OS ]",
            SystemComponentKind::CurrentShell => "[ Shell ]",
            SystemComponentKind::TerminalEmulator => "[ Terminal ]",
            SystemComponentKind::DesktopEnvironment => "[ DE ]",
            SystemComponentKind::WindowManager => "[ WM ]",
        }
    }
    pub fn collect_info(&self, vt: &VisualToggles) -> Result<Vec<String>> {
        match self {
            SystemComponentKind::Cpu => Cpu::collect_info(vt),
            SystemComponentKind::SystemMemory => SystemMemory::collect_info(vt),
            SystemComponentKind::BoardModel => BoardModel::collect_info(vt),
            SystemComponentKind::CurrentShell => CurrentShell::collect_info(vt),
            SystemComponentKind::TerminalEmulator => TerminalEmulator::collect_info(vt),
            SystemComponentKind::DesktopEnvironment => DesktopEnvironment::collect_info(vt),
            SystemComponentKind::WindowManager => WindowManager::collect_info(vt),
            SystemComponentKind::OperatingSystem => OperatingSystem::collect_info(vt),
            SystemComponentKind::Gpu => Gpu::collect_info(vt),
        }
    }

    pub const fn get_links(&self) -> ComponentLinks {
        match self {
            Self::Cpu => &[Self::BoardModel],

            Self::SystemMemory => &[Self::BoardModel],

            Self::Gpu => &[Self::BoardModel],

            Self::BoardModel => &[Self::OperatingSystem],

            Self::OperatingSystem => &[
                Self::TerminalEmulator,
                Self::DesktopEnvironment,
                Self::WindowManager,
            ],

            Self::TerminalEmulator => &[Self::CurrentShell],

            Self::CurrentShell => &[],

            Self::DesktopEnvironment => &[],

            Self::WindowManager => &[],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CollectedNode {
    pub width: u16,
    pub height: u16,
    pub title: &'static str,
    pub body: String,
}

pub(crate) fn collect(
    visual_toggles: VisualToggles,
) -> Result<(Vec<CollectedNode>, Vec<Connection>)> {
    type ComponentId = usize;
    let mut component_id_acc: ComponentId = 0;

    let mut ports: HashMap<ComponentId, (usize, usize)> =
        HashMap::with_capacity(SystemComponentKind::iter().len());

    let components: Vec<_> = SystemComponentKind::iter()
        .map(|k| (k, k.collect_info(&visual_toggles)))
        .flat_map(|(kind, component_info_outer)| match component_info_outer {
            Err(_) => vec![(0, kind, None)],
            Ok(component_info) => component_info
                .into_iter()
                .map(|info_string| {
                    let component_id = component_id_acc;
                    component_id_acc += 1;
                    ports.insert(component_id, (0, 0));
                    (component_id, kind, Some(format!(" {info_string} ")))
                })
                .collect(),
        })
        .collect();

    let links: Vec<_> = components
        .iter()
        .flat_map(|(idx, kind, _info)| {
            components
                .iter()
                .filter(|component| match kind {
                    SystemComponentKind::SystemMemory | SystemComponentKind::Cpu => {
                        component.1 == SystemComponentKind::BoardModel
                    }
                    SystemComponentKind::BoardModel => {
                        component.1 == SystemComponentKind::OperatingSystem
                    }
                    SystemComponentKind::OperatingSystem => matches!(
                        component.1,
                        SystemComponentKind::TerminalEmulator
                            | SystemComponentKind::WindowManager
                            | SystemComponentKind::DesktopEnvironment
                    ),
                    SystemComponentKind::TerminalEmulator => {
                        component.1 == SystemComponentKind::CurrentShell
                    }
                    _ => false,
                })
                .map(|c| {
                    // these unwraps should be fine :3
                    let src_port = ports
                        .get_mut(idx)
                        .map(|ports| {
                            let port = ports.0;
                            ports.0 += 1;
                            port
                        })
                        .unwrap();
                    let dst_port = ports
                        .get_mut(&c.0)
                        .map(|ports| {
                            let port = ports.1;
                            ports.1 += 1;
                            port
                        })
                        .unwrap();
                    (*idx, src_port, c.0, dst_port)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let components: Vec<CollectedNode> = components
        .into_iter()
        .filter(|(_, _, i)| i.is_some())
        .map(|(id, kind, info)| {
            let title = kind.title();
            let body = info.unwrap();
            let width = (max(title.len(), body.len())) + 2;
            let ports = ports[&id];
            let height = max(ports.0, ports.1) + 2;

            CollectedNode {
                width: width as u16,
                height: height as u16,
                title,
                body,
            }
        })
        .collect();

    let links: Vec<Connection> = links
        .into_iter()
        .map(|(from_node, from_port, to_node, to_port)| {
            Connection::new(from_node, from_port, to_node, to_port)
        })
        .collect();

    Ok((components, links))
}
