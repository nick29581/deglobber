#export LD_LIBRARY_PATH=/home/ncameron/rust3/x86_64-unknown-linux-gnu/stage1/lib
target/deglobber

#export RUSTC=/home/ncameron/rust3/x86_64-unknown-linux-gnu/stage1/bin/rustc
#$RUSTC ./data/hello.rs --pretty expanded,identified

cat data/hello.rs
rm data/hello.rs
mv data/hello.rs.bk data/hello.rs
