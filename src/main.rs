use raylib::prelude::*;
use rand::prelude::*;

struct Ball {
    position: Vector2,
    movvec: Vector2,
    speed: f32,
    color: Color,
    radius: f32
}
impl Ball {
    fn mover(&mut self,width: i32, height: i32) -> bool{
        let mut deltax = self.movvec.x * self.speed;
        let mut deltay = self.movvec.y * self.speed;
        let mut count = 0;
        if self.position.x + deltax + self.radius > width as f32 {
            deltax = deltax - (self.position.x + deltax + self.radius - width as f32);
            self.movvec.x = -1f32;
            count+=1;
        } else if self.position.x + deltax - self.radius < 0.0 {
            deltax = deltax - (self.position.x + deltax - self.radius);
            self.movvec.x = 1f32;
            count+=1;
        }
        if self.position.y + deltay + self.radius > height as f32 {
            deltay = deltay - (self.position.y + deltay + self.radius - height as f32);
            self.movvec.y = -1f32;
            count+=1;
        } else if self.position.y + deltay - self.radius < 0.0 {
            deltay = deltay - (self.position.y + deltay - self.radius);
            self.movvec.y = 1f32;
            count+=1;
        }
        self.position.x += deltax;
        self.position.y += deltay;
        count == 2
    }
    fn new_basic_ball(position: Vector2 , color: Color) -> Ball {
        Ball { position, movvec: Vector2::one(), speed: 4.0, color, radius: 8.0 }
    }
}

fn main() {
    const RENDER_WIDTH: i32 = 640;
    const RENDER_HEIGHT: i32 = 480;
    let mut rng = rand::rng();

    let (mut rl, thread) = raylib::init()
        .size(RENDER_WIDTH, RENDER_HEIGHT)
        .title("Balls!")
        .build();

    let mut balls = vec![
        Ball::new_basic_ball(Vector2 {
            x: rng.random_range(1.0..RENDER_WIDTH as f32),
            y: rng.random_range(1.0..RENDER_HEIGHT as f32)
        }, Color::TOMATO)
    ];

    let colors = [Color::TOMATO, Color::GREEN, Color::BLUE, Color::YELLOW, Color::PURPLE, Color::ORANGE, Color::PINK, Color::MAGENTA];
    let mut new_balls = vec![];

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);


        for circ in balls.iter_mut() {
             if circ.mover(RENDER_WIDTH, RENDER_HEIGHT) {
                 new_balls.push(Ball::new_basic_ball(Vector2 {
                     x: rng.random_range(1.0..RENDER_WIDTH as f32),
                     y: rng.random_range(1.0..RENDER_HEIGHT as f32)
                 }, *colors.choose(&mut rng).unwrap()));
             }
            d.draw_circle(circ.position.x as i32, circ.position.y as i32, circ.radius, circ.color);
        }
        if !new_balls.is_empty() {
            balls.append(&mut new_balls);
            new_balls = vec![];
        }

        d.clear_background(Color::WHITE);
        d.draw_text(&format!("Ball count: {}", balls.len()), 12, 12, 20, Color::BLACK);
    }
}
