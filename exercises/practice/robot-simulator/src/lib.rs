// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    pub fn turn_left(self) -> Self{
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North
        }
    }
}


#[derive(Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position{

    pub fn new(x: i32, y:i32) -> Self{
        Position {
            x:x, 
            y:y
        }
    }

    pub fn advance(self, d:Direction) -> Self {
        match d {
            Direction::North => Position::new(self.x, self.y + 1 ),
            Direction::South => Position::new(self.x, self.y - 1 ),
            Direction::East => Position::new(self.x+1, self.y),
            Direction::West => Position::new(self.x-1, self.y)
        }
    }
}

#[derive(Clone)]
pub struct Robot {
    position: Position,
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        // let pos = Position{x:x, y:y};
        Robot {
            position: Position{x:x, y:y},
            direction: d
        }
    }

    pub fn build(position: Position, direction: Direction) -> Self {
        Robot {position:position, direction:direction}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot::build(self.position, self.direction.turn_right())
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot::build(self.position, self.direction.turn_left())
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Robot::build(self.position.advance(self.direction), self.direction)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // unimplemented!("Follow the given sequence of instructions: {instructions}")

        instructions
            .chars()
            .fold(self.clone(), |robot, instruction| {
                robot.execute(instruction)
            })

        
    }

    fn execute(self, command: char) -> Self{
         match command {
            'A' => self.advance(),
            'R' => self.turn_right(),
            'L' => self.turn_left(),   
            _ => self
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

}
