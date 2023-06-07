#!/bin/sh
set -ex
cd settings-macros
cargo publish
cd ..
cargo publish
cd examples/example_crate_with_settings
cargo publish
echo "done."
