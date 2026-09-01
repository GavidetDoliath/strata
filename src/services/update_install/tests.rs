// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;

use super::{find_binaries, first_hash_token};

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
fn find_binaries_locates_a_single_nested_binary() {
    let dir = std::env::temp_dir().join(format!(
        "strata-update-install-test-{}-{}",
        std::process::id(),
        line!()
    ));
    let package_dir = dir.join("strata-0.2.0-x86_64-unknown-linux-gnu");
    fs::create_dir_all(&package_dir).expect("create package dir");
    fs::write(package_dir.join("strata"), b"binary").expect("write binary");

    let found = find_binaries(&dir, &["strata"]).expect("binary should be found");
    assert_eq!(found, vec![package_dir.join("strata")]);

    fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn find_binaries_errors_when_a_requested_name_is_missing() {
    let dir = std::env::temp_dir().join(format!(
        "strata-update-install-test-empty-{}-{}",
        std::process::id(),
        line!()
    ));
    fs::create_dir_all(&dir).expect("create empty dir");

    assert!(find_binaries(&dir, &["strata"]).is_err());

    fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn find_binaries_returns_all_requested_names_when_several_are_present() {
    let dir = std::env::temp_dir().join(format!(
        "strata-update-install-test-multi-{}-{}",
        std::process::id(),
        line!()
    ));
    let package_dir = dir.join("strata-0.2.0-x86_64-unknown-linux-gnu");
    fs::create_dir_all(&package_dir).expect("create package dir");
    fs::write(package_dir.join("strata"), b"binary").expect("write strata binary");
    fs::write(package_dir.join("strata-helper"), b"binary").expect("write helper binary");

    let found =
        find_binaries(&dir, &["strata", "strata-helper"]).expect("both binaries should be found");
    assert_eq!(
        found,
        vec![
            package_dir.join("strata"),
            package_dir.join("strata-helper"),
        ]
    );

    fs::remove_dir_all(&dir).expect("cleanup");
}
