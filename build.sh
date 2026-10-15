# shellcheck disable=SC2164
cd ./src-tauri
cargo tauri dev;
cd ../api/RustyChatApi/src
cargo run main.rs;