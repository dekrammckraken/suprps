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

use std::{env::home_dir, fs};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    dev_block: String,
    launcher: String,
    lounge: Option<String>,
    mac: String,
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Serialization(toml::de::Error),
    InvalidHome
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) ->Self {
        Error::IO(e)
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Serialization(e)
    }
}
impl Config {
    pub fn from_file(cfg_file: &str) -> Result<Config, Error> {
       
        let mut home_path = home_dir()
            .ok_or(Error::InvalidHome)?;
        
        home_path.push(&cfg_file);

        let path = fs::read_to_string(&home_path)?;
        let conf = toml::from_str(&path)?;

        Ok(conf)
    }

    pub fn get_device(&self) -> &str {
        &self.dev_block
    }
    pub fn get_launcher(&self) -> &str {
        &self.launcher
    }
    pub fn get_mac(&self) -> &str {
        &self.mac
    }
     pub fn get_lounge(&self) -> Option<&str> {
        self.lounge.as_deref()
    }
    pub fn validate(&self) -> bool {
        
        if self.dev_block.is_empty(){
            return false;
        }

        if self.mac.is_empty() {
            return false;
        }

        if self.launcher.is_empty() {
            return false;
        }
        true
    }
}
