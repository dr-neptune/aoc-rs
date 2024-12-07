advent_of_code::solution!(6);

use std::collections::HashSet;

struct Grid {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
    guard: Guard,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut data: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let height = data.len();
        let width = data.first().map_or(0, |row| row.len());

        // Locate '^'
        let (start_x, start_y) = data.iter().enumerate().find_map(|(row_idx, row)| {
            row.iter().position(|&c| c == '^').map(|col_idx| (row_idx, col_idx))
        }).expect("One '^' must be present");

        // Replace '^' with 'G'
        data[start_x][start_y] = 'G';

        // Facing up means decreasing x by 1 each step: Direction(-1,0)
        let guard = Guard::new(
            Position::new(start_x as i32, start_y as i32),
            Direction::new(-1, 0) // Up
        );

        Grid {
            width,
            height,
            data,
            guard,
        }
    }

    fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        let x = x as usize;
        let y = y as usize;
        x >= self.height || y >= self.width
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.data.get(x).and_then(|row| row.get(y))
    }

    fn update_guard_position(&mut self) {
        // Clear old position
        self.data[self.guard.position.x as usize][self.guard.position.y as usize] = '.';

        // Move forward
        self.guard.move_forward();

        // Place guard on new position
        self.data[self.guard.position.x as usize][self.guard.position.y as usize] = 'G';
    }

    fn display(&self) {
        for row in &self.data {
            let line: String = row.iter().collect();
            println!("{}", line);
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

struct Direction {
    dx: i8,
    dy: i8,
}

impl Direction {
    fn new(dx: i8, dy: i8) -> Self {
        Direction { dx, dy }
    }

    fn rotate_right(&self) -> Self {
        Direction {
            dx: self.dy,
            dy: -self.dx,
        }
    }
}

struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn new(position: Position, direction: Direction) -> Self {
        Guard { position, direction }
    }

    fn move_forward(&mut self) {
        self.position.x += self.direction.dx as i32;
        self.position.y += self.direction.dy as i32;
    }

    fn rotate_right(&mut self) {
        self.direction = self.direction.rotate_right();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let mut grid = Grid::new(input);

    // Track visited cells
    let mut visited = HashSet::new();
    visited.insert((grid.guard.position.x, grid.guard.position.y));

    loop {
        let current_x = grid.guard.position.x as isize;
        let current_y = grid.guard.position.y as isize;

        let next_x = current_x + grid.guard.direction.dx as isize;
        let next_y = current_y + grid.guard.direction.dy as isize;

        // Check out of bounds before moving
        if grid.out_of_bounds(next_x, next_y) {
            // Guard leaves the mapped area
            break;
        }

        let next_cell = grid.get(next_x as usize, next_y as usize).unwrap();

        if *next_cell == '#' {
            // Turn right
            grid.guard.rotate_right();
        } else {
            // '.' cell: move forward
            grid.update_guard_position();
            visited.insert((grid.guard.position.x, grid.guard.position.y));
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
