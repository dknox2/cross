use std::io::stdout;

use crossterm::{
	cursor::{Hide, MoveTo},
	event, execute,
	style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
	terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
	ExecutableCommand, QueueableCommand,
};

use crate::entity::Entity;
use crate::monster::Monster;
use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};

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

pub fn draw_monsters(monsters: &[Monster]) -> std::io::Result<()> {
	let entities = monsters.iter().map(|monster| &monster.creature_info.entity);
	for entity in entities {
		draw_entity(entity)?;
	}
	Ok(())
}

pub fn draw_entity(entity: &Entity) -> std::io::Result<()> {
	stdout()
		.queue(MoveTo(
				entity.position.x as u16,
				entity.position.y as u16
		))?
		.queue(Print(entity.glyph))?;

	Ok(())
}
