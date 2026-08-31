// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn gpu_devices(dev: &Path) -> Vec<PathBuf> {
    let mut devices = Vec::new();
    if let Ok(entries) = fs::read_dir(dev.join("dri")) {
        for entry in entries.flatten() {
            if numbered_name(&entry.file_name(), "renderD") {
                devices.push(entry.path());
            }
        }
    }
    if let Ok(entries) = fs::read_dir(dev) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            if name == "nvidiactl" || numbered_name(&name, "nvidia") {
                devices.push(entry.path());
            }
        }
    }
    devices.sort();
    devices
}

pub(crate) fn numbered_name(name: &std::ffi::OsStr, prefix: &str) -> bool {
    name.to_str()
        .and_then(|name| name.strip_prefix(prefix))
        .is_some_and(|suffix| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
}
