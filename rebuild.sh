./x.py clean
./x.py build library/alloc --stage 0 --target ./x86_64-theseus.json
RUSTFLAGS="-L $PWD/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-theseus/release/deps" ./x.py build library/std --stage 0 --target ./x86_64-theseus.json 
