use std::io::stdout;

use crossterm::{
	cursor::{Hide, MoveTo},
	style::Print,
	terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
	ExecutableCommand, QueueableCommand,
};

use crate::entity::Entity;
use crate::game::Game;
use crate::map::{Map, TileType, MAP_HEIGHT, MAP_WIDTH};
use crate::monster::Monster;

pub fn setup_terminal() -> std::io::Result<()> {
	terminal::enable_raw_mode()?;
	stdout()
		.execute(EnterAlternateScreen)?
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
		.queue(MoveTo(entity.position.x as u16, entity.position.y as u16))?
		.queue(Print(entity.glyph))?;

	Ok(())
}

pub fn draw_hud(game: &Game) -> std::io::Result<()> {
	stdout()
		.queue(MoveTo(1, MAP_HEIGHT as u16 + 1))?
		.queue(Print(format!("HP: {}({})", game.player.creature_info.health, game.player.creature_info.max_health)))?;

	Ok(())
}

pub fn draw_game_over_screen() -> std::io::Result<()> {
	stdout()
		.queue(Clear(ClearType::All))?
		.queue(MoveTo(1, 1))?
		.queue(Print("The dungeon claims another..."))?
		.queue(MoveTo(1, 2))?
		.queue(Print("Press Enter to continue."))?;

	Ok(())
}
