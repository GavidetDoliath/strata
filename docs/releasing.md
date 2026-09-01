# Releasing

Strata publishes signed, attested Linux binaries through the **Release** GitHub Actions workflow (`.github/workflows/release.yml`). This is the maintainer runbook for cutting a release, plus the tag grammar the self-updater depends on.

## Tag grammar

Exactly these three forms are valid release tags. The self-updater rejects anything else outright, since this is the only contract it parses:

| Form | Example | Meaning |
| --- | --- | --- |
| `v?MAJOR.MINOR.PATCH` | `v0.5.0` | A final, stable release. |
| `v?MAJOR.MINOR.PATCH-rc.N` | `v0.5.0-rc.1` | A release candidate for the given core version. `N` is a positive integer, compared numerically -- `rc.10` sorts after `rc.9`, not before it. |
| `v?MAJOR.MINOR.PATCH-nightly.YYYYMMDD[.N]` | `v0.5.0-nightly.20260901` | A nightly build for the given core version, dated `YYYYMMDD`, with an optional same-day disambiguator `N`. |

The leading `v` is optional in the grammar but always present in tags this workflow creates. At an equal core version, a release candidate outranks a nightly (`0.5.0-rc.1` sorts above `0.5.0-nightly.20260901`) -- see D5 in the release-channel design notes. Strata does not yet publish nightly builds; the workflow this runbook describes covers stable and release-candidate publication only.

## Publishing a stable release

Run the **Release** workflow from GitHub's Actions tab on the default branch, choose a `bump` (`patch`, `minor`, or `major`), and leave `mode` at its default, `stable`. Once both Linux targets build:

- the `prepare` job refuses to proceed if a release candidate tag exists for the target core version whose commit is not yet reachable from the release source -- promote or discard that RC first, so a stable release can never silently supersede an untested one;
- the `release` job commits the new version into `Cargo.toml` and `Cargo.lock`, tags the commit `vX.Y.Z`, and pushes both to the default branch; and
- it publishes x86-64 and ARM64 archives, checksums, and build-provenance attestations as an ordinary (non-prerelease) GitHub release -- the endpoint a Stable install polls.

## Cutting and promoting a release candidate

Run the same workflow with `mode` set to `rc`. The workflow:

- computes the next core version from `bump` the same way a stable release would, then scans existing `v<core>-rc.*` tags and picks the next ordinal -- the first RC for a line is `rc.1`;
- never touches `Cargo.toml` or `Cargo.lock`, and never pushes a commit to the default branch (D3) -- it tags the source commit directly and pushes only the tag;
- injects the prerelease identity into the build through the `STRATA_RELEASE_TAG` and `STRATA_BUILD_KIND=rc` environment variables, which `build.rs` reads at compile time; and
- publishes the GitHub release with `--prerelease`, which keeps `/releases/latest` -- what a Stable install polls -- pointing at the last final release. Only a Preview install ever sees it.

To promote a validated RC line to stable, run the workflow again with `mode: stable` and the same `bump` level used for the RC. The resulting `vX.Y.Z` stable tag supersedes the RC line; the guard described above only blocks this if the RC's commit was never folded into the stable release's history.

## Version calculation

The version arithmetic described above is implemented in [`scripts/release_version.py`](../scripts/release_version.py), extracted out of the workflow (it was previously an inline heredoc) so it can be unit tested. Run its tests with:

```bash
python3 scripts/test_release_version.py
```

or the way CI runs every script test in the repo:

```bash
python3 -m unittest discover -s scripts -p 'test_*.py'
```

Cases covered:

| Case | Input | Result |
| --- | --- | --- |
| Stable patch bump | `0.5.0`, `patch`, `stable`, no tags | `0.5.1` |
| Stable minor bump | `0.5.7`, `minor`, `stable`, no tags | `0.6.0` |
| Stable major bump | `0.5.7`, `major`, `stable`, no tags | `1.0.0` |
| Stable tag collision | `0.5.0`, `patch`, `stable`, tags include `v0.5.1` | rejected |
| First RC for a core with no existing RC tags | `0.5.0`, `patch`, `rc`, no tags | `0.5.1-rc.1` |
| RC after an existing RC | `0.5.0`, `patch`, `rc`, tags include `v0.5.1-rc.1` | `0.5.1-rc.2` |
| RC ordinal is numeric, not lexicographic | `0.5.0`, `patch`, `rc`, tags include `v0.5.1-rc.1` .. `v0.5.1-rc.10` | `0.5.1-rc.11` |
| RC tag collision (defense in depth) | computed tag already present | rejected |
| Unrelated tags ignored | tags for other cores, or other build kinds (stable, nightly) mixed in | ignored |

## Known limitation

This repository has no workflow test harness. The version-calculation logic above is unit tested; the workflow's job wiring, environment plumbing, and git/`gh` interactions are verified only by manual dry runs of the underlying shell logic against this repository's real `Cargo.toml` and tags, not by an actual GitHub Actions run.
