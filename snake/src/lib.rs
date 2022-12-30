use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_compatible_with(&self, other: Direction) -> bool {
        match (self, other) {
            (Self::Up, Self::Down) => false,
            (Self::Down, Self::Up) => false,
            (Self::Left, Self::Right) => false,
            (Self::Right, Self::Left) => false,
            _ => true,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

struct Snake {
    head: Point,
    body: Vec<Point>,
    index: usize,
}

impl Snake {
    pub fn r#move(
        &mut self,
        direction: Direction,
        should_add_new_body_piece: bool,
        w: usize,
        h: usize,
    ) -> Point {
        self.move_body(should_add_new_body_piece);
        self.move_head(direction, w, h);

        self.head
    }

    pub fn contains(&self, point: Point) -> bool {
        self.head == point || self.body.contains(&point)
    }

    pub fn on_body(&self, point: Point) -> bool {
        self.body.contains(&point)
    }

    fn move_body(&mut self, should_add_new_body_piece: bool) {
        if should_add_new_body_piece {
            self.body.push(self.body[self.index]);
        }

        let index_to_move = (self.index + 1) % self.body.len();

        self.body[index_to_move] = self.head;

        self.index = index_to_move;
    }

    fn move_head(&mut self, direction: Direction, w: usize, h: usize) {
        match direction {
            Direction::Up => self.head.y = (self.head.y + 1) % h,
            Direction::Down => {
                self.head.y = if self.head.y == 0 {
                    h - 1
                } else {
                    self.head.y - 1
                }
            }
            Direction::Right => self.head.x = (self.head.x + 1) % w,
            Direction::Left => {
                self.head.x = if self.head.x == 0 {
                    w - 1
                } else {
                    self.head.x - 1
                }
            }
        };
    }
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub on_food: bool,
    pub on_wall: bool,
    pub eat_itself: bool,
    pub food_position: Point,
    pub snake: Vec<Point>,
}

pub struct Game {
    width: usize,
    height: usize,
    snake: Snake,
    board: Vec<Cell>,
    food: Point,
    previous_direction: Direction,
    new_piece_to_generate: usize,
    last_snapshot: Snapshot,
}

impl Game {
    pub fn tick(&mut self, mut direction: Direction) {
        if !direction.is_compatible_with(self.previous_direction) {
            direction = self.previous_direction;
        }
        self.previous_direction = direction;

        let should_add_new_body_piece = self.new_piece_to_generate > 0;
        if should_add_new_body_piece {
            self.new_piece_to_generate -= 1;
        }

        let head = self.snake.r#move(
            self.previous_direction,
            should_add_new_body_piece,
            self.width,
            self.height,
        );

        let index = coordinate_to_index(head, self.width);

        let on_wall = self.board[index] == Cell::Wall;
        let on_food = head == self.food;

        let eat_itself = self.snake.on_body(head);

        if on_food {
            self.new_piece_to_generate += 1;
            self.food = self.generate_new_food_position();
        }

        let mut snake = self.snake.body.clone();
        snake.push(self.snake.head);

        self.last_snapshot = Snapshot {
            on_food,
            on_wall,
            eat_itself,
            food_position: self.food,
            snake,
        };
    }

    pub fn last_snapshot(&self) -> Snapshot {
        self.last_snapshot.clone()
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn walls(&self) -> Vec<Point> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, c)| Cell::Wall == **c)
            .map(|(i, _)| index_to_coordinate(i, self.width))
            .collect()
    }

    fn generate_new_food_position(&self) -> Point {
        let x = fastrand::usize(..(self.width as usize));
        let y = fastrand::usize(..(self.height as usize));
        let p = Point { x, y };

        if self.board[coordinate_to_index(p, self.width)] == Cell::Wall {
            return self.generate_new_food_position();
        }

        if self.snake.contains(p) {
            return self.generate_new_food_position();
        }

        p
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h = s.lines().count();
        let w = s.lines().next().unwrap().len();

        let chars = s.lines().rev().flat_map(|l| l.chars());

        let mut snake_head = None;
        let mut snake_body = vec![];
        let mut food = None;
        let mut board = Vec::with_capacity(h as usize * w as usize);
        for (i, c) in chars.enumerate() {
            match c {
                '#' => {
                    board.push(Cell::Wall);
                }
                ' ' => {
                    board.push(Cell::Empty);
                }
                'h' => {
                    snake_head = Some(index_to_coordinate(i, w));
                    board.push(Cell::Empty);
                }
                'b' => {
                    snake_body.push(index_to_coordinate(i, w));
                    board.push(Cell::Empty);
                }
                'f' => {
                    food = Some(index_to_coordinate(i, w));
                    board.push(Cell::Empty);
                }
                _ => return Err(format!("Invalid char {} at {}", c, i)),
            }
        }

        let mut snake = snake_body.clone();
        snake.insert(0, snake_head.unwrap());

        Ok(Game {
            width: w,
            height: h,
            snake: Snake {
                head: snake_head.unwrap(),
                body: snake_body,
                index: 0,
            },
            board,
            food: food.unwrap(),
            previous_direction: Direction::Up,
            new_piece_to_generate: 0,
            last_snapshot: Snapshot {
                on_food: false,
                on_wall: false,
                eat_itself: false,
                food_position: food.unwrap(),
                snake,
            },
        })
    }
}

