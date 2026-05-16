#!/usr/bin/env sh
set -eu

printf "rustc: "
rustc --version
printf "cargo: "
cargo --version

if command -v stellar >/dev/null 2>&1; then
  printf "stellar: "
  stellar --version
else
  printf "stellar: not installed\n"
fi

if rustc --print target-list | grep -qx "wasm32v1-none"; then
  printf "wasm target available in toolchain list: yes\n"
else
  printf "wasm target available in toolchain list: no\n"
fi

if [ -d "$(rustc --print sysroot)/lib/rustlib/wasm32v1-none/lib" ]; then
  printf "wasm target stdlib installed: yes\n"
else
  printf "wasm target stdlib installed: no\n"
fi
