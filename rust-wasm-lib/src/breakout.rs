use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Breakout {
    width: u32,
    height: u32,
    paddle_x: f64,
    paddle_width: f64,
    paddle_height: f64,
    ball_x: f64,
    ball_y: f64,
    ball_dx: f64,
    ball_dy: f64,
    ball_radius: f64,
    ctx: CanvasRenderingContext2d,
    blocks: Vec<Block>,
    rows: u32,
    columns: u32,
    is_running: bool,
    move_left: bool,
    move_right: bool,
}

struct Block {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    active: bool,
}

#[wasm_bindgen]
impl Breakout {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, rows: u32, columns: u32) -> Result<Breakout, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap().dyn_into::<HtmlCanvasElement>()?;
        let ctx = canvas.get_context("2d")?.unwrap().dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width();
        let height = canvas.height();

        let paddle_width = 75.0;
        let paddle_height = 10.0;
        let ball_radius = 5.0;

        let mut blocks = Vec::new();
        let block_width = (width as f64 - 20.0) / columns as f64;
        let block_height = 20.0;

        for row in 0..rows {
            for col in 0..columns {
                blocks.push(Block {
                    x: 10.0 + col as f64 * block_width,
                    y: 30.0 + row as f64 * (block_height + 5.0),
                    width: block_width - 5.0,
                    height: block_height,
                    active: true,
                });
            }
        }

        Ok(Breakout {
            width,
            height,
            paddle_x: (width as f64 - paddle_width) / 2.0,
            paddle_width,
            paddle_height,
            ball_x: width as f64 / 2.0,
            ball_y: height as f64 - 30.0,
            ball_dx: 2.0,
            ball_dy: -2.0,
            ball_radius,
            ctx,
            blocks,
            rows,
            columns,
            is_running: false,
            move_left: false,
            move_right: false,
        })
    }

    pub fn update(&mut self) {
        if !self.is_running {
            return;
        }

        // Update paddle position
        if self.move_left {
            self.paddle_x = (self.paddle_x - 5.0).max(0.0);
        }
        if self.move_right {
            self.paddle_x = (self.paddle_x + 5.0).min(self.width as f64 - self.paddle_width);
        }

        // Update ball position
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        // Ball collision with walls
        if self.ball_x - self.ball_radius <= 0.0 || self.ball_x + self.ball_radius >= self.width as f64 {
            self.ball_dx = -self.ball_dx;
        }
        if self.ball_y - self.ball_radius <= 0.0 {
            self.ball_dy = -self.ball_dy;
        }

        // Ball collision with paddle
        if self.ball_y + self.ball_radius >= self.height as f64 - self.paddle_height &&
           self.ball_x >= self.paddle_x && self.ball_x <= self.paddle_x + self.paddle_width {
            self.ball_dy = -self.ball_dy;
        }

        // Ball collision with blocks
        for i in 0..self.blocks.len() {
            if self.blocks[i].active && self.check_collision(&self.blocks[i]) {
                self.ball_dy = -self.ball_dy;
                self.blocks[i].active = false;
                break;
            }
        }
    }

    fn check_collision(&self, block: &Block) -> bool {
        self.ball_x + self.ball_radius > block.x &&
        self.ball_x - self.ball_radius < block.x + block.width &&
        self.ball_y + self.ball_radius > block.y &&
        self.ball_y - self.ball_radius < block.y + block.height
    }

    pub fn draw(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // Draw blocks
        for block in &self.blocks {
            if block.active {
                self.ctx.begin_path();
                self.ctx.set_fill_style(&JsValue::from_str("#0095DD"));
                self.ctx.rect(block.x, block.y, block.width, block.height);
                self.ctx.fill();
            }
        }

        // Draw paddle
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#0095DD"));
        self.ctx.rect(self.paddle_x, (self.height as f64) - self.paddle_height, self.paddle_width, self.paddle_height);
        self.ctx.fill();

        // Draw ball
        self.ctx.begin_path();
        self.ctx.set_fill_style(&JsValue::from_str("#0095DD"));
        self.ctx.arc(self.ball_x, self.ball_y, self.ball_radius, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.ctx.fill();

        // Draw "Press Space to Start" message if the game is not running
        if !self.is_running {
            self.ctx.set_font("20px Arial");
            self.ctx.set_fill_style(&JsValue::from_str("#000000"));
            self.ctx.fill_text("Press Space to Start", self.width as f64 / 2.0 - 80.0, self.height as f64 / 2.0).unwrap();
        }
    }

    pub fn move_paddle(&mut self, direction: i32) {
        let new_x = self.paddle_x + (direction as f64 * 5.0);
        if new_x >= 0.0 && (new_x + self.paddle_width) <= self.width as f64 {
            self.paddle_x = new_x;
        }
    }

    pub fn start_move(&mut self, direction: &str) {
        match direction {
            "left" => self.move_left = true,
            "right" => self.move_right = true,
            _ => {}
        }
    }

    pub fn stop_move(&mut self, direction: &str) {
        match direction {
            "left" => self.move_left = false,
            "right" => self.move_right = false,
            _ => {}
        }
    }

    pub fn toggle_game(&mut self) {
        self.is_running = !self.is_running;
        // If the game is starting, reset the ball position
        if self.is_running {
            self.ball_x = self.width as f64 / 2.0;
            self.ball_y = self.height as f64 - 30.0;
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn restart(&mut self) {
        // Reset ball position
        self.ball_x = self.width as f64 / 2.0;
        self.ball_y = self.height as f64 - 30.0;
        self.ball_dx = 2.0;
        self.ball_dy = -2.0;

        // Reset paddle position
        self.paddle_x = (self.width as f64 - self.paddle_width) / 2.0;

        // Reset blocks
        for block in &mut self.blocks {
            block.active = true;
        }

        // Stop the game
        self.is_running = false;
    }
}