extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use std::time::{Duration, Instant};
use std::collections::VecDeque;

use rand::{thread_rng, Rng};

// Set game variables
const BLOCK_SIZE: u32 = 20; //amount of pixels per in 1blocklength, eg. 10 means blocks of 10x10 pixels
const BLOCKS_HORIZONTAL: u32 = 20;
const BLOCKS_VERTICAL: u32 = 20;
const SCREEN_WIDTH: u32 = BLOCK_SIZE*BLOCKS_HORIZONTAL;
const SCREEN_HEIGHT: u32 = BLOCK_SIZE*BLOCKS_VERTICAL;

// Define some used colors
const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const RED: Color = Color::RGB(205, 0, 34);
const GREEN: Color = Color::RGB(0, 153, 51);

const SNAKE_COLOR: Color = BLACK;


//Function for getting a random block-position on the game grid
fn rand_grid_point() -> [i32;2] {
    [(thread_rng().gen::<u32>() % BLOCKS_HORIZONTAL) as i32, (thread_rng().gen::<u32>() % BLOCKS_HORIZONTAL) as i32]
}

//A Block consisting of a base rectangle and a color
struct GameBlock {
    rectangle: Rect,
    color: Color
}

impl GameBlock{
    fn draw_snake_block(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        let cornersize: u32 = 2;
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rectangle)
            .expect("failed to draw");
        canvas.set_draw_color(WHITE);

        canvas.fill_rect(Rect::new(self.rectangle.x(), self.rectangle.y(),
            cornersize, cornersize))
            .expect("failed to draw");
        canvas.fill_rect(Rect::new(self.rectangle.x() + self.rectangle.width() as i32 -cornersize as i32, self.rectangle.y(),
            cornersize, cornersize))
            .expect("failed to draw");
        canvas.fill_rect(Rect::new(self.rectangle.x(), self.rectangle.y() + self.rectangle.height() as i32 -cornersize as i32,
            cornersize, cornersize))
            .expect("failed to draw");
        canvas.fill_rect(Rect::new(self.rectangle.x() + self.rectangle.width() as i32 -cornersize as i32, self.rectangle.y() + self.rectangle.height() as i32 -cornersize as i32,
            cornersize, cornersize))
            .expect("failed to draw");
    }
    fn draw_apple(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        let cornersize: u32 = 1;
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rectangle)
            .expect("failed to draw");
        canvas.set_draw_color(GREEN);
        canvas.fill_rect(Rect::new(self.rectangle.x() + (self.rectangle.width() as i32)/2, self.rectangle.y(),
            5, 5))
            .expect("failed to draw");
        canvas.set_draw_color(WHITE);

        canvas.fill_rect(Rect::new(self.rectangle.x(), self.rectangle.y(),
            cornersize, cornersize))
            .expect("failed to draw");
        canvas.fill_rect(Rect::new(self.rectangle.x() + self.rectangle.width() as i32 -cornersize as i32, self.rectangle.y(),
            cornersize, cornersize))
            .expect("failed to draw");
        canvas.fill_rect(Rect::new(self.rectangle.x(), self.rectangle.y() + self.rectangle.height() as i32 -cornersize as i32,
            cornersize, cornersize))
            .expect("failed to draw");
        canvas.fill_rect(Rect::new(self.rectangle.x() + self.rectangle.width() as i32 -cornersize as i32, self.rectangle.y() + self.rectangle.height() as i32 -cornersize as i32,
            cornersize, cornersize))
            .expect("failed to draw");
    }
    //Return grid-coordinates of gameblock
    fn coords(&self) -> [i32;2] {
        [self.rectangle.x()/BLOCK_SIZE as i32, self.rectangle.y()/BLOCK_SIZE as i32]
    }

    fn add_snake_block(coordinates: [i32; 2])-> GameBlock {
        GameBlock {rectangle: Rect::new(coordinates[0]*BLOCK_SIZE as i32, coordinates[1] *BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE), color: SNAKE_COLOR}
    }
    fn add_apple(coordinates: [i32; 2])-> GameBlock {
        GameBlock {rectangle: Rect::new(coordinates[0]*BLOCK_SIZE as i32, coordinates[1] *BLOCK_SIZE as i32, BLOCK_SIZE, BLOCK_SIZE), color: RED}
    }

}




struct SnakePlayer {
    body_parts: VecDeque<GameBlock>,
    set_grow: bool
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl SnakePlayer {
    //Player movement done by popping tail and making new head
    fn move_player(&mut self, direction: &Direction) {
        match direction {
            Direction::UP => self.body_parts.push_front(GameBlock::add_snake_block([self.body_parts[0].coords()[0], self.body_parts[0].coords()[1] - 1])),
            Direction::DOWN => self.body_parts.push_front(GameBlock::add_snake_block([self.body_parts[0].coords()[0], self.body_parts[0].coords()[1] + 1])),
            Direction::LEFT => self.body_parts.push_front(GameBlock::add_snake_block([self.body_parts[0].coords()[0] - 1, self.body_parts[0].coords()[1]])),
            Direction::RIGHT => self.body_parts.push_front(GameBlock::add_snake_block([self.body_parts[0].coords()[0] + 1, self.body_parts[0].coords()[1]]))
        }
        // If grow is set (after eating apple), tail is not popped and snake grows by one block
        if !self.set_grow{
        self.body_parts.pop_back();
        }
    }

    fn collision(&self) -> bool {
        let mut collision: bool = false;

        //Collision with own body
        for (pos, part) in self.body_parts.iter().enumerate() { //Iterate over body parts of snake
            if pos == 0 {continue;}   //Skip head of snake because head position always == itself

            //Check if head collides with any body part except itself
            if part.coords()[0] == self.body_parts[0].coords()[0] && part.coords()[1] == self.body_parts[0].coords()[1] {
                collision = true;
                break;
            }
        }

        //Collision with wall
        if self.body_parts[0].coords()[0] < 0 as i32 || self.body_parts[0].coords()[0] >= BLOCKS_HORIZONTAL as i32
            || self.body_parts[0].coords()[1] < 0 as i32 || self.body_parts[0].coords()[1] >= BLOCKS_VERTICAL as i32 {
                collision = true;
            }
        collision
    }
    fn draw_to_window(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window> ){
        for body_part in self.body_parts.iter(){
            body_part.draw_snake_block(canvas);
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 snake-game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let event = sdl_context.event().unwrap();

    //construct player with initial direction moving to the left.
    let mut player = SnakePlayer {body_parts: VecDeque::new(), set_grow: false};  //initialize player
    let mut current_direction = Direction::LEFT;

    for i in 0..=2{    //initialize a snake with 3 bodyparts
        player.body_parts.push_back(GameBlock::add_snake_block([10+i, 5]));
    }

    //initialize first apple add random unoccupied location
    let mut apple;
    loop{
        apple = GameBlock::add_apple(rand_grid_point());
        for body_part in player.body_parts.iter(){
            if body_part.coords() == apple.coords() {
                continue;
            }
        }
        break;
    }

    //GAMELOOP
    'running: loop {
        let frametime = Instant::now();     //Record precise current time to account for fluctuation in duration of loop at end

        //Make a clear white canvas
        canvas.set_draw_color(WHITE);
        canvas.clear();

        //Input handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    match current_direction {
                        Direction::DOWN => {break},
                        _ => {current_direction = Direction::UP; break}
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    match current_direction {
                        Direction::UP => {break},
                        _ => {current_direction = Direction::DOWN; break}
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    match current_direction {
                        Direction::RIGHT => {break},
                        _ => {current_direction = Direction::LEFT; break}
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    match current_direction {
                        Direction::LEFT => {break},
                        _ => {current_direction = Direction::RIGHT; break}
                    }
                },
                _ => {}
            }
        }

        // Draw apple and snake to window
        apple.draw_apple(&mut canvas);
        player.draw_to_window(&mut canvas);

        if player.collision() {break 'running;} //Check for collision and end game if collision occurs

        if player.body_parts[0].coords() == apple.coords() {
            player.set_grow = true;     //Set player_grow to true after apple is eaten
            'occupied: loop{
                apple = GameBlock::add_apple(rand_grid_point());
                for body_part in player.body_parts.iter(){
                    if body_part.coords() == apple.coords() {
                        continue 'occupied;
                    }
                }
                break;
            }
        }

        player.move_player(&current_direction);
        canvas.present();

        player.set_grow = false;    //Set player grow to false after each frame

        std::thread::sleep(Duration::from_micros(1_000_000/(8)) - frametime.elapsed()); //Wait till frametime is reached
        event.flush_event(sdl2::event::EventType::KeyDown);     //empty the eventpump after each frame so that key inputs aren't queued up
    }
}
