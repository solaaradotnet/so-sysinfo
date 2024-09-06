use anyhow::Result;
use std::{cmp::max, collections::HashMap};
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
    // accumulator to generate component ids
    let mut component_id_acc: usize = 0;

    // gonna allocate for all possible components so this shouldnt need any resizing
    let mut ports: HashMap<usize, (usize, usize)> =
        HashMap::with_capacity(SystemComponentKind::iter().len());

    let components: Vec<_> = SystemComponentKind::iter()
        // collect info
        .map(|k| (k, k.collect_info(&visual_toggles)))
        .flat_map(|(kind, component_info_outer)| match component_info_outer {
            // if component is disabled or cant be displayed
            Err(_) => vec![(0, kind, None)],
            // otherwise
            Ok(component_info) => component_info
                .into_iter()
                .map(|info_string| {
                    // get an id
                    let component_id = component_id_acc;
                    component_id_acc += 1;
                    // prepare ports entry
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
                // we get the linked component(s)
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
                .map(|dst_component| {
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
                        .get_mut(&dst_component.0)
                        .map(|ports| {
                            let port = ports.1;
                            ports.1 += 1;
                            port
                        })
                        .unwrap();
                    (*idx, src_port, dst_component.0, dst_port)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let components: Vec<CollectedNode> = components
        .into_iter()
        // TODO: properly handle missing components
        .filter(|(_, _, i)| i.is_some())
        .map(|(id, kind, info)| {
            let title = kind.title();
            let body = info.unwrap();
            let ports = ports[&id];
            // 2 is the box's borders, we make sure we can fit either the title or body (or both)
            let width = (max(title.len(), body.len())) + 2;
            // same thing with either of the box's sides
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
