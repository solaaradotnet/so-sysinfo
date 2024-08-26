use clap::Parser;

use crate::logos::LogoKind;

#[derive(Parser)]
#[command(version)]
pub(crate) struct Args {
    #[arg(short, long = "logo", default_value = "shadow")]
    pub logo_kind: LogoKind,
}
