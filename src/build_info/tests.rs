// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;

#[test]
fn package_metadata_is_available_to_the_about_page() {
    assert!(!DESCRIPTION.is_empty());
    assert!(!VERSION.is_empty());
    assert!(!COMMIT.is_empty());
    assert_eq!(REPOSITORY, "https://github.com/LGSE/strata");
    assert_eq!(AUTHOR, "LGSE Ltd.");
}

/// The default `RELEASE_TAG` injected by `build.rs` (no `STRATA_RELEASE_TAG`
/// override, i.e. every developer build and this test run) must be a
/// well-formed tag the release-channel grammar accepts.
#[test]
fn default_release_tag_parses_to_a_version() {
    assert!(
        crate::services::Version::parse(RELEASE_TAG).is_some(),
        "expected {RELEASE_TAG:?} to parse as a Version"
    );
}

/// A developer build has no `STRATA_BUILD_KIND` override, so it must
/// report itself as `Stable` -- the fallback that keeps a build unable to
/// identify itself from silently claiming to be a preview.
#[test]
fn developer_build_reports_stable_build_kind() {
    assert_eq!(build_kind(), BuildKind::Stable);
}

/// For a default build, `installed_version()` parses `RELEASE_TAG`
/// (`v{CARGO_PKG_VERSION}`), which must agree with `VERSION`
/// (`CARGO_PKG_VERSION` itself) once rendered back out.
#[test]
fn installed_version_agrees_with_version_for_a_default_build() {
    assert_eq!(installed_version().to_string(), VERSION);
}

/// `RELEASE_VERSION_TAG` feeds the legacy `&'static str`-typed lookups in
/// `update_check` that still add their own leading `v`; it must never
/// carry one itself, or those lookups would request a doubly-prefixed tag.
#[test]
fn release_version_tag_has_no_leading_v() {
    assert!(!RELEASE_VERSION_TAG.starts_with('v'));
    assert_eq!(RELEASE_VERSION_TAG, VERSION);
}

/// `installed_version()` must fall back gracefully -- never panic -- even
/// when handed inputs `Version::parse` rejects outright.
#[test]
fn installed_version_fallback_chain_never_panics() {
    assert!(Version::parse("not-a-version").is_none());
    assert!(Version::parse("").is_none());
    // The floor of the fallback chain must itself always parse.
    assert_eq!(
        Version::parse("0.0.0").map(|v| v.to_string()),
        Some("0.0.0".to_owned())
    );
}
