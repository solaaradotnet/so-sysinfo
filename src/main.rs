use anyhow::Result;
use clap::{Parser, ValueEnum};
use collection::collect;
use logos::LogoKind;
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
use std::{io::stdout, panic::AssertUnwindSafe, time::Instant};
use tracing::{debug, trace};
use tui_nodes::{NodeGraph, NodeLayout};

mod args;
mod collection;
mod logos;

fn main() -> Result<()> {
    let args = args::Args::parse();
    debug!("Got args {args:?}");

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

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FgColor {
    SolaaraGold,
    LightMagenta,
}

impl From<FgColor> for Color {
    fn from(value: FgColor) -> Self {
        match value {
            FgColor::SolaaraGold => Color::Rgb(255, 241, 164),
            FgColor::LightMagenta => Color::LightMagenta,
        }
    }
}

struct AppState<'a> {
    current_logo: logos::LogoKind,
    logo_text: Text<'a>,
    logo_text_width: usize,
    logo_text_height: usize,
    fg_color: FgColor,
    pub needs_to_redraw: bool,
}

impl<'a> AppState<'a> {
    pub fn init(logo_kind: logos::LogoKind, fg_color: FgColor) -> Self {
        let (logo_text, logo_text_width, logo_text_height) = Self::generate_logo_data(logo_kind);
        let logo_text = ratatui::text::Text::from(logo_text).fg(Color::from(fg_color));

        Self {
            current_logo: logo_kind,
            logo_text,
            logo_text_width,
            logo_text_height,
            fg_color,
            needs_to_redraw: true,
        }
    }
    pub fn update_logo(&mut self, logo_kind: logos::LogoKind) {
        if self.current_logo == logo_kind {
            return;
        }

        let (logo_text, logo_text_width, logo_text_height) = Self::generate_logo_data(logo_kind);
        let logo_text = ratatui::text::Text::from(logo_text);
        self.logo_text = logo_text.fg(self.fg_color);
        self.logo_text_width = logo_text_width;
        self.logo_text_height = logo_text_height;
        self.current_logo = logo_kind;
    }

    pub fn update_fg_color(&mut self, new_color: FgColor) {
        self.fg_color = new_color;
        self.logo_text = self.logo_text.clone().fg(Color::from(self.fg_color));
    }

    fn generate_logo_data(logo_kind: logos::LogoKind) -> (String, usize, usize) {
        logo_kind.get_rendered()
    }

    fn cycle_next_logo(&mut self) {
        match self.current_logo {
            LogoKind::Shadow => self.update_logo(LogoKind::Graffiti),
            LogoKind::Graffiti => self.update_logo(LogoKind::Shadow),
        }
    }
    fn cycle_next_color(&mut self) {
        match self.fg_color {
            FgColor::SolaaraGold => self.update_fg_color(FgColor::LightMagenta),
            FgColor::LightMagenta => self.update_fg_color(FgColor::SolaaraGold),
        }
    }
}

impl<'a> From<args::Args> for AppState<'a> {
    fn from(value: args::Args) -> Self {
        Self::init(value.logo_kind, value.fg_color)
    }
}

fn app<T: Backend>(mut terminal: ratatui::Terminal<T>, args: args::Args) -> Result<()> {
    let now = Instant::now();
    let (nodes, links) = collect(args.visual_toggles)?;
    let elapsed = now.elapsed().as_millis();

    let bottom_text = format!(" took {elapsed}ms ");

    let mut app_state = AppState::from(args);

    let hostname = collection::utils::get_hostname()?;

    loop {
        let frame_start = Instant::now();
        if app_state.needs_to_redraw {
            let graph_nodes = nodes
                .iter()
                .map(|node| NodeLayout::new((node.width, node.height)).with_title(node.title))
                .collect();

            trace!("copied graph nodes {:?}", frame_start.elapsed());

            terminal.draw(|frame| {
                trace!(
                    "------------------------ frame draw started {:?}",
                    frame_start.elapsed()
                );
                let area = frame.area();

                let [header_area, body_area] = Layout::vertical([
                    Constraint::Length(app_state.logo_text_height as u16),
                    Constraint::Fill(1),
                ])
                .areas(area);

                let [logo_area] =
                    Layout::horizontal([Constraint::Length(app_state.logo_text_width as u16)])
                        .flex(ratatui::layout::Flex::Center)
                        .areas(header_area);

                frame.render_widget(Clear, logo_area);
                frame.render_widget(&app_state.logo_text, logo_area);
                trace!("logo drawn {:?}", frame_start.elapsed());

                let window_widget = Block::new()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::new().fg(Color::from(app_state.fg_color)))
                    .borders(Borders::ALL)
                    .title(
                        Title::from(" ".to_owned() + hostname.as_ref() + " ")
                            .alignment(ratatui::layout::Alignment::Center),
                    )
                    .title_bottom(
                        Line::from(bottom_text.clone())
                            .right_aligned()
                            .italic()
                            .dim(),
                    );
                frame.render_widget(&window_widget, body_area);

                trace!("window frame drawn {:?}", frame_start.elapsed());

                let body_area = window_widget.inner(body_area);

                let mut system_info_nodes_graph = NodeGraph::new(
                    graph_nodes,
                    links.to_vec(),
                    body_area.width.into(),
                    body_area.height.into(),
                );

                trace!("node graph created {:?}", frame_start.elapsed());

                // TODO: do this better... please...
                // horrid panic suppression code...
                std::panic::set_hook(Box::new(|_| {}));
                trace!("noop panic hook set {:?}", frame_start.elapsed());
                let test = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    trace!("node graph calculate(start) {:?}", frame_start.elapsed());
                    system_info_nodes_graph.calculate();
                    trace!("node graph calculate(done) {:?}", frame_start.elapsed());
                }));
                trace!("catch_unwind() {:?}", frame_start.elapsed());
                let _ = std::panic::take_hook();
                trace!("panic hook restored {:?}", frame_start.elapsed());
                // horridness over!

                if test.is_err() {
                    trace!("window too small {:?}", frame_start.elapsed());
                    frame.render_widget(
                        Paragraph::new("Window too small. Resize it to show system graph.")
                            .red()
                            .centered(),
                        body_area,
                    );
                } else {
                    trace!("window good {:?}", frame_start.elapsed());
                    let zones = system_info_nodes_graph.split(body_area);
                    trace!("zones obtained {:?}", frame_start.elapsed());
                    for (idx, ea_zone) in zones.into_iter().enumerate() {
                        frame.render_widget(
                            Paragraph::new(nodes[idx].body.clone()).centered(),
                            ea_zone,
                        );
                        trace!("zone {idx} drawn {:?}", frame_start.elapsed());
                    }
                    frame.render_stateful_widget(system_info_nodes_graph, body_area, &mut ());
                    trace!("node graph widget drawn {:?}", frame_start.elapsed());
                }

                app_state.needs_to_redraw = false;
            })?;

            trace!(
                "------------------------ frame draw over {:?}",
                frame_start.elapsed()
            );
        }
        if event::poll(std::time::Duration::from_millis(16))? {
            trace!("polled for event {:?}", frame_start.elapsed());
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') => app_state.cycle_next_color(),
                        KeyCode::Char('l') => app_state.cycle_next_logo(),
                        _ => {}
                    }
                }
            }
            app_state.needs_to_redraw = true;
        }
    }

    Ok(())
}
