// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::East,
                }
            }
            Direction::South => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::West,
                }
            }
            Direction::East => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::South,
                }
            }
            Direction::West => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::North,
                }
            }
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::West,
                }
            }
            Direction::South => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::East,
                }
            }
            Direction::East => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::North,
                }
            }
            Direction::West => {
                return Robot {
                    x: self.x,
                    y: self.y,
                    direction: Direction::South,
                }
            }
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::East => {
                let mut x = self.x;
                x += 1;
                return Robot { x: x, y: self.y, direction: self.direction }
            },
            Direction::West => {
                let mut x = self.x;
                x -= 1;
                return Robot { x: x, y: self.y, direction: self.direction }
            },
            Direction::North => {
                let mut y = self.y;
                y += 1;
                return Robot { x: self.x, y: y, direction: self.direction }
            },
            Direction::South => {
                let mut y = self.y;
                y -= 1;
                return Robot { x: self.x, y: y, direction: self.direction }
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = Robot {x: self.x, y: self.y, direction: self.direction.clone() };
        for command in instructions.as_bytes().iter() {

            if *command == b'L' {
                robot = robot.turn_left();
            }

            if *command == b'R' {
                robot = robot.turn_right();
            }

            if *command == b'A' {
                robot = robot.advance();
            }
        }

        return robot;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
