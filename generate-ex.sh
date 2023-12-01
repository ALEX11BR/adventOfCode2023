#!/usr/bin/env sh

if [ -z "$1" ]; then
    echo !!! Supply ex number !!!
    exit 1
fi

cp -r ex1 "ex$1"

sed -i s/ex1/ex"$1"/g "ex$1/Cargo.toml"
sed -i '/^\]$/i\ \ \ \ "'"ex$1\"," Cargo.toml
