use ggez::{Context, GameResult};
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, Mesh, DrawParam};

const LAUNCH_X: f32 = 100.0; // Coordenada x da área de lançamento
const LANDING_X: f32 = 500.0; // Coordenada x da área de pouso (aumentada para 500.0)
const Y_LEVEL: f32 = 500.0; // Mesma coordenada y para área de lançamento e área de pouso
const GROUND_HEIGHT: f32 = 20.0; // Altura do chão
const SCREEN_HEIGHT: f32 = 600.0; // Altura da tela

struct MainState {
    pos_x: f32,
    pos_y: f32,
    velocity_y: f32,
    fuel: f32,
    gravity: f32,
    circle: Mesh,
    in_launch_area: bool,
    game_over: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(7.0, 8.0, 8.0, 20.0),
            Color::WHITE,
        )?;

        // Ajuste a coordenada Y inicial da nave para que ela comece acima da área de lançamento
        let pos_y = Y_LEVEL - 30.0;

        Ok(MainState {
            pos_x: LAUNCH_X,
            pos_y,
            velocity_y: 0.0,
            fuel: 100.0,
            gravity: 0.2,
            circle,
            in_launch_area: true,
            game_over: false,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.in_launch_area {
            println!("Na área de lançamento! Pressione 'W' para lançar.");
        } else {
            self.velocity_y += self.gravity;
            self.pos_y += self.velocity_y;
    
            if self.pos_y > Y_LEVEL {
                self.pos_y = Y_LEVEL;
                self.velocity_y = 0.0;
    
                if self.pos_x >= LANDING_X - 20.0 && self.pos_x <= LANDING_X + 20.0 {
                    println!("Pousou com segurança!");
                    self.game_over = true;
                } else {
                    println!("Aterrissagem incorreta! Game Over.");
                    self.game_over = true;
                }
            }
    
            if self.fuel <= 0.0 {
                println!("Sem combustível! Fim de jogo.");
                self.game_over = true;
            }
        }
    
        if self.game_over {
            event::quit(ctx);
        }
    
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 100, 255));

        // Desenhe o chão (retângulo marrom)
        let ground = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(0.0, Y_LEVEL + GROUND_HEIGHT, 800.0, 1000.0),
            Color::from_rgb(139, 69, 19), 
        )?;
        graphics::draw(ctx, &ground, DrawParam::default())?;

        // Desenhe a área de lançamento (retângulo verde)
        let launch_area = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(LAUNCH_X - 20.0, Y_LEVEL - 5.0, 40.0, 10.0),
            Color::from_rgb(0, 255, 0), // Verde
        )?;
        graphics::draw(ctx, &launch_area, DrawParam::default())?;

        // Desenhe a área de pouso (retângulo azul)
        let landing_area = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(LANDING_X - 20.0, Y_LEVEL - 5.0, 40.0, 10.0),
            Color::from_rgb(0, 0, 255), // Azul
        )?;
        graphics::draw(ctx, &landing_area, DrawParam::default())?;

        // Desenhe a nave
        graphics::draw(
            ctx,
            &self.circle,
            DrawParam::new()
                .dest([self.pos_x, self.pos_y])
        )?;

        // Desenhe a informação de combustível na tela
        let text = graphics::Text::new(format!("Fuel: {:.2}", self.fuel));
        let text_dest = graphics::mint::Point2 { x: 10.0, y: 10.0 }; // Coordenadas para a posição do texto
        graphics::draw(ctx, &text, (text_dest, Color::WHITE))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::A => {
                if self.pos_x > 0.0 {
                    self.pos_x -= 15.0;
                    self.fuel -= 1.0; // Gaste 5 de combustível ao mover para a esquerda
                }
            }
            KeyCode::D => {
                if self.pos_x < 800.0 {
                    self.pos_x += 15.0;
                    self.fuel -= 1.0; // Gaste 5 de combustível ao mover para a direita
                }
            }
            KeyCode::W => {
                if self.in_launch_area {
                    println!("Lançado!");
                    self.in_launch_area = false;
                } else if self.fuel >= 10.0 { // Gaste 10 de combustível apenas se houver pelo menos 10
                    self.velocity_y -= 5.0;
                    self.fuel -= 5.0; // Gaste 10 de combustível ao pressionar 'W'
                }
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("aterrizagem_nave", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
