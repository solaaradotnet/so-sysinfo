use anyhow::{Error, Result};
use ratatui::text::Text;
use so_logo_ascii_generator::generate;
use unicode_segmentation::UnicodeSegmentation;

static GRAFFITI_LOGO: &str = 
"
.▄▄ ·           .▄▄ ·  ▄· ▄▌.▄▄ · ▪   ▐ ▄ ·▄▄▄
▐█ ▀. ▪         ▐█ ▀. ▐█▪██▌▐█ ▀. ██ •█▌▐█▐▄▄·▪
▄▀▀▀█▄ ▄█▀▄     ▄▀▀▀█▄▐█▌▐█▪▄▀▀▀█▄▐█·▐█▐▐▌██▪  ▄█▀▄
▐█▄▪▐█▐█▌.▐▌    ▐█▄▪▐█ ▐█▀·.▐█▄▪▐█▐█▌██▐█▌██▌.▐█▌.▐▌
 ▀▀▀▀  ▀█▄▀▪     ▀▀▀▀   ▀ •  ▀▀▀▀ ▀▀▀▀▀ █▪▀▀▀  ▀█▄▀▪

";

pub(crate) fn get(which: LogoKind) -> Result<(Text<'static>, usize, usize)> {
    let logo_text = match which {
        LogoKind::Shadow => generate("so-sysinfo", true, so_logo_ascii_generator::TextFont::Shadow)?,
        LogoKind::Graffiti => GRAFFITI_LOGO.to_owned(),
    };
    let logo_text_height = logo_text.lines().count();
    let logo_text_width = logo_text
        .lines()
        .map(|l| l.graphemes(true).count())
        .max()
        .ok_or(Error::msg("uhhh"))?;
    let logo_text = Text::from(logo_text);

    Ok(
        (
            logo_text,
            logo_text_width,
            logo_text_height        )
    )
}

#[test]
fn test_get_graffiti() {
    let logo = get(LogoKind::Graffiti).unwrap();

    assert_eq!(logo.1, 52);
    assert_eq!(logo.2, 5);
}

pub(crate) enum LogoKind {
    Shadow,
    Graffiti
}

