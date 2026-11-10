# Releasing

A release is a tag `vX.Y.Z` on `main`. Pushing it runs [`.github/workflows/release.yml`](../.github/workflows/release.yml), which:

1. runs every check in `check.yml` (the release cannot publish if one fails);
2. builds the packages with [`scripts/build-packages.sh`](../scripts/build-packages.sh) in an Ubuntu 22.04 container, so the programs need glibc 2.35 or newer;
3. installs them on Ubuntu 22.04, 24.04 and 26.04, Debian 12 and 13, Fedora 43 and 44, and Arch with [`scripts/test-packages.sh`](../scripts/test-packages.sh), which checks that the three programs, the menu entry and every library are there, that `bigman inspect` works and that `bigmand` starts;
4. creates a **draft** release with the files below and signed build provenance.

| File | For |
|---|---|
| `bigman_X.Y.Z_amd64.deb` | Debian, Ubuntu |
| `bigman-X.Y.Z-1.x86_64.rpm` | Fedora, RHEL-family |
| `bigman-X.Y.Z-x86_64-linux.tar.gz` | anything else: `./install.sh` under `~/.local`, or `sudo ./install.sh /usr/local` |
| `PKGBUILD` | Arch: the AUR package `bigman-bin`, built from the tarball |
| `bigman-extension-X.Y.Z.zip` | the Chromium extension, to load unpacked or upload to the Chrome Web Store |
| `SHA256SUMS` | checksums of all of the above |

Each package installs `/usr/bin/bigman`, `/usr/bin/bigmand`, `/usr/bin/bigman-app`, `bigman-app.desktop` (named after the window's application ID so Wayland desktops show its icon) and the icons. Nothing starts at install; the app's Setup tab installs the per-user services, which name the programs in `/usr/bin`.

There is no AppImage: Setup writes the daemon's path into systemd units and browser host manifests, and an AppImage's path changes every time it runs.

## Steps

1. Make sure `main` is green in Actions.
2. Rehearse: **Actions → release → Run workflow** on `main`. Everything except publishing runs; the built files are under the run's artifacts. Install one by hand if you want to look at it.
3. Set the version, commit and tag:
   ```sh
   python3 scripts/version.py set 1.0.0     # Cargo.toml, Cargo.lock, tauri.conf.json, package.json files, extension manifest
   git commit -am 'Release 1.0.0'
   git tag -a v1.0.0 -m 'Bigman 1.0.0'
   git push origin main v1.0.0
   ```
   Versions are plain `X.Y.Z`: Chrome's manifest takes only numbers, rpm forbids `-` in a version, and dpkg would sort `1.0.0-rc.1` after `1.0.0`. CI fails if any file disagrees with `Cargo.toml` (`python3 scripts/version.py check`), and the release fails if the tag does not match.
4. When the workflow finishes, open the draft under **Releases**, write the notes (what changed, anything users must do), and publish.
5. Arch: copy the release's `PKGBUILD` into the `bigman-bin` AUR repository, run `makepkg --printsrcinfo > .SRCINFO`, commit and push. The first time, create an AUR account with an SSH key and push to `ssh://aur@aur.archlinux.org/bigman-bin.git`.
6. Chrome Web Store (optional): upload `bigman-extension-X.Y.Z.zip` from the developer dashboard.

If a release is broken, delete the draft (or the release) and the tag, fix, and tag again with the next patch version once anything was published.

## Building packages locally

```sh
scripts/build-packages.sh                  # writes dist/
podman run --rm -v "$PWD:/src:ro" -w /src debian:12 scripts/test-packages.sh
```

Files built on a newer distribution than Ubuntu 22.04 need that distribution's glibc or newer.
