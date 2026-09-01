#!/usr/bin/env python3
"""Compute the next release version for `.github/workflows/release.yml`.

This is the only non-trivial logic in the release workflow, extracted so it
can be unit tested (see `scripts/test_release_version.py`) rather than living
as an inline, untestable Python heredoc.

Two publication modes are supported:

- `stable` -- bump the core `major.minor.patch` version from `Cargo.toml`
  and fail if the resulting `vMAJOR.MINOR.PATCH` tag already exists.
- `rc` -- bump the core version the same way, then scan the existing tags
  for `v<core>-rc.<N>` and pick `N = max + 1` (numerically, not
  lexicographically -- `rc.10` sorts after `rc.9`), producing e.g.
  `0.5.0-rc.1`, then `0.5.0-rc.2`. Prerelease identity travels this way
  instead of through `Cargo.toml` (see D3 in the release-channel design
  notes): an RC run never writes a prerelease version into the manifest.

The script prints the resulting version (without a leading `v`) to stdout on
success, e.g. `0.5.1` or `0.5.0-rc.1`. On failure it prints a message to
stderr and exits non-zero.
"""

from __future__ import annotations

import argparse
import re
import sys

CORE_PATTERN = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")

BUMP_LEVELS = ("major", "minor", "patch")
MODES = ("stable", "rc")


class VersionError(ValueError):
    """Raised for any invalid input or a version collision."""


def parse_core(version: str) -> tuple[int, int, int]:
    """Parses a plain `major.minor.patch` string, as found in `Cargo.toml`."""
    match = CORE_PATTERN.match(version.strip())
    if match is None:
        raise VersionError(
            f"current version must be a plain major.minor.patch: {version!r}"
        )
    major, minor, patch = (int(part) for part in match.groups())
    return major, minor, patch


def bump_core(version: str, bump: str) -> tuple[int, int, int]:
    """Bumps a parsed core version by one `major`, `minor`, or `patch` step."""
    major, minor, patch = parse_core(version)
    if bump == "major":
        return major + 1, 0, 0
    if bump == "minor":
        return major, minor + 1, 0
    if bump == "patch":
        return major, minor, patch + 1
    raise VersionError(f"unsupported version bump: {bump!r}")


def format_core(core: tuple[int, int, int]) -> str:
    major, minor, patch = core
    return f"{major}.{minor}.{patch}"


def split_tags(raw_tags: str) -> list[str]:
    """Splits a whitespace-separated tag list, as produced by `git tag -l`."""
    return [tag for tag in raw_tags.split() if tag]


def next_rc_ordinal(core: str, existing_tags: list[str]) -> int:
    """Finds the next RC ordinal for `core`, comparing existing `rc.N`
    suffixes numerically -- so `rc.10` sorts after `rc.9`, never before it
    the way a plain string comparison would.
    """
    prefix = f"v{core}-rc."
    highest = 0
    for tag in existing_tags:
        if not tag.startswith(prefix):
            continue
        suffix = tag[len(prefix) :]
        if suffix.isdigit():
            highest = max(highest, int(suffix))
    return highest + 1


def ensure_tag_available(tag: str, existing_tags: list[str]) -> None:
    """Fails if `tag` is already present in `existing_tags`.

    Mirrors the workflow's pre-existing stable-release guard, and applies to
    RC releases the same way: a computed tag must never silently overwrite
    an existing one.
    """
    if tag in existing_tags:
        raise VersionError(f"tag {tag} already exists")


def compute_next_version(
    current_version: str, bump: str, mode: str, existing_tags: list[str]
) -> str:
    """Computes the next release version, without a leading `v`.

    `existing_tags` is every tag already published (as full tag names, e.g.
    `v0.5.0` or `v0.5.0-rc.2`), used to reject a collision and, for `rc`, to
    find the next ordinal.
    """
    if mode not in MODES:
        raise VersionError(f"unsupported mode: {mode!r}")

    core = format_core(bump_core(current_version, bump))

    if mode == "stable":
        tag = f"v{core}"
        ensure_tag_available(tag, existing_tags)
        return core

    ordinal = next_rc_ordinal(core, existing_tags)
    version = f"{core}-rc.{ordinal}"
    ensure_tag_available(f"v{version}", existing_tags)
    return version


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--current-version",
        required=True,
        help="the current plain major.minor.patch version, from Cargo.toml",
    )
    parser.add_argument("--bump", required=True, choices=BUMP_LEVELS)
    parser.add_argument("--mode", required=True, choices=MODES)
    parser.add_argument(
        "--tags",
        default="",
        help="every existing tag, whitespace-separated (e.g. `git tag -l 'v*'` output)",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    try:
        version = compute_next_version(
            args.current_version, args.bump, args.mode, split_tags(args.tags)
        )
    except VersionError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print(version)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
