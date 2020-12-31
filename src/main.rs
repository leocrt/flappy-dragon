use bracket_lib::prelude::*;
use std::{thread, time};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 =5.0;

enum GameMode{
    Menu,
    Playing,
    End
}

struct Player{
    x: i32,
    y: i32, 
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Self{
        Player{
            x: x,
            y: y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm){
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        );
    }
    fn gravity_and_move(&mut self){
        if self.velocity < 2.0{
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0{
            self.y = 0;
        }
    }
    fn flap(&mut self){
        self.velocity = -2.0
    }
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode
}

impl State {
    fn new() -> Self{
        State{
            player:Player::new(5,25),
            frame_time: 0.0,
            mode: GameMode::Menu
        }
    }
    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION{
            self.frame_time = 0.0;
            self.player.gravity_and_move()
        }
        if let Some(VirtualKeyCode::Space) = ctx.key{
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0,0,"Press space to flap");
        if self.player.y > SCREEN_HEIGHT{
            self.mode = GameMode::End;
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to flappy dragon!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key{
            match key{
                VirtualKeyCode::P => self.play(ctx),
                VirtualKeyCode::Q => self.quit_game(ctx),
                _=>{}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key{
            match key{
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => self.quit_game(ctx),
                _=>{}
            }
        }
    }
    fn restart(&mut self){
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn quit_game(&mut self, ctx: &mut BTerm){
        ctx.print_centered(5, "Quiting Game...");
        std::thread::sleep(time::Duration::from_secs(5));
        ctx.quitting = true;
    }
}

impl GameState for State{
    fn tick(&mut self, ctx: &mut BTerm){
        match self.mode{
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy dragon")
    .build()?;

    main_loop(context, State::new())
}


