use clap::Parser;

use crate::logos::LogoKind;

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Args {
    #[arg(short, long = "logo", default_value = "shadow")]
    pub logo_kind: LogoKind,
    #[arg(short = 'c', long = "fg-color", default_value = "solaara-gold")]
    pub fg_color: crate::FgColor,

    #[command(flatten, next_help_heading = "Visual Toggles")]
    pub visual_toggles: VisualToggles,
}

#[derive(Clone, Debug, Copy, clap::Args)]
pub(crate) struct VisualToggles {
    #[arg(long)]
    pub hide_terminal_version: bool,
}
