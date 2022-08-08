set -e
./x.py clean
RUSTFLAGS="--emit=obj -C code-model=large -C relocation-model=static" ./x.py build library/alloc --stage 0 --target ./x86_64-theseus.json
./build.sh
