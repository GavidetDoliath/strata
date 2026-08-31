// SPDX-License-Identifier: GPL-3.0-or-later

#[path = "../sandbox_devices.rs"]
mod sandbox_devices;
#[path = "../sandbox_helper.rs"]
mod sandbox_helper;

fn main() -> std::process::ExitCode {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    if let Err(error) = sandbox_helper::run(&arguments) {
        eprintln!("Preview helper failed: {error}");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
