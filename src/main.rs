use crossterm::ExecutableCommand;
use crossterm::{
    cursor::{self, MoveTo},
    event::{poll, read, Event, KeyCode}, // Ensure event handling items are imported
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType}, // Include Clear here
};
use rand::{thread_rng, Rng};
use std::{
    io::{stdout, Result},
    thread,
    time::Duration,
};

struct Player {
    health: i32,
    x_position: u16,
    y_position: u16,
    sprite: String,
}

fn main() -> Result<()> {
    // Setup terminal
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::Hide)?;
    enable_raw_mode()?;
    let mut game_over = false;
    let (width, height) = terminal::size()?;
    let container_width = width; // Use full width
    let container_height = height - 1; // Adjust for 0 indexing, use full height

    let mut bananas = vec![]; // Vec<(x, y)>
    let mut rng = thread_rng();
    let frame_time = Duration::from_millis(50);

    let mut player = Player {
        health: 5,
        x_position: width / 2,
        y_position: height - 2, 
        sprite: "🐱".to_string(),
    };

    while !game_over {

        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(MoveTo(0, 0))?;

        // Print the player's health
        println!("Player Health: {}", player.health);

        if poll(Duration::from_millis(0))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break, // Quit the game
                    KeyCode::Left => {
                        if player.x_position > 1 {
                            player.x_position -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if player.x_position < width {
                            player.x_position += 1;
                        }
                    }
                    KeyCode::Up => {
                        if player.y_position > 0 {
                            player.y_position -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if player.y_position < height - 1 {
                            player.y_position += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Clear the terminal or container area and set background to black
        execute!(
            stdout,
            SetBackgroundColor(Color::Black),
            Clear(ClearType::All)
        )?;

        // Add a new banana at a random position at the top
        if rng.gen_bool(0.5) {
            let banana_x = rng.gen_range(0..container_width);
            bananas.push((banana_x, 0)); // Always start at the top
        }

        // Move each banana down by 1
        for banana in bananas.iter_mut() {
            banana.1 += 1;
        }
        // Game logic here...

        for (_, &(banana_x, banana_y)) in bananas.iter().enumerate() {
            if banana_x == player.x_position && banana_y == player.y_position {
                // Collision detected
                player.health -= 1;
                if player.health <= 0 {
                    // Handle game over condition
                    println!("Game Over!");
                    game_over = true;
                    break;
                }
            }
        }

        if game_over {
            break;
        }

        bananas.retain(|&(banana_x, banana_y)| {
            banana_x != player.x_position || banana_y != player.y_position
        });
        // Draw all bananas
        for &(banana_x, banana_y) in &bananas {
            if banana_y < container_height {
                execute!(
                    stdout,
                    MoveTo(banana_x, banana_y),
                    SetForegroundColor(Color::Yellow),
                    Print("🍌"),
                    ResetColor // Corrected usage
                )?;
            }
        }

        // Remove bananas that have reached the bottom
        bananas.retain(|&(_, y)| y < container_height);

        // Draw the player
        execute!(
            stdout,
            MoveTo(player.x_position, player.y_position),
            SetForegroundColor(Color::White),
            Print(&player.sprite),
            ResetColor,
        )?;


        thread::sleep(frame_time);
    }
    // Cleanup
    execute!(stdout, SetBackgroundColor(Color::Reset), cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}
