#!/bin/sh

cargo build --release || exit 1

elf2uf2-rs target/thumbv6m-none-eabi/release/kbd-core || echo "Failed to generate uf2 file"

cp target/thumbv6m-none-eabi/release/kbd-core.uf2 kbd-core.uf2
cp target/thumbv6m-none-eabi/release/kbd-core kbd-core.bin

du -h kbd-core.bin
du -h kbd-core.uf2
