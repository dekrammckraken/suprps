// This file is part of suprps
//
// suprps is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// suprps is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use log::{LevelFilter, error, info};
use std::{io};
use systemd_journal_logger::JournalLog;

pub mod config;
pub mod padmon;

use config::Config;
use padmon::PadMon;
const CFG_FILE: &str = ".config/suprps/config";


#[tokio::main]
async fn main() -> io::Result<()> {
    
    JournalLog::new()?.install().expect("Failed log");

    
    log::set_max_level(LevelFilter::Info);
    info!( "Starting suprps");

    let cfg = match Config::from_file(CFG_FILE) {
        Ok(cfg) =>cfg,
        Err(_e) => {
            error!("Something in the config is wrong. Perhaps you should check it.");
            std::process::exit(1)
        }
    };

    if !cfg.validate() {
        error!("Some values in the config are wrong. Perhaps you should fill them in.");
        std::process::exit(1)
    }
    info!("Configuration file validated and ready to go! suprps is monitoring device {}-{} and {} will start in lounge mode {}.", 
        cfg.get_device(), cfg.get_mac(), cfg.get_launcher(),cfg.get_lounge().get_or_insert("NO")
    );

    PadMon::new().begin_monitor(&cfg).await?;
    
    Ok(())

}
