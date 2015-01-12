export LD_LIBRARY_PATH=/home/ncameron/rust3/x86_64-unknown-linux-gnu/stage1/lib
export RUSTC=/home/ncameron/rust3/x86_64-unknown-linux-gnu/stage1/bin/rustc
$RUSTC ./src/mod.rs -o "obj/deglob" -L /home/ncameron/reprint/obj
