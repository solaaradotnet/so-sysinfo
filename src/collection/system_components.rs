use anyhow::Error;
use libmacchina::{traits::GeneralReadout as _, traits::MemoryReadout as _};
use crate::args::VisualToggles;

lazy_static::lazy_static! {
    static ref LIBMACCHINA_GENERAL_READOUT: libmacchina::GeneralReadout = libmacchina::GeneralReadout::new();
}

pub(crate) trait SystemComponent {
    fn collect_info(_vt: &VisualToggles) -> Result<Vec<String>, Error>;
}

pub(crate) struct Hostname;

impl SystemComponent for Hostname {
    fn collect_info(_vt: &VisualToggles) -> Result<Vec<String>, Error> {
        Ok(vec![LIBMACCHINA_GENERAL_READOUT
            .hostname()
            .map_err(|_| Error::msg("Failed to get hostname."))?])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testresult::TestResult;
    use tracing_test::traced_test;

    #[test]
    fn test_get_hostname() -> TestResult {
        let _ = get_hostname()?;
        Ok(())
    }
}
