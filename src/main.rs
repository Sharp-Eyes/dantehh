use anyhow::{Context, Result};
use std::fs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use dantehh::*;


enum GameState {
    Window,
    Drive,
    MirrorDungeonMenu,
    MirrorDungeonMenuConfirm,
}


fn main() -> Result<()> {

    let config: Config = toml::from_str(
        fs::read_to_string("dantehh.toml")
            .context("Failed to read toml file")?
            .as_str(),
    )
    .expect("Failed to parse dantehh.toml");

    Command::new(&config.adb.path)
        .arg("connect")
        .arg(&config.adb.ip)
        .output()
        .context("Failed to connect to adb")?;

    let mut state = GameState::Window;

    loop {
        sleep(Duration::from_millis(800));

        match state {
            GameState::Window => {
                find_and_tap(&config, "menu-drive")?;
                state = GameState::Drive;
            },
            GameState::Drive => {
                find_and_tap(&config, "menu-md")?;
                state = GameState::MirrorDungeonMenu;
            }
            GameState::MirrorDungeonMenu => {
                find_and_tap(&config, "menu-md-simulation")?;
                state = GameState::MirrorDungeonMenuConfirm;
            }
            GameState::MirrorDungeonMenuConfirm => {
                find_and_tap(&config, "ENTER")?;
                // state = GameState::MirrorDungeonMenuConfirm;

                break;
            }
        }
    }

    Ok(())
}
