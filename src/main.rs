use std::{
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant}, collections::HashMap, borrow::BorrowMut, array::from_mut,
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
    render,
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut level: i8 = 1;
    let mut visited_map: HashMap<String, bool>= HashMap::new();

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut last_frame = new_frame(level, &HashMap::new());
        let mut stdout = std::io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true, 0);

        loop {
            let (cur_frame, score) = match rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &cur_frame, false, score);
            last_frame = cur_frame;
        }
    });

    let mut pac_man = PacMan::new();
    let mut instant = Instant::now();

    'gameLoop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut frame = new_frame(level,  &visited_map);

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

        pac_man.update_character(delta);
        pac_man.update_position(delta, &frame, &mut visited_map);

        // draw
        pac_man.draw(&mut frame);
        // render
        let _ = tx.send((frame, visited_map.len()));
    }

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;

    Ok(())
}
