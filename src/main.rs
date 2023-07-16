use std::{
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::Event,
    event::{self, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, Result,
};
use pac_man::{
    direction::Direction,
    frame::{new_frame, Drawable},
    pac_man::PacMan,
    render, map::level_1,
};

fn main() -> Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let last_frame = new_frame();
        let mut stdout = std::io::stdout();
        render::render(&mut stdout, &last_frame, true);

        loop {
            let cur_frame = match rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &cur_frame, false);
        }
    });

    let mut pac_man = PacMan::new();
    let mut instant = Instant::now();
    

    'gameLoop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => pac_man.update_direction(Direction::Up),
                    KeyCode::Down => pac_man.update_direction(Direction::Down),
                    KeyCode::Left => pac_man.update_direction(Direction::Left),
                    KeyCode::Right => pac_man.update_direction(Direction::Right),
                    KeyCode::Char('q') | KeyCode::Esc => break 'gameLoop,
                    _ => {}
                }
            }
        }

        level_1(&mut frame);


        pac_man.update_character(delta);
        pac_man.update_position(delta);

        // draw
        pac_man.draw(&mut frame);
        // render
        let _ = tx.send(frame);
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;

    Ok(())
}