fn index_to_coordinate(index: usize, w: usize) -> Point {
    Point {
        x: index % w,
        y: index / w,
    }
}

fn coordinate_to_index(p: Point, w: usize) -> usize {
    p.y * w + p.x
}

pub static SNAKE_2: &'static str = "          
          
    h     
    b     
      f   
          ";
pub static SNAKE_1: &'static str = "\
##########
#        #
#        #
#   h    #
#   b f  #
##########";

#[cfg(test)]
mod tests {
    use crate::{Direction, Game, Point, SNAKE_2};

    static FIRST_LEVEL: &'static str = "\
##########
#        #
#   h    #
#   b    #
#     f  #
##########";

    #[test]
    fn test_from_str() {
        let game: Game = FIRST_LEVEL.parse().unwrap();

        assert_eq!(game.snake.head, Point { x: 4, y: 3 });
        assert_eq!(game.snake.body, vec![Point { x: 4, y: 2 }]);
        assert_eq!(game.food, Point { x: 6, y: 1 });
    }

    #[test]
    fn test_tick_up() {
        let mut game: Game = FIRST_LEVEL.parse().unwrap();

        game.tick(crate::Direction::Up);

        assert_eq!(game.snake.head, Point { x: 4, y: 4 });
        assert_eq!(game.snake.body, vec![Point { x: 4, y: 3 }]);
        assert_eq!(game.food, Point { x: 6, y: 1 });
    }

    #[test]
    fn test_tick_all_direction() {
        let mut game: Game = FIRST_LEVEL.parse().unwrap();

        game.tick(crate::Direction::Up);

        assert_eq!(game.snake.head, Point { x: 4, y: 4 });
        assert_eq!(game.snake.body, vec![Point { x: 4, y: 3 }]);

        game.tick(crate::Direction::Up);

        assert_eq!(game.snake.head, Point { x: 4, y: 5 });
        assert_eq!(game.snake.body, vec![Point { x: 4, y: 4 }]);

        game.tick(crate::Direction::Left);

        assert_eq!(game.snake.head, Point { x: 3, y: 5 });
        assert_eq!(game.snake.body, vec![Point { x: 4, y: 5 }]);

        game.tick(crate::Direction::Left);

        assert_eq!(game.snake.head, Point { x: 2, y: 5 });
        assert_eq!(game.snake.body, vec![Point { x: 3, y: 5 }]);

        game.tick(crate::Direction::Down);

        assert_eq!(game.snake.head, Point { x: 2, y: 4 });
        assert_eq!(game.snake.body, vec![Point { x: 2, y: 5 }]);

        game.tick(crate::Direction::Down);

        assert_eq!(game.snake.head, Point { x: 2, y: 3 });
        assert_eq!(game.snake.body, vec![Point { x: 2, y: 4 }]);

        game.tick(crate::Direction::Right);

        assert_eq!(game.snake.head, Point { x: 3, y: 3 });
        assert_eq!(game.snake.body, vec![Point { x: 2, y: 3 }]);

        game.tick(crate::Direction::Right);

        assert_eq!(game.snake.head, Point { x: 4, y: 3 });
        assert_eq!(game.snake.body, vec![Point { x: 3, y: 3 }]);
    }

    #[test]
    fn test_tick_with_long_snake() {
        let board = "\
##########
#   bbbh #
#   b    #
#   b    #
#     f  #
##########";
        let mut game: Game = board.parse().unwrap();
        game.previous_direction = Direction::Right;

        assert_eq!(game.snake.head, Point { x: 7, y: 4 });
        assert_eq!(
            game.snake.body,
            vec![
                Point { x: 4, y: 2 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 4 },
                Point { x: 5, y: 4 },
                Point { x: 6, y: 4 }
            ]
        );

        game.tick(crate::Direction::Down);

        assert_eq!(game.snake.head, Point { x: 7, y: 3 });
        assert_eq!(
            game.snake.body,
            vec![
                Point { x: 4, y: 2 },
                Point { x: 7, y: 4 },
                Point { x: 4, y: 4 },
                Point { x: 5, y: 4 },
                Point { x: 6, y: 4 }
            ]
        );

        game.tick(crate::Direction::Down);

        assert_eq!(game.snake.head, Point { x: 7, y: 2 });
        assert_eq!(
            game.snake.body,
            vec![
                Point { x: 4, y: 2 },
                Point { x: 7, y: 4 },
                Point { x: 7, y: 3 },
                Point { x: 5, y: 4 },
                Point { x: 6, y: 4 }
            ]
        );
    }

