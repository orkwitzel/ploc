#!/bin/sh
set -eu

REPO="orkwitzel/ploc"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
MAN_DIR="${MAN_DIR:-$HOME/.local/share/man/man1}"

need() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "ploc install: missing required command: $1" >&2
        exit 1
    fi
}

target_archive() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os:$arch" in
        Linux:x86_64)
            printf '%s\n' "ploc-linux-x86_64.tar.gz"
            ;;
        Darwin:x86_64)
            printf '%s\n' "ploc-macos-x86_64.tar.gz"
            ;;
        Darwin:arm64)
            printf '%s\n' "ploc-macos-aarch64.tar.gz"
            ;;
        *)
            echo "ploc install: unsupported platform: $os $arch" >&2
            exit 1
            ;;
    esac
}

need curl
need tar
need mktemp

archive="$(target_archive)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT INT HUP TERM

release_url="https://github.com/$REPO/releases/latest/download/$archive"

echo "Downloading $release_url"
curl -fsSL "$release_url" -o "$tmp_dir/$archive"
tar -xzf "$tmp_dir/$archive" -C "$tmp_dir"

mkdir -p "$INSTALL_DIR" "$MAN_DIR"
install -m 0755 "$tmp_dir/ploc" "$INSTALL_DIR/ploc"

if [ -f "$tmp_dir/share/man/man1/ploc.1" ]; then
    install -m 0644 "$tmp_dir/share/man/man1/ploc.1" "$MAN_DIR/ploc.1"
else
    curl -fsSL "https://raw.githubusercontent.com/$REPO/master/share/man/man1/ploc.1" \
        -o "$MAN_DIR/ploc.1"
fi

echo "Installed ploc to $INSTALL_DIR/ploc"
echo "Installed man page to $MAN_DIR/ploc.1"
echo "Make sure $INSTALL_DIR is on your PATH."
