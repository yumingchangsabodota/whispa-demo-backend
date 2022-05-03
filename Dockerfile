FROM rustlang/rust:nightly

ARG PROFILE=release

WORKDIR whispa-node

# Upcd dates core parts
RUN apt-get update -y && \
	apt-get install -y cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev

# Install rust wasm. Needed for substrate wasm engine
RUN rustup target add wasm32-unknown-unknown

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY runtime runtime
COPY pallets pallets
COPY node node
COPY node_modules node_modules

RUN mkdir chain-state
RUN cargo build "--$PROFILE"

EXPOSE 30333 9933 9944 9615

#./target/release/node-template --base-path ./chain-state/alice --chain dev --alice --ws-external --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
##./target/release/node-template --base-path ./chain-state/alice --chain dev --alice --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator