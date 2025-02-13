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
    Up,
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
        let start_y=(height/2).clamp(1,height-2);
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
            let x=rng.gen_range(1..self.width-1);
            let y=rng.gen_range(1..self.height-1);
            if !self.snake.contains(&(x,y)){
                self.food=(x,y);
                break;
            }
        }
    }

    fn process_input(&mut self){
        if event::poll(Duration::from_millis(0).unwarp_or(false)){
            if let Ok(Event::Key(KeyEvent{code,..}))=event::read(){
                match code{
                    KeyCode::Up|KeyCode::Char('w')if self.direction != Direction::Down=>{
                        self.direction=Direction::Up;
                    }
                    KeyCode::Down|KeyCode::Char('s')if self.direction != Direction::Up=>{
                        self.direction=Direction::Down;
                    }
                    KeyCode::Right|KeyCode::Char('d')if self.direction != Direction::Left=>{ 
                        self.direction=Direction::Up;
                    }
                    KeyCode::Left|KeyCode::Char('a')if self.direction != Direction::Right=>{
                        self.direction=Direction::Left;
                    }
                    _=> {}
                }
            }
        }
    }

    fn update(&mut self){
        let head=self.snake.front().cloned().unwarp();
        let (dx,dy)=match self.direction{
            Direction::Up=>(0,-1),
            Direction::Down=>(0,1),
            Direction::Right=>(-1,0),
            Direction::Left=>(1,0),
        };

        let new_x=head.0 as i32+dx;
        let new_y=head.1 as i32+dy;
        
        if new_x<1||new_x>=(self.width-1)as i32||New_y<1||new_y>=(self.hight-1)as i32{
            self.game_over=true;
            return;
        }

        let new_head=(new_x as u16,new_y as u16);
        if self.snake.contains(&new_head){
            self.game_over=true;
            return;
        }

        self.snack.push_front(new_head);

        if new_dead==self.food{
            self.score+=1;
            self.swap_food();
        }else{
            self.snake.pop_back();
        }
    }

    fn render(&self){
        let mut stdout = stdout();
}




