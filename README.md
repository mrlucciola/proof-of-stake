# Proof-of-Stake Blockchain

## Repo init

```zsh
# Configure yarn 1
# visit https://classic.yarnpkg.com/lang/en/docs/install/#mac-stable
yarn --version # should == 1.22.19

# install deps
yarn

# Configure rust (https://www.rust-lang.org/tools/install)
# Version at latest check (cargo 1.70.0 (ec8a8a0ca 2023-04-25)) - may work with older versions
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install protobuf via homebrew (https://docs.brew.sh/Installation)
brew install protobuf
```

## Testing

Run tests with:
`cargo test`

### Running multi-node tests

Open separate terminal tab (tab 2);
(In tab 2) Run binary from `p2p.rs` to spin up node at `localhost:8898`;
`cargo run --bin p2p`

With node at port `8898` running, run node test;
`cargo test node`
