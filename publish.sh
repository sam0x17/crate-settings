#!/bin/sh
set -ex
cd settings-macros
cargo publish
cd ..
cargo publish
echo "done."
