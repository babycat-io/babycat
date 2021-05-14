#!/usr/bin/env python3
"""
Replaces the `version = "0.0.0" in the Cargo.toml with a more specific version.

Usage:
    .version-bump.py 0.1.2

This is to only be used in CI pipelines where we need to cut new versions.
"""
import re
import sys

version_num = sys.argv[1]

with open("Cargo.toml") as fh:
    cargo_toml = fh.read()

replaced = re.sub(
    r'version = "0.0.0"',
    f'version = "{version_num}"',
    cargo_toml,
    count = 1,
)

with open("Cargo.toml", "w") as fh:
    fh.write(replaced)
