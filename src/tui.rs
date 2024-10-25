use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo},
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
    ExecutableCommand, QueueableCommand,
};

use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use crate::player::Player;

pub fn setup_terminal() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout()
        .execute(EnterAlternateScreen)?
        //.execute(SetSize(MAP_HEIGHT.try_into().unwrap(), MAP_WIDTH.try_into().unwrap()))?
        .execute(Hide)?
        .execute(Clear(ClearType::All))?;

    Ok(())
}

pub fn teardown_terminal() -> std::io::Result<()> {
    terminal::disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

pub fn draw_map(map: &Map) -> std::io::Result<()> {
    stdout().execute(Clear(ClearType::All))?;

    for i in 0..MAP_HEIGHT {
        for j in 0..MAP_WIDTH {
            let index = map.coordinates_to_index(i as i32, j as i32);
            if map.tiles[index] == TileType::Wall {
                stdout()
                    .queue(MoveTo(j as u16, i as u16))?
                    .queue(Print('█'))?;
            }
        }
    }

    Ok(())
}

pub fn draw_player(player: &Player) -> std::io::Result<()> {
    let index = player.creature.position.i * 60 + player.creature.position.j;
    stdout()
        .queue(MoveTo(
            player.creature.position.j as u16,
            player.creature.position.i as u16,
        ))?
        .queue(Print('@'))?
        .queue(MoveTo(0, 0))?
        .queue(Print(format!(
            "{} {}: {}",
            player.creature.position.i, player.creature.position.j, index
        )))?;
    Ok(())
}