    #[test]
    fn test_tick_eat_food_and_generate_new_piece() {
        let board = "\
##########
#        #
#   f    #
#   h    #
#   b    #
##########";
        let mut game: Game = board.parse().unwrap();

        assert_eq!(game.food, Point { x: 4, y: 3 });

        game.tick(crate::Direction::Up);

        assert_eq!(game.snake.head, Point { x: 4, y: 3 });
        assert_eq!(game.snake.body.len(), 1);
        assert_ne!(game.food, Point { x: 4, y: 3 });

        game.tick(crate::Direction::Up);

        assert_eq!(game.snake.head, Point { x: 4, y: 4 });
        assert_eq!(
            game.snake.body,
            vec![Point { x: 4, y: 2 }, Point { x: 4, y: 3 }]
        );

        game.tick(crate::Direction::Up);

        assert_eq!(game.snake.head, Point { x: 4, y: 5 });
        assert_eq!(
            game.snake.body,
            vec![Point { x: 4, y: 4 }, Point { x: 4, y: 3 }]
        );

        game.tick(crate::Direction::Right);

        assert_eq!(game.snake.head, Point { x: 5, y: 5 });
        assert_eq!(
            game.snake.body,
            vec![Point { x: 4, y: 4 }, Point { x: 4, y: 5 }]
        );

        game.tick(crate::Direction::Right);

        assert_eq!(game.snake.head, Point { x: 6, y: 5 });
        assert_eq!(
            game.snake.body,
            vec![Point { x: 5, y: 5 }, Point { x: 4, y: 5 }]
        );

        game.food = Point { x: 7, y: 5 };

        game.tick(crate::Direction::Right);

        assert_eq!(game.snake.head, Point { x: 7, y: 5 });
        assert_eq!(
            game.snake.body,
            vec![Point { x: 5, y: 5 }, Point { x: 6, y: 5 }]
        );

        game.tick(crate::Direction::Right);

        assert_eq!(game.snake.head, Point { x: 8, y: 5 });
        assert_eq!(
            game.snake.body,
            vec![
                Point { x: 5, y: 5 },
                Point { x: 6, y: 5 },
                Point { x: 7, y: 5 }
            ]
        );

        game.tick(crate::Direction::Down);

        assert_eq!(game.snake.head, Point { x: 8, y: 4 });
        assert_eq!(
            game.snake.body,
            vec![
                Point { x: 8, y: 5 },
                Point { x: 6, y: 5 },
                Point { x: 7, y: 5 }
            ]
        );

        game.tick(crate::Direction::Down);

        assert_eq!(game.snake.head, Point { x: 8, y: 3 });
        assert_eq!(
            game.snake.body,
            vec![
                Point { x: 8, y: 5 },
                Point { x: 8, y: 4 },
                Point { x: 7, y: 5 }
            ]
        );
    }

    #[test]
    fn test_snake_2() {
        let mut game: Game = SNAKE_2.parse().unwrap();

        game.tick(Direction::Left);
        game.tick(Direction::Left);
        game.tick(Direction::Left);
        game.tick(Direction::Left);

        // Right -> Left
        assert_eq!(game.snake.head, Point { x: 0, y: 3 });
        game.tick(Direction::Left);
        assert_eq!(game.snake.head, Point { x: 9, y: 3 });

        game.tick(Direction::Up);

        // Left -> Right
        assert_eq!(game.snake.head, Point { x: 9, y: 4 });
        game.tick(Direction::Right);
        assert_eq!(game.snake.head, Point { x: 0, y: 4 });

        game.tick(Direction::Right);

        game.tick(Direction::Up);

        // Top -> Bottom
        assert_eq!(game.snake.head, Point { x: 1, y: 5 });
        game.tick(Direction::Up);
        assert_eq!(game.snake.head, Point { x: 1, y: 0 });

        game.tick(Direction::Right);

        // Top -> Bottom
        assert_eq!(game.snake.head, Point { x: 2, y: 0 });
        game.tick(Direction::Down);
        assert_eq!(game.snake.head, Point { x: 2, y: 5 });
    }

    #[test]
    fn test_box_leak() {
        let game_handler = {
            let game: Game = SNAKE_2.parse().unwrap();

            let game = Box::new(game);

            let game = Box::leak(game);
            let game = game as *const Game;
            game as usize
        };

        let mut game: Box<Game> = unsafe {
            let game = game_handler as *mut Game;
            Box::from_raw(game)
        };

        game.tick(Direction::Up);
    }
}
