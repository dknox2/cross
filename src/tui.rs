use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo},
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
    ExecutableCommand, QueueableCommand,
};

use crate::creature::Creature;
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

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let index = map.coordinates_to_index(x as i32, y as i32);
            if map.tiles[index] == TileType::Wall {
                stdout()
                    .queue(MoveTo(x as u16, y as u16))?
                    .queue(Print('█'))?;
            }
        }
    }

    Ok(())
}

pub fn draw_player(player: &Player) -> std::io::Result<()> {
    let index = player.creature.position.x * 60 + player.creature.position.y;
    stdout()
        .queue(MoveTo(
            player.creature.position.x as u16,
            player.creature.position.y as u16,
        ))?
        .queue(Print('@'))?
        .queue(MoveTo(0, 0))?
        .queue(Print(format!(
            "{} {}: {}",
            player.creature.position.x, player.creature.position.y, index
        )))?;
    Ok(())
}

pub fn draw_monsters(monsters: &[Creature]) -> std::io::Result<()> {
    for monster in monsters {
        stdout()
            .queue(MoveTo(monster.position.x as u16, monster.position.y as u16))?
            .queue(Print('g'))?;
    }

    Ok(())
}
