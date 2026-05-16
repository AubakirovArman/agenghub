#!/usr/bin/env sh
set -eu

repo="${AGENTHUB_REPO:-AubakirovArman/agenthub}"
version="${AGENTHUB_VERSION:-latest}"
install_dir="${AGENTHUB_INSTALL_DIR:-$HOME/.agenthub/bin}"
artifact="${AGENTHUB_ARTIFACT:-}"
checksum="${AGENTHUB_CHECKSUM:-}"
checksum_file="${AGENTHUB_CHECKSUM_FILE:-}"
skip_checksum="${AGENTHUB_SKIP_CHECKSUM:-0}"

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "agenthub installer: missing required command: $1" >&2
    exit 1
  fi
}

detect_asset() {
  os="$(uname -s)"
  arch="$(uname -m)"
  case "$os" in
    Linux) os_part="unknown-linux-gnu" ;;
    Darwin) os_part="apple-darwin" ;;
    *) echo "agenthub installer: unsupported OS: $os" >&2; exit 1 ;;
  esac
  case "$arch" in
    x86_64|amd64)
      if [ "$os" = "Darwin" ]; then
        echo "agenthub installer: Intel macOS releases are not published; use Apple Silicon macOS, Linux x86_64, or Windows x86_64" >&2
        exit 1
      fi
      arch_part="x86_64"
      ;;
    arm64|aarch64) arch_part="aarch64" ;;
    *) echo "agenthub installer: unsupported architecture: $arch" >&2; exit 1 ;;
  esac
  echo "agenthub-$arch_part-$os_part.tar.gz"
}

download() {
  url="$1"
  output="$2"
  if command -v curl >/dev/null 2>&1; then
    curl -fL "$url" -o "$output"
  elif command -v wget >/dev/null 2>&1; then
    wget -O "$output" "$url"
  else
    echo "agenthub installer: install curl or wget" >&2
    exit 1
  fi
}

compute_sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  else
    echo "agenthub installer: install sha256sum or shasum, or set AGENTHUB_SKIP_CHECKSUM=1" >&2
    exit 1
  fi
}

read_checksum_file() {
  awk 'NF {print $1; exit}' "$1"
}

verify_checksum() {
  archive="$1"
  expected="$checksum"
  if [ "$skip_checksum" = "1" ]; then
    echo "agenthub installer: checksum verification skipped"
    return
  fi
  if [ -n "$checksum_file" ]; then
    expected="$(read_checksum_file "$checksum_file")"
  elif [ -z "$expected" ] && [ -f "$archive.sha256" ]; then
    expected="$(read_checksum_file "$archive.sha256")"
  fi
  if [ -z "$expected" ]; then
    echo "agenthub installer: missing checksum; set AGENTHUB_CHECKSUM, AGENTHUB_CHECKSUM_FILE, or AGENTHUB_SKIP_CHECKSUM=1" >&2
    exit 1
  fi

  actual="$(compute_sha256 "$archive")"
  if [ "$actual" != "$expected" ]; then
    echo "agenthub installer: checksum mismatch for $archive" >&2
    echo "  expected: $expected" >&2
    echo "  actual:   $actual" >&2
    exit 1
  fi
  echo "agenthub installer: checksum verified"
}

need_cmd tar
need_cmd mktemp
need_cmd awk

asset="$(detect_asset)"
tmp="$(mktemp -d "${TMPDIR:-/tmp}/agenthub-install.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT INT TERM

if [ -n "$artifact" ]; then
  archive="$artifact"
else
  archive="$tmp/$asset"
  if [ "$version" = "latest" ]; then
    url="https://github.com/$repo/releases/latest/download/$asset"
  else
    url="https://github.com/$repo/releases/download/$version/$asset"
  fi
  download "$url" "$archive"
  if [ "$skip_checksum" != "1" ] && [ -z "$checksum" ] && [ -z "$checksum_file" ]; then
    download "$url.sha256" "$archive.sha256"
  fi
fi

verify_checksum "$archive"
tar -xzf "$archive" -C "$tmp"
binary="$(find "$tmp" -type f -name agenthub | head -n 1)"
if [ -z "$binary" ]; then
  echo "agenthub installer: archive does not contain agenthub binary" >&2
  exit 1
fi

mkdir -p "$install_dir"
cp "$binary" "$install_dir/agenthub"
chmod +x "$install_dir/agenthub"

echo "agenthub installed to $install_dir/agenthub"
echo "Add this directory to PATH if needed:"
echo "  export PATH=\"$install_dir:\$PATH\""
