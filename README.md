## Build nodeos
```
git clone https://github.com/EOSIO/eos --recursive
cd eos && git submodule update --init --recursive
./eosio_build.sh
cd build && sudo make install
```

## Build wasm tooling
```
git clone --recursive https://github.com/WebAssembly/wabt
cd wabt && make && sudo make install
rustup update
cargo default nightly
cargo install wasm-gc
rustup target add wasm32-unknown-unknown
```


## Starting private blockchain
```
nodeos -e -p eosio --plugin eosio::chain_api_plugin --plugin eosio::history_api_plugin
```

## Setting up wallet and accounts
In `eos` folder
```
cleos wallet create
cleos wallet unlock
cleos set contract eosio build/contracts/eosio.bios -p eosio@active
cleos create key
cleos wallet import <private key from previous step>
# You can use public key from previous step
cleos create account eosio <username> <owner_public_key> <active_public_key>
```

## List accounts by key
`cleos get accounts <public_key>`


## Execute contract action
`cleos push action hello.code hi '["user"]' -p hello.code`

