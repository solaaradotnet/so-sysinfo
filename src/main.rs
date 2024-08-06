use anyhow::{Error, Result};
use indexmap::IndexMap;
use libmacchina::{traits::GeneralReadout as _, GeneralReadout};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{block::Title, Block, Borders, Clear, Paragraph},
    Terminal,
};
use so_logo_ascii_generator_core::generate;
use std::{cmp::max, io::stdout, str::FromStr};
use tui_nodes::NodeGraph;

mod utils;
use utils::{
    get_cpu, get_de, get_model, get_os, get_shell, get_system_memory, get_terminal, get_wm,
};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let res = app(terminal);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    if let Err(error) = res {
        println!("{error}");
    }

    Ok(())
}

fn app<T: Backend>(mut terminal: ratatui::Terminal<T>) -> Result<()> {
    let fg_color = Color::from_str("#FFF1A4")?;

    let system_info = sysinfo::System::new_all();
    let os_info = os_info::get();

    let readout = GeneralReadout::new();

    let hostname = readout
        .hostname()
        .map_err(|_| Error::msg("Failed to get hostname."))?;

    let logo_text = generate("SOLAARA", true)?;
    let logo_text_height = logo_text.lines().count();
    let logo_text_width = logo_text
        .lines()
        .map(|l| l.len())
        .max()
        .ok_or(Error::msg("uhhh"))?;
    let logo_text = Text::from(logo_text).fg(fg_color);

    // TODO: rework this into something more resilient that can handle optional nodes
    let graph_contents: IndexMap<&str, (String, u16)> = [
        ("[ CPU ]", (get_cpu(&readout)?, 1)),              // IDX 0: CPU
        ("[ RAM ]", (get_system_memory(&system_info), 1)), // IDX 1: RAM
        ("[ Model ]", (get_model(&readout), 2)),           // IDX 3: System
        ("[ OS ]", (get_os(&readout, &os_info)?, 2)),      // IDX 3: OS
        ("[ Shell ]", (get_shell(&system_info)?, 1)),      // IDX 4: Shell
        ("[ Terminal ]", (get_terminal(&readout)?, 1)),    // IDX 5: Terminal
        ("[ DE ]", (get_de(&readout)?, 1)),                // IDX 6: DE
        ("[ WM ]", (get_wm(&readout)?, 1)),                // IDX 7: WM
    ]
    .into();

    let graph_links: Vec<(usize, usize, usize, usize)> = vec![
        (0, 0, 2, 0), // CPU -> Model
        (1, 0, 2, 1), // RAM -> Model
        (2, 0, 3, 0), // Model -> OS
        (3, 1, 7, 0), // OS -> WM
        (3, 1, 6, 0), // OS -> DE
        (3, 0, 5, 0), // OS -> Terminal
        (5, 0, 4, 0), // Terminal -> Shell
    ];

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            let main_layout = Layout::vertical([
                Constraint::Length(logo_text_height as u16),
                Constraint::Fill(1),
            ])
            .split(area);

            let logo_text_layout = Layout::horizontal([Constraint::Length(logo_text_width as u16)])
                .flex(ratatui::layout::Flex::Center)
                .split(main_layout[0]);

            frame.render_widget(Clear, logo_text_layout[0]);
            frame.render_widget(&logo_text, logo_text_layout[0]);

            let window_widget = Block::new()
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::new().fg(fg_color))
                .borders(Borders::ALL)
                .title(
                    Title::from(" ".to_owned() + hostname.as_ref() + " ")
                        .alignment(ratatui::layout::Alignment::Center),
                )
                .title_bottom(Line::from(" so-sysinfo ").right_aligned());
            frame.render_widget(&window_widget, main_layout[1]);
            let window_inner_area = window_widget.inner(main_layout[1]);

            if area.height < 27 {
                frame.render_widget(
                    Paragraph::new(
                        "Window too small. Resize to have at least 27 lines to show system graph.",
                    )
                    .red()
                    .centered(),
                    window_inner_area,
                );
            } else {
                let mut system_info_nodes_graph = NodeGraph::new(
                    graph_contents
                        .iter()
                        .map(|(title, (contents, height))| {
                            tui_nodes::NodeLayout::new((
                                (max(contents.len(), title.len()) + 2) as u16,
                                2 + height,
                            ))
                            .with_title(title)
                        })
                        .collect::<Vec<_>>(),
                    graph_links
                        .iter()
                        .map(|(from_node, from_port, to_node, to_port)| {
                            tui_nodes::Connection::new(*from_node, *from_port, *to_node, *to_port)
                        })
                        .collect::<Vec<_>>(),
                    window_inner_area.width.into(),
                    window_inner_area.height.into(),
                );
                system_info_nodes_graph.calculate();
                let zones = system_info_nodes_graph.split(window_inner_area);
                for (idx, ea_zone) in zones.into_iter().enumerate() {
                    frame.render_widget(
                        Paragraph::new(graph_contents[idx].0.to_string()).centered(),
                        ea_zone,
                    );
                }
                frame.render_stateful_widget(system_info_nodes_graph, window_inner_area, &mut ());
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    Ok(())
}
