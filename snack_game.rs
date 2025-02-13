use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    cursor::Hide,
    style::Print,
    terminal::{
        Clear,
        ClearType
    }
};
use rand::Rng;
use std::{
    collections::VecDeque,
    io::{
        stdout,
        Write
    },
    time::{
        Duration,
        Instant
    },
};

#[derive(PartialEq, Clone, Copy)]
enum Direction{
    up,
    Down,
    Left,
    Right,
}

struct Game{
    snake: VecDeque<(u16,u16)>,
    direction: Direction,
    food: (u16,u16),
    score: u32,
    game_over: bool,
    width: u16,
    height: u16,
}

impl Game{
    fn new(width: u16,height: u16) -> Self{
        let mut snake=VecDeque::new();
        let start_x=(width/2).clamp(1,width-2);
        let start_y=(hight/2).clamp(1,hight-2);
        snake.push_back((start_x,start_y));
        snake.push_back((start_x-1,start_y));
        snake.push_back((start_x-2,start_y));

        let mut game=Game{
            snake,
            direction: Direction::Right,
            food: (0,0),
            score: 0,
            game_over: false,
            width,
            height,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self){
        let mut rng=rand::thread_rng();
        loop{
            let x=rng.gen_range




