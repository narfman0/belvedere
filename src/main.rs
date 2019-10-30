use ggez;

use ggez::{event, GameResult};
use std::{env, path};

mod entity;
mod states;
mod world;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("belvedere", "narfman0").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut states::LevelState::new(ctx)?;
    event::run(ctx, event_loop, state)
}