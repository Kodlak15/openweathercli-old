# About

This is command line interface for interacting with the [OpenWeatherMap](https://openweathermap.org/) API, written in Rust. Do note that I only intend to work on support for features included in the free tier. This is currently VERY early in devlopment. There are almost surely bugs, and the code base is not particularly elegant at this point in time. As of now the project has only been tested on Linux.

# Installation

```
# Assumes you have a directory called $HOME/bin, and that it has been added to your path
# To add to your path, add the following to your shell init script (.bashrc, .zshrc, etc):
export PATH="$HOME/bin:$PATH"

# From within the source directory:
./setup.sh

# Create a symbolic link for the binary to a directory included in PATH
ln -s {absolute_path_to_source}/target/release/openweathercli $HOME/bin

# You should now be able to run the program from anywhere as the current user
```

# Example usage

```
# Assumes openweathercli has been added to your PATH variable
# The following command prints the temperature for the location specified in $HOME/.config/config.yaml

openweathercli --key {your_api_key} --print temp --verbose
```

# Goals

I am still fairly new to Rust, so I am using this project as an excuse to get a better feel for the language. One of the motivating factors for building this project is to create a weather widget on my taskbar, and I thought it would be interesting and rewarding to build my own project to make the API calls.

# To do

- Fix existing bugs and optimize code
- Add support for multi-day forecast
- Improve user experience & command line functionality
- Build eww widget using project for example usage
