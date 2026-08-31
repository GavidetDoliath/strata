// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;

use super::{find_package_dir, first_hash_token};

#[test]
fn first_hash_token_lowercases_and_ignores_trailing_filename() {
    assert_eq!(
        first_hash_token("ABCDEF  strata-0.2.0-x86_64-unknown-linux-gnu.tar.gz\n"),
        Some("abcdef".to_owned())
    );
}

#[test]
fn first_hash_token_rejects_empty_input() {
    assert_eq!(first_hash_token("   \n"), None);
}

#[test]
fn find_package_dir_locates_the_nested_executables() {
    let dir = std::env::temp_dir().join(format!(
        "strata-update-install-test-{}-{}",
        std::process::id(),
        line!()
    ));
    let package_dir = dir.join("strata-0.2.0-x86_64-unknown-linux-gnu");
    fs::create_dir_all(&package_dir).expect("create package dir");
    fs::write(package_dir.join("strata"), b"binary").expect("write binary");
    fs::write(package_dir.join("strata-preview-helper"), b"helper").expect("write helper");

    let found = find_package_dir(&dir).expect("binaries should be found");
    assert_eq!(found, package_dir);

    fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn find_package_dir_errors_when_missing() {
    let dir = std::env::temp_dir().join(format!(
        "strata-update-install-test-empty-{}-{}",
        std::process::id(),
        line!()
    ));
    fs::create_dir_all(&dir).expect("create empty dir");

    assert!(find_package_dir(&dir).is_err());

    fs::remove_dir_all(&dir).expect("cleanup");
}
