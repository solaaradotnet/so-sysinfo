use anyhow::Result;
use clap::Parser;
use collection::collect;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{block::Title, Block, Borders, Clear, Paragraph},
    Terminal,
};
use std::{io::stdout, rc::Rc, str::FromStr, time::Instant};
use tui_nodes::{NodeGraph, NodeLayout};

mod collection;
mod logos;
mod args;

fn main() -> Result<()> {
    let args = args::Args::parse();
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let res = app(terminal, args);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    res?;

    Ok(())
}

fn app<T: Backend>(mut terminal: ratatui::Terminal<T>, args: args::Args) -> Result<()> {
    let mut args = args;

    let fg_color = Color::from_str("#FFF1A4")?;

    let hostname = collection::utils::get_hostname()?;

    let (logo_text, mut logo_text_width, mut logo_text_height) = logos::get(args.logo_kind)?;
    let mut logo_text = logo_text.fg(fg_color);

    let now = Instant::now();
    let (nodes, links) = collect()?;
    let elapsed = now.elapsed().as_millis();

    let bottom_text = format!(" took {elapsed}ms ");

    let nodes = Rc::new(nodes);
    let links = Rc::new(links);

    loop {
        let graph_nodes = nodes
            .iter()
            .map(|node| NodeLayout::new((node.width, node.height)).with_title(node.title))
            .collect();

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
                .title_bottom(Line::from(bottom_text.clone()).right_aligned().italic().dim());
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
                    graph_nodes,
                    links.to_vec(),
                    window_inner_area.width.into(),
                    window_inner_area.height.into(),
                );
                system_info_nodes_graph.calculate();
                let zones = system_info_nodes_graph.split(window_inner_area);
                for (idx, ea_zone) in zones.into_iter().enumerate() {
                    frame
                        .render_widget(Paragraph::new(nodes[idx].body.clone()).centered(), ea_zone);
                }
                frame.render_stateful_widget(system_info_nodes_graph, window_inner_area, &mut ());
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('c') {
                    let new_kind = match args.logo_kind {
                        logos::LogoKind::Shadow => logos::LogoKind::Graffiti,
                        logos::LogoKind::Graffiti => logos::LogoKind::Shadow,
                    };

                    args.logo_kind = new_kind;
                    (logo_text, logo_text_width, logo_text_height) = logos::get(args.logo_kind)?;
                    logo_text = logo_text.fg(fg_color);
                }
            }
        }
    }

    Ok(())
}
