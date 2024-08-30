use chrono::Local;
use clap::ValueEnum;

fn make_graffiti_logo() -> (String, usize, usize) {
    let cr_year = Local::now().format("%Y");
    (
        format!(
            ".▄▄ ·           .▄▄ ·  ▄· ▄▌.▄▄ · ▪   ▐ ▄ ·▄▄▄
▐█ ▀. ▪         ▐█ ▀. ▐█▪██▌▐█ ▀. ██ •█▌▐█▐▄▄·▪
▄▀▀▀█▄ ▄█▀▄     ▄▀▀▀█▄▐█▌▐█▪▄▀▀▀█▄▐█·▐█▐▐▌██▪  ▄█▀▄
▐█▄▪▐█▐█▌.▐▌    ▐█▄▪▐█ ▐█▀·.▐█▄▪▐█▐█▌██▐█▌██▌.▐█▌.▐▌
 ▀▀▀▀  ▀█▄▀▪     ▀▀▀▀   ▀ •  ▀▀▀▀ ▀▀▀▀▀ █▪▀▀▀  ▀█▄▀▪
                          (C) Solaara's Network {cr_year:0>4}"
        ),
        53,
        6,
    )
}
fn make_shadow_logo() -> (String, usize, usize) {
    let cr_year = Local::now().format("%Y");
    (
        format!(
            "                 =@-
     =%.         *@:          .
     .%@=        %@.        .=@#
       =@%.    ..-=.      .*@@:
        .#=.-@@%##%@@=. .#@%:
         .*@=.       =@#.=.
        .@*            +@:
==--:.. *#.             ##
++*#%@* @=              .@.
        +@.       
         +@-                                              _)          _|        
      .+%::#@+.      __|   _ \\           __|  |   |   __|  |  __ \\   |     _ \\  
    .*@@-   .:+%@. \\__ \\  (   | _____| \\__ \\  |   | \\__ \\  |  |   |  __|  (   | 
   .*%:        ##. ____/ \\___/         ____/ \\__, | ____/ _| _|  _| _|   \\___/  
              .@@.                           ____/                              
              :@%                                     (C) Solaara's Network {cr_year:0>4} "
        ),
        81,
        16,
    )
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LogoKind {
    Shadow,
    Graffiti,
}

impl LogoKind {
    pub fn get_rendered(&self) -> (String, usize, usize) {
        match self {
            LogoKind::Shadow => make_shadow_logo(),
            LogoKind::Graffiti => make_graffiti_logo(),
        }
    }
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
