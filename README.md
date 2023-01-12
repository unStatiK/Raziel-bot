# Raziel-bot
fork from bot template https://github.com/unStatiK/serenity-bot-skeleton for custom functionality 

# Commands

* whois - list of whois info for domains
* version - get bot version
* uptime - get bot start time
* cur - get currencies rates


# Build

RUSTFLAGS="-C target-cpu=native" CARGO_HOME="<path_to_cargo_dir>" RUSTUP_HOME="<path_to_rustup_dir>" cargo build --release

# Run

DISCORD_TOKEN=<discord_bot_token> ./target/release/raziel.exe
