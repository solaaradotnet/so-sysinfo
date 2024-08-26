use anyhow::{Error, Result};
use clap::ValueEnum;
use so_logo_ascii_generator::generate;
use unicode_segmentation::UnicodeSegmentation;

static GRAFFITI_LOGO: &str = "
.▄▄ ·           .▄▄ ·  ▄· ▄▌.▄▄ · ▪   ▐ ▄ ·▄▄▄
▐█ ▀. ▪         ▐█ ▀. ▐█▪██▌▐█ ▀. ██ •█▌▐█▐▄▄·▪
▄▀▀▀█▄ ▄█▀▄     ▄▀▀▀█▄▐█▌▐█▪▄▀▀▀█▄▐█·▐█▐▐▌██▪  ▄█▀▄
▐█▄▪▐█▐█▌.▐▌    ▐█▄▪▐█ ▐█▀·.▐█▄▪▐█▐█▌██▐█▌██▌.▐█▌.▐▌
 ▀▀▀▀  ▀█▄▀▪     ▀▀▀▀   ▀ •  ▀▀▀▀ ▀▀▀▀▀ █▪▀▀▀  ▀█▄▀▪

";

pub(crate) fn get(which: LogoKind) -> Result<(String, usize, usize)> {
    let logo_text = match which {
        LogoKind::Shadow => generate(
            "so-sysinfo",
            true,
            so_logo_ascii_generator::TextFont::Shadow,
        )?,
        LogoKind::Graffiti => GRAFFITI_LOGO.to_owned(),
    };
    let logo_text_height = logo_text.lines().count();
    let logo_text_width = logo_text
        .lines()
        .map(|l| l.graphemes(true).count())
        .max()
        .ok_or(Error::msg("uhhh"))?;

    Ok((logo_text, logo_text_width, logo_text_height))
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogoKind {
    Shadow,
    Graffiti,
}

impl std::fmt::Display for LogoKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogoKind::Shadow => "Shadow",
                LogoKind::Graffiti => "Graffiti",
            }
        )
    }
}
