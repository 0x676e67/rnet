#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <target>"
  exit 1
fi

BASH_IMAGE="ghcr.io/0x676e67/rust-musl-cross"
TARGET=$1
VOLUME_MAPPING="-v $(pwd):/home/rust/src"
MATURIN_CMD="maturin build --release --out dist --find-interpreter"

case $TARGET in
  x86_64-unknown-linux-musl | \
  x86_64-unknown-linux-gnu | \
  aarch64-unknown-linux-musl | \
  aarch64-unknown-linux-gnu | \
  armv7-unknown-linux-musleabihf | \
  armv7-unknown-linux-gnueabihf | \
  i686-unknown-linux-musl | \
  i686-unknown-linux-gnu)
    ;;
  *)
    echo "Unknown target: $TARGET"
    exit 1
    ;;
esac

echo "Building for $TARGET..."
docker run --rm $VOLUME_MAPPING $BASH_IMAGE:$TARGET /bin/bash -c "$MATURIN_CMD"

echo "Build completed for target: $TARGET"