use robot_simulator::*;

#[test]
fn robots_are_created_with_position_and_direction() {
    let robot = Robot::new(0, 0, Direction::North);
    assert_eq!((0, 0), robot.position());
    assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn positions_can_be_negative() {
    let robot = Robot::new(-1, -1, Direction::South);
    assert_eq!((-1, -1), robot.position());
    assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_does_not_change_position() {
    let robot = Robot::new(0, 0, Direction::North).turn_right();
    assert_eq!((0, 0), robot.position());
}

#[test]
//#[ignore]
fn turning_right_from_north_points_the_robot_east() {
    let robot = Robot::new(0, 0, Direction::North).turn_right();
    assert_eq!(&Direction::East, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_from_east_points_the_robot_south() {
    let robot = Robot::new(0, 0, Direction::East).turn_right();
    assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_from_south_points_the_robot_west() {
    let robot = Robot::new(0, 0, Direction::South).turn_right();
    assert_eq!(&Direction::West, robot.direction());
}

#[test]
//#[ignore]
fn turning_right_from_west_points_the_robot_north() {
    let robot = Robot::new(0, 0, Direction::West).turn_right();
    assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_does_not_change_position() {
    let robot = Robot::new(0, 0, Direction::North).turn_left();
    assert_eq!((0, 0), robot.position());
}

#[test]
//#[ignore]
fn turning_left_from_north_points_the_robot_west() {
    let robot = Robot::new(0, 0, Direction::North).turn_left();
    assert_eq!(&Direction::West, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_from_west_points_the_robot_south() {
    let robot = Robot::new(0, 0, Direction::West).turn_left();
    assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_from_south_points_the_robot_east() {
    let robot = Robot::new(0, 0, Direction::South).turn_left();
    assert_eq!(&Direction::East, robot.direction());
}

#[test]
//#[ignore]
fn turning_left_from_east_points_the_robot_north() {
    let robot = Robot::new(0, 0, Direction::East).turn_left();
    assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn advance_does_not_change_the_direction() {
    let robot = Robot::new(0, 0, Direction::North).advance();
    assert_eq!(&Direction::North, robot.direction());
}

#[test]
//#[ignore]
fn advance_increases_the_y_coordinate_by_one_when_facing_north() {
    let robot = Robot::new(0, 0, Direction::North).advance();
    assert_eq!((0, 1), robot.position());
}

#[test]
//#[ignore]
fn advance_decreases_the_y_coordinate_by_one_when_facing_south() {
    let robot = Robot::new(0, 0, Direction::South).advance();
    assert_eq!((0, -1), robot.position());
}

#[test]
//#[ignore]
fn advance_increases_the_x_coordinate_by_one_when_facing_east() {
    let robot = Robot::new(0, 0, Direction::East).advance();
    assert_eq!((1, 0), robot.position());
}

#[test]
//#[ignore]
fn advance_decreases_the_x_coordinate_by_one_when_facing_west() {
    let robot = Robot::new(0, 0, Direction::West).advance();
    assert_eq!((-1, 0), robot.position());
}

#[test]
//#[ignore]
fn follow_instructions_to_move_west_and_north() {
    let robot = Robot::new(0, 0, Direction::North).instructions("LAAARALA");
    assert_eq!((-4, 1), robot.position());
    assert_eq!(&Direction::West, robot.direction());
}

#[test]
//#[ignore]
fn follow_instructions_to_move_west_and_south() {
    let robot = Robot::new(2, -7, Direction::East).instructions("RRAAAAALA");
    assert_eq!((-3, -8), robot.position());
    assert_eq!(&Direction::South, robot.direction());
}

#[test]
//#[ignore]
fn follow_instructions_to_move_east_and_north() {
    let robot = Robot::new(8, 4, Direction::South).instructions("LAAARRRALLLL");
    assert_eq!((11, 5), robot.position());
    assert_eq!(&Direction::North, robot.direction());
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
  /// Starting position
  #[arg(short, long)]
  x: i32,
  
  #[arg(short, long)]
  y: i32,

  /// Faced to
  #[arg(short, long)]
  facing: char,

    /// instructions
    #[arg(short, long)]
    commands: String
}
impl Args {
    fn get_direction(&self) -> Direction {
        let c:char = self.facing.to_ascii_uppercase();
        
        match c {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            _ => Direction::North
        }
    }
}

#[test]
fn test_command_line() {
    let args = Args::parse();
    let robot = Robot::new(args.x,args.y,args.get_direction()).instructions(&args.commands);

    assert_eq!((7,9), robot.position());
    assert_eq!(&Direction::West, robot.direction());

}