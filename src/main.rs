use rand::RngExt;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};
const X_BORDER: u16 = 58;
const Y_BORDER: u16 = 24;

struct Enemy {
    x: u16,
    y: u16,
    alive: bool,
}

impl Enemy {
    pub fn new(enemies: &mut Vec<Enemy>) -> Self {
        let positions = get_new_random_position(enemies);
        Enemy {
            alive: true,
            x: positions.0,
            y: positions.1,
        }
    }

    fn draw(&self, f: &mut ratatui::Frame<'_>) {
        f.buffer_mut()
            .set_string(self.x, self.y, "+", Style::default().fg(Color::Red));
    }
}

fn get_new_random_position(enemies: &Vec<Enemy>) -> (u16, u16) {
    let mut rng = rand::rng();

    let x_pos = rng.random_range(1..X_BORDER - 3);
    let y_pos = rng.random_range(1..Y_BORDER - 3);
    // give a position that would not collide with other enemies.
    for i in 0..enemies.len() {
        if x_pos == enemies[i].x && y_pos == enemies[i].y {
            return get_new_random_position(enemies);
        }
    }
    (x_pos, y_pos)
}

struct Player {
    game_over: bool,
    points: usize,
    x: u16,
    y: u16,
}

impl Player {
    pub fn new(enemies: &Vec<Enemy>) -> Self {
        let positions = get_new_random_position(enemies);
        Player {
            game_over: false,
            points: 0,
            x: positions.0,
            y: positions.1,
        }
    }

    fn draw(&self, f: &mut ratatui::Frame<'_>) {
        f.buffer_mut()
            .set_string(self.x, self.y, "@", Style::default().fg(Color::Yellow));
    }

    fn teleport(&mut self, enemies: &mut Vec<Enemy>) {
        let positions = get_new_random_position(enemies);

        self.x = positions.0;
        self.y = positions.1;
    }

    fn move_enemies(&mut self, enemies: &mut Vec<Enemy>) {
        // check if enemies has eaten the player
        for enemy in &mut *enemies {
            if enemy.y < self.y && enemy.y <= Y_BORDER - 3 {
                enemy.y += 1;
            }
            if enemy.y > self.y && enemy.y >= 2 {
                enemy.y -= 1;
            }
            if enemy.x < self.x && enemy.x <= X_BORDER - 3 {
                enemy.x += 1;
            }
            if enemy.x > self.x && enemy.x >= 2 {
                enemy.x -= 1;
            }

            if enemy.y == self.y && enemy.x == self.x {
                self.game_over = true;
            }
        }
    }

    fn movement(&mut self, enemies: &mut Vec<Enemy>) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('w') => {
                    if self.y >= 2 {
                        self.y -= 1
                    }
                    self.move_enemies(enemies);
                }
                KeyCode::Char('a') => {
                    if self.x >= 2 {
                        self.x -= 1
                    }
                    self.move_enemies(enemies);
                }
                KeyCode::Char('s') => {
                    if self.y <= Y_BORDER - 3 {
                        self.y += 1
                    }
                    self.move_enemies(enemies);
                }
                KeyCode::Char('d') => {
                    if self.x <= X_BORDER - 3 {
                        self.x += 1
                    }
                    self.move_enemies(enemies);
                }
                KeyCode::Char('t') => {
                    self.teleport(enemies);
                }
                _ => {}
            }

            // check if enemies have crashed in each other
            for i in 0..enemies.len() {
                for j in 0..enemies.len() {
                    if i != j && enemies[i].y == enemies[j].y && enemies[i].x == enemies[j].x {
                        enemies[i].alive = false;
                        enemies[j].alive = false;
                        break;
                    }
                }
            }

            for i in (0..enemies.len()).rev() {
                if !enemies[i].alive {
                    self.points += 2;
                    enemies.remove(i);
                }
            }

            //if no enemies are left, add 10 more.
            if enemies.is_empty() {
                for _ in 0..10 {
                    let en = Enemy::new(enemies);
                    enemies.push(en);
                }
            }
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let mut enemies: Vec<Enemy> = Vec::with_capacity(10);
    for _ in 0..10 {
        let en = Enemy::new(&mut enemies);
        enemies.push(en);
    }

    let mut player = Player::new(&enemies);

    loop {
        terminal.draw(|f| {
            let frame_area = Rect::new(0, 0, X_BORDER, Y_BORDER);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow));

            f.render_widget(block, frame_area);

            for enemy in &enemies {
                enemy.draw(f);
            }
            player.draw(f);

            let _ = player.movement(&mut enemies);
        })?;

        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
            && key.code == KeyCode::Char('q')
        {
            break;
        }

        if player.game_over {
            break;
        }
    }

    ratatui::restore();
    println!("Game over. Your result was: {}. ", player.points);
    Ok(())
}
