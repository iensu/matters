#!/usr/bin/env bash

set -e

nix build .#site
python3 -m http.server --directory result
