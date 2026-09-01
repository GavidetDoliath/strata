// SPDX-License-Identifier: GPL-3.0-or-later

use crate::services::{BuildKind, Channel, ReleaseMetadata, UpdateCheck, Version};

use super::{
    COMPACT_NAVIGATION_BREAKPOINT, DIALOG_HEIGHT, DIALOG_MARGIN, DIALOG_WIDTH, channel_copy,
    installed_version_status, responsive_dialog_size, shows_available_release_notes,
    theme_background_is_light, theme_name_matches, uses_compact_navigation,
};

#[test]
fn settings_dialog_keeps_its_preferred_size_when_space_allows() {
    assert_eq!(
        responsive_dialog_size(
            DIALOG_WIDTH + DIALOG_MARGIN * 2,
            DIALOG_HEIGHT + DIALOG_MARGIN * 2,
        ),
        (DIALOG_WIDTH, DIALOG_HEIGHT)
    );
}

#[test]
fn settings_dialog_shrinks_to_leave_a_margin_in_small_windows() {
    assert_eq!(responsive_dialog_size(640, 480), (592, 432));
}

#[test]
fn settings_dialog_size_stays_valid_at_tiny_allocations() {
    assert_eq!(responsive_dialog_size(20, 20), (1, 1));
}

#[test]
fn settings_navigation_compacts_below_the_breakpoint() {
    assert!(uses_compact_navigation(COMPACT_NAVIGATION_BREAKPOINT - 1));
    assert!(!uses_compact_navigation(COMPACT_NAVIGATION_BREAKPOINT));
}

#[test]
fn theme_search_is_case_insensitive_and_ignores_outer_whitespace() {
    assert!(theme_name_matches("Tokyo Night Storm", " night "));
    assert!(theme_name_matches("Dracula", "DRAC"));
    assert!(theme_name_matches("Nord", ""));
    assert!(!theme_name_matches("Solarized Light", "dark"));
}

#[test]
fn theme_appearance_uses_background_luminance() {
    assert!(theme_background_is_light("#ffffff"));
    assert!(theme_background_is_light("#efecf4"));
    assert!(!theme_background_is_light("#1e1d1f"));
    assert!(!theme_background_is_light("invalid"));
}

#[test]
fn available_notes_are_shown_only_for_a_newer_release() {
    assert!(!shows_available_release_notes(&UpdateCheck::UpToDate));
    assert!(!shows_available_release_notes(&UpdateCheck::Failed(
        "offline".to_owned()
    )));
    assert!(shows_available_release_notes(&UpdateCheck::Available {
        release: ReleaseMetadata {
            version: "1.0.0".to_owned(),
            url: "https://example.test/release".to_owned(),
            notes: "Changes".to_owned(),
            note_blocks: vec![crate::services::ReleaseNoteBlock::Paragraph(
                "Changes".to_owned(),
            )],
            kind: BuildKind::Stable,
            tag: "v1.0.0".to_owned(),
            published_at: None,
            commit: None,
        },
        download_url: "https://example.test/download".to_owned(),
    }));
}

#[test]
fn channel_copy_uses_the_issues_exact_language() {
    assert_eq!(
        channel_copy(Channel::Stable),
        ("Stable", "Final releases only.")
    );
    assert_eq!(
        channel_copy(Channel::Preview),
        (
            "Nightly / preview",
            "Preview builds, including release candidates. These may be unstable."
        )
    );
}

#[test]
fn installed_version_status_stays_plain_for_a_stable_build() {
    let version = Version::parse("0.6.0").expect("valid version");
    assert_eq!(
        installed_version_status(&version, BuildKind::Stable),
        "Version 0.6.0"
    );
}

#[test]
fn installed_version_status_names_the_build_kind_for_a_prerelease() {
    let version = Version::parse("0.6.0-rc.1").expect("valid version");
    assert_eq!(
        installed_version_status(&version, BuildKind::Rc),
        "Version 0.6.0-rc.1 · Release candidate"
    );
}
