// SPDX-License-Identifier: GPL-3.0-or-later

use super::{BuildKind, Channel, Version};

fn parse(tag: &str) -> Version {
    Version::parse(tag).unwrap_or_else(|| panic!("expected {tag} to parse"))
}

#[test]
fn rc_ordinal_orders_within_same_core() {
    assert!(parse("0.5.0-rc.2") > parse("0.5.0-rc.1"));
}

#[test]
fn final_release_outranks_its_own_prerelease() {
    assert!(parse("0.5.0") > parse("0.5.0-rc.2"));
}

#[test]
fn patch_bump_outranks_prior_release() {
    assert!(parse("0.5.0") > parse("0.4.9"));
}

#[test]
fn nightly_ordinal_orders_by_date_within_same_core() {
    assert!(parse("0.5.0-nightly.20260901") > parse("0.5.0-nightly.20260831"));
}

#[test]
fn nightly_same_date_orders_by_suffix() {
    assert!(parse("0.5.0-nightly.20260901.2") > parse("0.5.0-nightly.20260901.1"));
    assert!(parse("0.5.0-nightly.20260901.1") > parse("0.5.0-nightly.20260901"));
}

#[test]
fn equal_versions_compare_equal() {
    assert_eq!(parse("0.5.0-rc.1"), parse("0.5.0-rc.1"));
    assert_eq!(parse("0.5.0"), parse("0.5.0"));
    assert_eq!(
        parse("0.5.0-nightly.20260901.2"),
        parse("0.5.0-nightly.20260901.2")
    );
}

#[test]
fn nightly_outranks_rc_for_the_same_core() {
    // Pinned convention: within an equal core, a nightly build compares
    // greater than a release candidate. This is an arbitrary but explicit
    // choice (see BuildKind's declaration order) -- if it ever changes,
    // this test must change with it.
    assert!(parse("0.5.0-nightly.20260901") > parse("0.5.0-rc.1"));
}

#[test]
fn accepts_canonical_forms() {
    assert!(Version::parse("v0.5.0").is_some());
    assert!(Version::parse("0.5.0").is_some());
    assert!(Version::parse("v0.5.0-rc.1").is_some());
    assert!(Version::parse("v0.5.0-nightly.20260901").is_some());
    assert!(Version::parse("v0.5.0-nightly.20260901.2").is_some());
}

#[test]
fn rejects_malformed_tags() {
    assert!(Version::parse("0.5").is_none());
    assert!(Version::parse("0.5.x").is_none());
    assert!(Version::parse("v0.5.0-beta.1").is_none());
    assert!(Version::parse("v0.5.0-rc").is_none());
    assert!(Version::parse("v0.5.0-rc.x").is_none());
    assert!(Version::parse("").is_none());
    assert!(Version::parse("vv0.5.0").is_none());
    assert!(Version::parse("0.5.0-rc.1-extra").is_none());
}

#[test]
fn display_renders_canonical_rc_tag() {
    assert_eq!(parse("v0.5.0-rc.1").to_string(), "0.5.0-rc.1");
}

#[test]
fn display_renders_canonical_final_tag() {
    assert_eq!(parse("v0.5.0").to_string(), "0.5.0");
}

#[test]
fn display_renders_canonical_nightly_tag() {
    assert_eq!(
        parse("v0.5.0-nightly.20260901.2").to_string(),
        "0.5.0-nightly.20260901.2"
    );
    assert_eq!(
        parse("v0.5.0-nightly.20260901").to_string(),
        "0.5.0-nightly.20260901"
    );
}

#[test]
fn channel_round_trips() {
    assert_eq!(Channel::parse("stable"), Channel::Stable);
    assert_eq!(Channel::parse("preview"), Channel::Preview);
    assert_eq!(Channel::parse("nightly"), Channel::Stable);
    assert_eq!(Channel::parse(""), Channel::Stable);
}

#[test]
fn channel_as_str_matches_persisted_values() {
    assert_eq!(Channel::Stable.as_str(), "stable");
    assert_eq!(Channel::Preview.as_str(), "preview");
}

#[test]
fn build_kind_labels_are_ui_facing() {
    assert_eq!(BuildKind::Stable.label(), "Stable");
    assert_eq!(BuildKind::Rc.label(), "Release candidate");
    assert_eq!(BuildKind::Nightly.label(), "Nightly");
}
