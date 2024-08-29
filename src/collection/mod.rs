use anyhow::{Error, Result};
use std::{cmp::max, collections::HashMap, fmt::Display};
use strum::IntoEnumIterator;
use tui_nodes::Connection;

pub(crate) mod utils;
use utils::{
    get_cpu, get_de, get_model, get_os, get_shell, get_system_memory, get_terminal, get_wm,
};

use crate::args::VisualToggles;

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
    pub fn collect_info(&self, visual_toggles: &VisualToggles) -> Result<Vec<String>> {
        match self {
            SystemComponentKind::Cpu => Ok(vec![get_cpu()?]),
            SystemComponentKind::SystemMemory => Ok(vec![get_system_memory()]),
            SystemComponentKind::BoardModel => Ok(vec![get_model()]),
            SystemComponentKind::CurrentShell => Ok(vec![get_shell()?]),
            SystemComponentKind::TerminalEmulator => Ok(vec![get_terminal(visual_toggles)?]),
            SystemComponentKind::DesktopEnvironment => Ok(vec![get_de()?]),
            SystemComponentKind::WindowManager => Ok(vec![get_wm()?]),
            SystemComponentKind::Gpu => Err(Error::msg("N/A")),
            SystemComponentKind::OperatingSystem => Ok(vec![get_os()?]),
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
    let partial_nodes: Vec<_> = SystemComponentKind::iter()
        .filter_map(|kind| {
            if let Ok(info) = kind.collect_info(&visual_toggles) {
                return Some(info.into_iter().map(move |info| (kind, info)));
            };
            None
        })
        .flatten()
        .enumerate()
        .map(|(idx, (kind, ele))| (idx, kind, ele))
        .map(|(idx, kind, body)| (idx, kind, format!(" {body} ")))
        .collect();

    let mut source_ports: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut dest_ports: HashMap<usize, Vec<usize>> = HashMap::new();

    partial_nodes.iter().for_each(|(idx, kind, _)| {
        let dest_ports = dest_ports.entry(*idx).or_default();
        for dst in partial_nodes.iter() {
            if dst.1.get_links().contains(kind) {
                dest_ports.push(dst.0);
                source_ports.entry(dst.0).or_default().push(*idx);
            }
        }
    });

    let source_ports: HashMap<_, _> = source_ports
        .into_iter()
        .map(|(k, v)| {
            let stuff: Vec<_> = v.into_iter().enumerate().collect();
            (k, stuff)
        })
        .collect();

    let dest_ports: HashMap<_, _> = dest_ports
        .into_iter()
        .map(|(k, v)| {
            let stuff: Vec<_> = v.into_iter().enumerate().collect();
            (k, stuff)
        })
        .collect();

    let links = dest_ports
        .iter()
        .flat_map(|(dst_idx, links_to_me)| {
            // (from_node, from_port, to_node, to_port)
            let test: Vec<_> = links_to_me
                .iter()
                .map(|(dst_port, src_idx)| {
                    let src_ports = &source_ports[&src_idx];
                    let me = src_ports
                        .iter()
                        .find(|src_port| src_port.1 == *dst_idx)
                        .unwrap();
                    let src_port = me.0;

                    (src_idx, src_port, dst_idx, dst_port)
                })
                .map(|(src_idx, src_port, dst_idx, dst_port)| {
                    Connection::new(*src_idx, src_port, *dst_idx, *dst_port)
                })
                .collect();

            test
        })
        .collect();

    let nodes = partial_nodes
        .into_iter()
        .map(|(index, kind, body)| {
            let title = kind.title();

            let width = (max(body.len(), title.len()) + 2) as u16;

            let left_side_height = dest_ports.get(&index).map(|c| c.len()).unwrap_or(0);
            let right_side_height = source_ports.get(&index).map(|c| c.len()).unwrap_or(0);

            let height = 2 + max(left_side_height, right_side_height) as u16;

            CollectedNode {
                width,
                height,
                title,
                body,
            }
        })
        .collect();

    Ok((nodes, links))
}
