#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <target> [maturin_args]"
  exit 1
fi

TARGET=$1
ARGS=$2

IMAGE="ghcr.io/0x676e67/rust-musl-cross"
VOLUME_MAPPING="-v $(pwd):/home/rust/src"
MATURIN_CMD="maturin build --release --out dist $ARGS"

case $TARGET in
  x86_64-unknown-linux-musl | \
  aarch64-unknown-linux-musl | \
  armv7-unknown-linux-musleabihf | \
  i686-unknown-linux-musl)
    ;;
  *)
    echo "Unknown target: $TARGET"
    exit 1
    ;;
esac

echo "Building for $TARGET..."
docker pull $IMAGE:$TARGET
docker run --rm $VOLUME_MAPPING $IMAGE:$TARGET /bin/bash -c "$MATURIN_CMD"

echo "Build completed for target: $TARGET"