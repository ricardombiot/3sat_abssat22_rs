cd ./../
cargo build --release
rm ./test_3sat_rs/abs3sat
mv ./target/release/abs3sat ./test_3sat_rs/abs3sat