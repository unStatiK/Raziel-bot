# Raziel-bot
fork from bot template https://github.com/unStatiK/serenity-bot-skeleton for custom functionality 

# Commands

* whois - list of whois info for domains
* version - get bot version
* uptime - get bot start time
* cur - get currencies rates
* commands - get list enable commands
* foxy - get random image link with fox


# Build

RUSTFLAGS="-C target-cpu=native" CARGO_HOME="<path_to_cargo_dir>" RUSTUP_HOME="<path_to_rustup_dir>" cargo build --release

# Run

DISCORD_TOKEN=<discord_bot_token> ./target/release/raziel.exe

# For developers style guideline

-  import order: extern crates, project crates, std imports, serenity imports, other imports
-  commands file names should same as command name
-  at command file async functions should be declared first then normal functions
-  always call init() and registry() methods for all commands in init_command_system() and registry_commands()
