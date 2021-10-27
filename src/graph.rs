pub use ggez::{ GameResult, Context, graphics, conf, event };
use std::time::Duration;
use std::thread::sleep;
use std::sync::Mutex;
use super::*;

pub struct MainState {
    board: Board,
    time_step: Duration,
    first_frame: bool,
    pixel_size: usize,
    col1: (f32,f32,f32),
    col2: (f32,f32,f32),
    server_keys: Vec<Mutex<ServerKey>>, 
    zeros: (Ciphertext, Ciphertext, Ciphertext),
    client_key: ClientKey
}

impl MainState {
    pub fn new(board: Board, config: &Config, 
               server_keys: Vec<Mutex<ServerKey>>, zeros: (Ciphertext, Ciphertext, Ciphertext),
               client_key: ClientKey) 
        -> Result<MainState, Box<dyn std::error::Error>>
    {
        Ok(MainState { board, 
            time_step: Duration::from_micros(config.wait_time_micros), 
            first_frame: true, 
            pixel_size: config.pixel_size, 
            col1: config.col1, 
            col2: config.col2,
            server_keys, 
            zeros,
            client_key })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.first_frame {
            self.first_frame = false;
        } else {
            sleep(self.time_step);        
            self.board.update(&self.server_keys, &self.zeros);
        }
        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut Context) -> GameResult {

        // clear the window
        graphics::clear(ctx, [self.col1.0, self.col1.1, self.col1.2, 1.].into());

        for i in 0..self.board.dimensions.0 {
            for j in 0..self.board.dimensions.1 {
                if self.client_key.decrypt(&self.board.states[i*self.board.dimensions.1 + j]) {
                    let pixel = graphics::MeshBuilder::new().rectangle(
                            graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
                            graphics::Rect::new(0., 0., self.pixel_size as f32, self.pixel_size as f32),
                            graphics::Color::new(self.col2.0, self.col2.1, self.col2.2, 1.)
                        )
                        .unwrap()
                        .build(&mut ctx).unwrap();
                    graphics::draw(ctx, &pixel, graphics::DrawParam::new()
                                   .offset([-((j*self.pixel_size) as f32), -((i*self.pixel_size) as f32)]))?;
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
