            match key.code {
                keycode::char('w') => {
                    if self.y >= 2 {
                        self.y -= 1
                    }
                    self.move_enemies(enemies);
                }
                keycode::char('a') => {
                    if self.x >= 2 {
                        self.x -= 1
                    }
                    self.move_enemies(enemies);
                }
                keycode::char('s') => {
                    if self.y <= y_border - 3 {
                        self.y += 1
                    }
                    self.move_enemies(enemies);
                }
                keycode::char('d') => {
                    if self.x <= x_border - 3 {
                        self.x += 1
                    }
                    self.move_enemies(enemies);
                }
                keycode::char('t') => {
                    self.teleport(enemies);
                }

            }
