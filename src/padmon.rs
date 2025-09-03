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

use log::{error, info};
use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
use std::{io, os::fd::AsFd, process::Command};
use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::config::Config;

pub struct PadMon {
    pub is_monitoring: bool,
}

impl PadMon {
    pub fn new() -> Self {
        Self {
            is_monitoring: false,
        }
    }
    pub fn is_begin_monitor(&self) -> bool {
        self.is_monitoring
    }
    pub fn disconnect_bluetooth_device(&self, cfg: &Config) -> io::Result<()> {
        let status = Command::new("bluetoothctl")
            .arg("disconnect")
            .arg(cfg.get_mac())
            .status()?;

        if status.success() {
            info!("{} device disconnected!", cfg.get_mac());
            Ok(())
        } else {
            error!("{} error while disconnecting device...", cfg.get_mac());
            Err(io::Error::other("Disconnect failed"))
        }
    }
    pub async fn dbus_bluetooth_disconnect_device(&self, cfg: &Config) -> zbus::Result<()> {
        let conn = Connection::system().await?;
        let mac = cfg.get_mac().replace(":", "_");
        let path = format!("/org/bluez/hci0/dev_{}", mac);

        let proxy = zbus::Proxy::new(
            &conn,
            "org.bluez",
            OwnedObjectPath::try_from(path)?,
            "org.bluez.Device1",
        )
        .await?;

        proxy.call_method("Disconnect", &()).await?;
        Ok(())
    }
    pub async fn ensure_launcher_running(&self, cfg: &Config) {
        let running = Command::new("pgrep")
            .arg(&cfg.get_launcher())
            .output()
            .map(|output| !output.stdout.is_empty())
            .unwrap_or(false);

        if !running {
            let mut launcher_commmand = Command::new(&cfg.get_launcher());

            if let Some(lounge_param) = &cfg.get_lounge() {
                launcher_commmand.arg(lounge_param);
            }

            match launcher_commmand.spawn() {
                Ok(mut child) => {
                    if let Ok(status) = child.wait() {
                        info!("{} exited with status {}", cfg.get_launcher(), status);

                        if status.success() {
                            info!("{} waited and stopped.", cfg.get_launcher());
                        } else {
                            error!("{} cannot wait for launcher.", cfg.get_launcher())
                        }

                        match self.dbus_bluetooth_disconnect_device(cfg).await {
                            Ok(_) => info!("{} disconnected.", cfg.get_device()),
                            Err(e) => error!("{} failed to disconnect: {}", cfg.get_device(), e),
                        }
                    } else {
                        error!("Failed to wait for {} process!", cfg.get_launcher());
                    }
                }
                Err(e) => {
                    error!("Failed to spawn {}: {}", cfg.get_launcher(), e);
                }
            }
        }
    }
    pub async fn begin_monitor(&mut self, cfg: &Config) -> io::Result<()> {
        let monitor = udev::MonitorBuilder::new()?
            .match_subsystem("input")?
            .listen()?;

        self.is_monitoring = true;

        let fd = monitor.as_fd();
        let mut fds = [PollFd::new(fd, PollFlags::POLLIN)];
        let mut iter = monitor.iter();

        loop {
            poll(&mut fds, PollTimeout::NONE)?;
            if let Some(event) = iter.next() {
                if let Some(action) = event.action() {
                    let dev = event.device();
                    let node = dev.devnode().map(|d| d.to_string_lossy());

                    if action == "add" {
                        if let Some(node) = node {
                            if node.as_ref() == cfg.get_device() {
                                info!(
                                    "{} We detected you pressed a button on your controller, didn’t you? Sorry, someone is playing instead of you.",
                                    cfg.get_device()
                                );
                                self.ensure_launcher_running(cfg).await;
                            }
                        }
                    } else if action == "remove" {
                        if let Some(node) = node {
                            if node.as_ref() == cfg.get_device() {
                                info!("{} Time to do some other stuff.", cfg.get_device());
                            }
                        }
                    }
                }
            }
            info!("suprps has shut down. What a shame.");
        }
    }
}
