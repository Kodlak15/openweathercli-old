#!/bin/bash

ICONDIR=$1/icons

# Create the icons directory if it does not exist
if [[ ! -d $ICONDIR ]]; then
    mkdir $ICONDIR
fi

# Names of all icons to retrieve
declare -a icons=(
    "01d"
    "01n"
    "02d"
    "02n"
    "03d"
    "03n"
    "04d"
    "04n"
    "09d"
    "09n"
    "10d"
    "10n"
    "11d"
    "11n"
    "13d"
    "13n"
    "50d"
    "50n"
)

# If there are less than 18 items in icons directory, install
if [[ $(ls $ICONDIR | wc -l) -lt 18 ]]; then
    # Get each icon and save it in the icons directory
    for icon in "${icons[@]}" 
    do
        curl "https://openweathermap.org/img/wn/$icon@2x.png" --output "$ICONDIR/$icon.png"
    done
fi
