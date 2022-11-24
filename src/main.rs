mod game;

use crate::game::Controller;

fn main() -> Result<(), String> {
    let mut game = Controller::new(12345);

    game.start()?;

    Ok(())
}
