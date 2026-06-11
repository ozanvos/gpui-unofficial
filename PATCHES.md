# Patch layer over gpui-unofficial

This branch (`patches`) is the upstream release tag plus a small patch
stack. It is consumed via `[patch.crates-io]` from the gpui-component
workspace, which pins `gpui-unofficial`, `gpui-macos-gpui-unofficial`,
`gpui-windows-gpui-unofficial`, and `gpui-wgpu-gpui-unofficial` to this
branch.

## Current patches

| Commit | Upstream source | Drop when |
|---|---|---|
| Glass content rendering (`Styled::glass`) | [zed-industries/zed#58833](https://github.com/zed-industries/zed/pull/58833) | The PR ships in a Zed release |
| BlurRect/LensRect backdrop primitives | [zed-industries/zed#54512](https://github.com/zed-industries/zed/pull/54512) | An equivalent ships in a Zed release |
| `Styled::backdrop_blur` + `backdrop_blur` example | ours (declarative layer over `Window::paint_blur_rect`) | The upstream feature includes a styled API |
| Remove `crates/transformed-crates` | n/a (release artifact cleanup) | Upstream stops committing the duplicate copy |
| `.cargo/config.toml` + `.gitignore` dev setup | n/a | — |

Package versions on this branch must stay identical to the release tag
(`1.2.7`), otherwise they no longer satisfy the workspace's version
requirements through `[patch.crates-io]`.

## Developing on this branch

There is no root workspace (each transformed crate carries its own
`[workspace]` table), but `.cargo/config.toml` routes the unofficial
crates to their local copies for any build rooted in this repo:

```sh
cd crates/gpui-unofficial
cargo check
cargo run --example backdrop_blur --features gpui_platform/runtime_shaders
```

`gpui_platform/runtime_shaders` skips ahead-of-time Metal shader
compilation, which requires the Xcode Metal Toolchain. Consumers never
read this config file — cargo configs only load from the consuming
build's own directory tree.

The DirectX (`gpui-windows`) side of the glass patch and the WGPU side
of the blur patch are not compiled on macOS hosts; the consuming
workspace's CI (Windows target, story-web wasm build) is what exercises
them.

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
cargo update -p gpui-unofficial -p gpui-macos-gpui-unofficial \
  -p gpui-windows-gpui-unofficial -p gpui-wgpu-gpui-unofficial
```

If a rebased patch no longer applies because upstream changed the same
code, check whether the feature landed upstream — if so, delete the
patch commit and, once the stack is empty, remove the corresponding
`[patch.crates-io]` entries from the workspace.
