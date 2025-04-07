# SDR Antenna Pattern Plotter

A handy tool to plot the radiation pattern of an antenna using an RTL-SDR and a
signal source.

## Usage

1. Wire up an antenna to the RTL-SDR and connect it to your laptop
1. Make sure the laptop is in the same wireless network as your mobile phone.
1. Start up the SDR antenna pattern plotter on the laptop.
1. Visit the Web-application in the browser of your mobile phone
1. Set your desired frequency and number of measurements in the Web-application.
1. Optionally connect the signal source to an reference antenna and perform the
    calibration.
1. Connect the signal source to your actual antenna and turn it as described in
    the Web-application while measuring the radiated signals strength.

## Development

The project is split into two parts:
1. a frontend written mostly in Typescript using Svelte and TailwindCSS
1. a backend written in Rust

### Using docker

When using docker for development, there is no need to install Rust or Node.js
locally.
Several convenience services are defined inside the
[`docker-compose.yaml`](./docker-compose.yaml) to wrap commonly used tools.

For frontend development use

```
# Install packages
docker compose run --rm yarn install
# Run eslint
docker compose run --rm yarn lint
# Format the code with prettier
docker compose run --rm yarn format
# Run svelte-check
docker compose run --rm yarn svelte-check
# Run vite on port 5173
docker compose run --rm --service-ports yarn dev
# Build the frontend for deployment
docker compose run --rm yarn build
# Add a package
docker compose run --rm yarn add <package>
```

For backend development use

```
# Format the code
docker compose run --rm cargo fmt
# Add a crate
docker compose run --rm cargo add <crate>
# Run the tool with custom arguments (only port 8000 is exposed by default)
docker compose run --rm --service-ports cargo run -- --address=0.0.0.0
# Run the tool with the frontend exposed to https://0.0.0.0:8000
docker compose run --rm --service-ports sdr_antenna_pattern_plotter
```
