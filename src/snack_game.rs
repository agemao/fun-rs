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
            food:(0,0),
            score:0,
            game_over:false,
            width,
            height,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self){
        let mut rng=rand::rng();
        loop{
            let x=rng.random_range(1..self.width-1);
            let y=rng.random_range(1..self.height-1);
            if !self.snake.contains(&(x,y)){
                self.food=(x,y);
                break;
            }
        }
    }

    fn process_input(&mut self){
        if let Ok(true)=event::poll(Duration::from_millis(0)){
            if let Ok(Event::Key(KeyEvent{code,..}))=event::read(){
                match code{
                    KeyCode::Up|KeyCode::Char('w')if self.direction != Direction::Down=>{
                        self.direction=Direction::Up;
                    }
                    KeyCode::Down|KeyCode::Char('s')if self.direction != Direction::Up=>{
                        self.direction=Direction::Down;
                    }
                    KeyCode::Right|KeyCode::Char('d')if self.direction != Direction::Left=>{ 
                        self.direction=Direction::Right;
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
        let head=self.snake.front().cloned().unwrap();
        let (dx,dy)=match self.direction{
            Direction::Up=>(0,-1),
            Direction::Down=>(0,1),
            Direction::Right=>(1,0),
            Direction::Left=>(-1,0),
        };

        let new_x=head.0 as i32+dx;
        let new_y=head.1 as i32+dy;
        
        if new_x<1||new_x>=(self.width-1)as i32||new_y<1||new_y>=(self.height-1)as i32{
            self.game_over=true;
            return;
        }

        let new_head=(new_x as u16,new_y as u16);
        //if self.snake.contains(&new_head){
        //    self.game_over=true;
        //    return;
        //}

        self.snake.push_front(new_head);

        if new_head==self.food{
            self.score+=1;
            self.spawn_food();
        }else{
            self.snake.pop_back();
        }
    }

    fn render(&self){
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();

        for y in 0..self.height{
            for x in 0..self.width{
                let char=if x==0||x==self.width-1||y==0||y==self.height-1{
                    '#'
                }else if self.snake.contains(&(x,y)){
                    '■'
                }else if (x,y)==self.food{
                    '★'
                }else{
                    ' '
                };
                execute!(
                    stdout,
                    crossterm::cursor::MoveTo(x,y),
                    Print(char)
                    ).unwrap();
            }
        }
        execute!(
            stdout,
            crossterm::cursor::MoveTo(0,self.height),
            Print(format!("Score:{}",self.score))
        ).unwrap();

        if self.game_over{
            execute!(
                stdout,
                crossterm::cursor::MoveTo(self.width/2-5,self.height/2),
                Print("Game Over!"),
                crossterm::cursor::MoveTo(self.width/2-10,self.height/2+1),
                Print("press any key to exit...")
            ).unwrap();

            stdout.flush().unwrap();
        }
    }
}

pub fn test()->Result<(),Box<dyn std::error::Error>>{
    enable_raw_mode()?;
    let mut stdout=stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Hide
        )?;

    let (width,height)=crossterm::terminal::size()?;
    let mut game=Game::new(width,height);

    let mut last_update=Instant::now();
    let update_interval=Duration::from_millis(500);

    while!game.game_over{
        game.process_input();
        if last_update.elapsed()>=update_interval{
            game.update();
            last_update=Instant::now();
        }
        game.render();
        std::thread::sleep(Duration::from_millis(10));
    }
    
    while!event::poll(Duration::from_secs(1))?{}
    let _=event::read()?;

    execute!(
        stdout,
        LeaveAlternateScreen,
        crossterm::cursor::Show
        )?;
    disable_raw_mode()?;
    Ok(())
}




