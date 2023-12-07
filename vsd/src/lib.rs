mod commands;
mod cookie;
mod dash;
mod downloader;
mod hls;
mod merger;
mod playlist;
mod update;
mod utils;

use clap::Parser;
use std::ffi::OsString;

use commands::{Args, Commands};

#[no_mangle]
pub unsafe extern "C" fn save(
    cli_args: *const u8,
    cli_args_length: u32,
    __callback__: extern "C" fn(*const u8, u32),
) {
    let step1 = std::slice::from_raw_parts(cli_args, cli_args_length as usize);
    let step2 = std::str::from_utf8(step1).unwrap();
    let args = Args::try_parse_from([OsString::from(step2)]).unwrap();

    match args.command {
        Commands::Extract(args) => args.execute(),
        Commands::Merge(args) => args.execute(),
        Commands::Save(args) => args.execute(),
    }
    .unwrap();
}
