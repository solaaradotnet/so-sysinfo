use clap::Parser;
use ratatui::style::Color;

use crate::logos::LogoKind;

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Args {
    #[arg(short, long = "logo", default_value = "shadow")]
    pub logo_kind: LogoKind,
    #[arg(short = 'c', long = "fg-color", default_value = "solaara-gold")]
    pub fg_color: FgColor,

    #[command(flatten, next_help_heading = "Visual Toggles")]
    pub visual_toggles: VisualToggles,
}

#[derive(Clone, Debug, Copy, clap::Args)]
pub(crate) struct VisualToggles {
    #[arg(long)]
    pub hide_terminal_version: bool,
}

#[derive(Debug, clap::ValueEnum, Clone, Copy, PartialEq, Eq)]
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
