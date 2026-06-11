# Patch layer over gpui-unofficial

This branch (`patches`) is the upstream release tag plus a small patch
stack. It is consumed via `[patch.crates-io]` from the gpui-component
workspace, which pins `gpui-unofficial`, `gpui-macos-gpui-unofficial`,
and `gpui-windows-gpui-unofficial` to this branch.

## Current patches

| Commit | Upstream source | Drop when |
|---|---|---|
| Glass content rendering | [zed-industries/zed#58833](https://github.com/zed-industries/zed/pull/58833) | The PR ships in a Zed release |
| Remove `crates/transformed-crates` | n/a (release artifact cleanup) | Upstream stops committing the duplicate copy |

Package versions on this branch must stay identical to the release tag
(`1.2.7`), otherwise they no longer satisfy the workspace's version
requirements through `[patch.crates-io]`.

## Updating to a new release

```sh
git fetch upstream --tags
git tag patches-v1.2.7 patches        # archive the old state
git rebase --onto v1.3.0 v1.2.7 patches
git push -f origin patches
```

The `transformed-crates` removal commit will raise delete/modify
conflicts for any file upstream changed in that copy; resolve with
`git rm -r crates/transformed-crates && git rebase --continue`.

Then, in the workspace: bump the `1.2.7` versions in the root
`Cargo.toml` and refresh the lockfile pins:

```sh
cargo update -p gpui-unofficial -p gpui-macos-gpui-unofficial -p gpui-windows-gpui-unofficial
```

If a rebased patch no longer applies because upstream changed the same
code, check whether the feature landed upstream — if so, delete the
patch commit and, once the stack is empty, remove the
`[patch.crates-io]` entries from the workspace instead.
