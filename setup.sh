#!/bin/bash

CONFIGDIR=$HOME/.config/owcli

# Create config directory if it does not exist
if [[ ! -d $CONFIGDIR ]]; then
    mkdir $CONFIGDIR
fi

# Create configuration file if it does not exist
if [[ ! -f $CONFIGDIR/config.yaml ]]; then
    echo "lat: 0" >> $CONFIGDIR/config.yaml
    echo "lon: 0" >> $CONFIGDIR/config.yaml
    echo "units: 'imperial'" >> $CONFIGDIR/config.yaml
fi

# Install icons
./scripts/install_icons.sh $CONFIGDIR

# Build project binary
cargo build --release
