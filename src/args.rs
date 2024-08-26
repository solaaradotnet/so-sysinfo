use clap::Parser;

use crate::logos::LogoKind;

#[derive(Parser)]
#[command(version)]
pub(crate) struct Args {
    #[arg(short, long = "logo", default_value = "shadow")]
    pub logo_kind: LogoKind,
    #[arg(short = 'c', long = "fg-color", default_value = "solaara-gold")]
    pub fg_color: crate::FgColor,
}
