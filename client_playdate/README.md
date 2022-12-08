# Bouquet Game Engine Playdate Client

- Only a stub implementation.
- A sample Playdate-based client to build out the Bouquet Game Engine.

One time setup:
```
# starting from the client_playdate package directory
# install crankstart in the same directory as the bouquet workspace
cd ../..
git clone git@github.com:pd-rs/crankstart.git
# install crank
cargo install --git=https://github.com/pd-rs/crank
```

Build instructions:
```
crank build --release --device
crank run --release
# from simulator, upload game to device
# run "bouquet test client" from games menu
```

