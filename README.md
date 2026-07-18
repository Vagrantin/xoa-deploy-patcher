# xoa-deploy-patcher

Small Rust CLI that patches XO Lite's `xoa-deploy.vue` at package-build time, turning the stock "Deploy XOA" screen into the Community Edition variant.

Instead of fragile whole-file diffs, it applies a list of named search/replace **patch definitions** anchored on structural landmarks in the upstream source (imports, template blocks, script sections). Each patch is validated: if a landmark is missing — e.g. after an upstream refactor — the tool reports which patch failed and exits non-zero, so the RPM build breaks loudly rather than shipping a half-patched UI.

## Usage

```bash
cargo build --release
./target/release/xoa-deploy-patcher <path-to-xoa-deploy.vue>
```

The file is modified in place.

## Where it's used

Vendored into `../xolite-ce/patches/xoa-deploy-patcher` and run during the `xo-lite-ce` RPM build (see `../xolite-ce/SPECS/xo-lite-community.spec`). The applied changes redirect the deploy flow to the community XOA image (URL resolved at runtime from `https://xo-image.yawn.fi/downloads/image.txt`) and route the download through `../xoa-proxy` with its per-request options (e.g. SSL verification toggle).
