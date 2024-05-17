use ggez::{event, graphics, Context, GameResult};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

enum RobotType {
    Explorer,
}

struct Robot {
    position: (usize, usize),
}

struct Station {
    position: (usize, usize),
}

struct Map {
    obstacles: Vec<Vec<bool>>,
    energy_spots: Vec<(usize, usize)>, // Ajout pour stocker les positions des ressources d'énergie

}

impl Map {
    fn generate(seed: u64, width: usize, height: usize) -> Map {
        let mut rng = Pcg32::seed_from_u64(seed);
        let obstacles: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| rng.gen::<f32>() > 0.7).collect())
            .collect();

        // Génération aléatoire des spots d'énergie
        let energy_spots: Vec<(usize, usize)> = (0..10) // Générer 10 spots d'énergie
            .map(|_| (rng.gen_range(0..width), rng.gen_range(0..height)))
            .collect();

        Map { obstacles, energy_spots }
    }

    fn render(&self, ctx: &mut Context) -> GameResult<()> {
        // Dessiner les obstacles
        for (y, row) in self.obstacles.iter().enumerate() {
            for (x, &obstacle) in row.iter().enumerate() {
                if obstacle {
                    let rect = graphics::Rect::new(
                        (x * 20) as f32,
                        (y * 20) as f32,
                        20.0,
                        20.0,
                    );
                    let obstacle_color = graphics::Color::from_rgb(100, 100, 100);
                    let obstacle_mesh = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        obstacle_color,
                    )?;
                    graphics::draw(ctx, &obstacle_mesh, graphics::DrawParam::default())?;
                }
            }
        }
    
        // Dessiner les spots d'énergie
        for &(x, y) in &self.energy_spots {
            let rect = graphics::Rect::new(
                (x * 20) as f32,
                (y * 20) as f32,
                20.0,
                20.0,
            );
            let energy_color = graphics::Color::from_rgb(0, 255, 0); // Utiliser une couleur verte pour l'énergie
            let energy_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                energy_color,
            )?;
            graphics::draw(ctx, &energy_mesh, graphics::DrawParam::default())?;
        }
    
        Ok(())
    }
    
}

impl Robot {
    fn new(position: (usize, usize)) -> Robot {
        Robot { position }
    }

    fn explore(&mut self, map: &Map) {
        let (x, y) = self.position;
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1);
        let dy = rng.gen_range(-1..=1);
        
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;
        
        if new_x < map.obstacles[0].len() && new_y < map.obstacles.len() && !map.obstacles[new_y][new_x] {
            self.position = (new_x, new_y);
        }
    }
}

struct GameState {
    map: Map,
    robot: Robot,
    station: Station,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let map = Map::generate(1234, 20, 15);
        let robot = Robot::new((5, 5));
        let station = Station { position: (10, 10) };
        Ok(GameState { map, robot, station })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.robot.explore(&self.map);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.map.render(ctx)?;

        // Dessiner le robot
        let (x, y) = self.robot.position;
        let rect = graphics::Rect::new((x * 20) as f32, (y * 20) as f32, 20.0, 20.0);
        let robot_color = graphics::Color::from_rgb(255, 0, 0); // Rouge pour l'explorateur
        let robot_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, robot_color)?;
        graphics::draw(ctx, &robot_mesh, graphics::DrawParam::default())?;

        // Dessiner la station
        let (sx, sy) = self.station.position;
        let station_rect = graphics::Rect::new((sx * 20) as f32, (sy * 20) as f32, 20.0, 20.0);
        let station_color = graphics::Color::from_rgb(255, 255, 0); // Jaune pour la station
        let station_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), station_rect, station_color)?;
        graphics::draw(ctx, &station_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("exploration_spatiale", "votre_nom");
    let (ctx, event_loop) = &mut cb.build()?;
    let game = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, game)
}
