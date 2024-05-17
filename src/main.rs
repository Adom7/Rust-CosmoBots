use ggez::{event, graphics, Context, GameResult};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

enum RobotType {
    Explorer,
}
struct ExtractorRobot {
    position: (usize, usize),
    target_energy: Option<(usize, usize)>,
    has_collected: bool,
}
struct Robot {
    position: (usize, usize),
    discovered_energy_spots: Vec<(usize, usize)>,
    done_exploring: bool,
}
struct Station {
    position: (usize, usize),
}

struct Map {
    obstacles: Vec<Vec<bool>>,
    energy_spots: Vec<(usize, usize)>, // Ajout pour stocker les positions des ressources d'énergie
    fog_of_war: Vec<Vec<bool>>, // Ajout pour le brouillard de guerre


}

impl ExtractorRobot {
    fn new(position: (usize, usize)) -> ExtractorRobot {
        ExtractorRobot {
            position,
            target_energy: None,
            has_collected: false,
        }
    }

    fn move_towards_target(&mut self, target: (usize, usize)) {
        let (tx, ty) = target;
        let (rx, ry) = self.position;

        let dx = (tx as isize - rx as isize).signum();
        let dy = (ty as isize - ry as isize).signum();

        self.position = ((rx as isize + dx) as usize, (ry as isize + dy) as usize);

        if self.position == target {
            self.has_collected = true;
        }
    }

    fn return_to_station(&mut self, station_position: &(usize, usize)) {
        self.move_towards_target(*station_position);
    }
}
impl Map {
    fn generate(seed: u64, width: usize, height: usize) -> Map {
        let mut rng = Pcg32::seed_from_u64(seed);
        let obstacles = (0..height)
            .map(|_| (0..width).map(|_| rng.gen::<f32>() > 0.7).collect())
            .collect();
        let energy_spots = (0..10)
            .map(|_| (rng.gen_range(0..width), rng.gen_range(0..height)))
            .collect();
        let fog_of_war = vec![vec![true; width]; height]; // Tout couvert initialement

        Map { obstacles, energy_spots, fog_of_war }
    }

    fn render(&self, ctx: &mut Context) -> GameResult<()> {
        // Dessiner les obstacles et autres éléments seulement si le brouillard est dissipé
        for (y, row) in self.obstacles.iter().enumerate() {
            for (x, &obstacle) in row.iter().enumerate() {
                if !self.fog_of_war[y][x] { // L'obstacle est dessiné seulement si le brouillard est dissipé
                    if obstacle {
                        let rect = graphics::Rect::new(
                            (x * 20) as f32, (y * 20) as f32, 20.0, 20.0,
                        );
                        let obstacle_color = graphics::Color::from_rgb(100, 100, 100);
                        let obstacle_mesh = graphics::Mesh::new_rectangle(
                            ctx, graphics::DrawMode::fill(), rect, obstacle_color,
                        )?;
                        graphics::draw(ctx, &obstacle_mesh, graphics::DrawParam::default())?;
                    }
                }
            }
        }
    
        // Dessiner les ressources d'énergie, si elles ne sont pas couvertes par le brouillard
        for &(x, y) in &self.energy_spots {
            if !self.fog_of_war[y][x] {
                let rect = graphics::Rect::new(
                    (x * 20) as f32, (y * 20) as f32, 20.0, 20.0,
                );
                let energy_color = graphics::Color::from_rgb(0, 255, 0);
                let energy_mesh = graphics::Mesh::new_rectangle(
                    ctx, graphics::DrawMode::fill(), rect, energy_color,
                )?;
                graphics::draw(ctx, &energy_mesh, graphics::DrawParam::default())?;
            }
        }
    
        Ok(())
    }
    fn explored_percentage(&self) -> f32 {
        let total_cells = self.fog_of_war.len() * self.fog_of_war[0].len();
        let explored_cells = self.fog_of_war.iter().flatten().filter(|&&cell| !cell).count();
        explored_cells as f32 / total_cells as f32 * 100.0
    }

    
    
}

impl Robot {
    fn return_to_station(&mut self, station_position: &(usize, usize)) {
        if self.position != *station_position {
            let (sx, sy) = *station_position;
            let (rx, ry) = self.position;

            let dx = (sx as isize - rx as isize).signum();
            let dy = (sy as isize - ry as isize).signum();

            self.position = ((rx as isize + dx) as usize, (ry as isize + dy) as usize);
        }
    }
    fn new(position: (usize, usize)) -> Robot {
        Robot {
            position,
            discovered_energy_spots: Vec::new(),
            done_exploring: false,
        }
    }

    fn explore(&mut self, map: &mut Map) {
        if self.done_exploring {
            return; // S'arrêter si l'exploration est terminée.
        }
        
        let (x, y) = self.position;
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1);
        let dy = rng.gen_range(-1..=1);
        
        let new_x = (x as isize + dx).clamp(0, map.obstacles[0].len() as isize - 1) as usize;
        let new_y = (y as isize + dy).clamp(0, map.obstacles.len() as isize - 1) as usize;
        
        if !map.obstacles[new_y][new_x] {
            self.position = (new_x, new_y);
            // Dissiper le brouillard autour de la nouvelle position
            for j in new_y.saturating_sub(1)..=new_y + 1 {
                for i in new_x.saturating_sub(1)..=new_x + 1 {
                    if i < map.fog_of_war[0].len() && j < map.fog_of_war.len() {
                        map.fog_of_war[j][i] = false; // Dissiper le brouillard
                    }
                }
            }
        }
    }
}

struct GameState {
    map: Map,
    robot: Robot,
    station: Station,
    extractors: Vec<ExtractorRobot>,

}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let map = Map::generate(1234, 20, 15);
        let robot = Robot::new((5, 5));
        let station = Station { position: (10, 10) };
        let extractors = Vec::new();  // Initialisation de extractors comme un vecteur vide
        Ok(GameState { map, robot, station, extractors })
    }
    fn deploy_extractors(&mut self) {
        for energy_spot in self.map.energy_spots.clone() {
            let mut extractor = ExtractorRobot::new(self.station.position);
            extractor.target_energy = Some(energy_spot);
            self.extractors.push(extractor);
        }
    }

    fn update_extractors(&mut self) {
        for extractor in self.extractors.iter_mut() {
            if let Some(target) = extractor.target_energy {
                if !extractor.has_collected {
                    extractor.move_towards_target(target);
                } else {
                    extractor.return_to_station(&self.station.position);
                    // Remove the energy spot from the map when collected
                    self.map.energy_spots.retain(|&e| e != target);
                }
            } else {
                // Assign a target energy spot if the extractor doesn't have one
                if let Some(energy_spot) = self.map.energy_spots.pop() {
                    extractor.target_energy = Some(energy_spot);
                }
            }
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let explored_percent = self.map.explored_percentage();
        println!("Map explored: {:.2}%", explored_percent); // Afficher le pourcentage exploré

        if !self.robot.done_exploring {
            self.robot.explore(&mut self.map);
            if explored_percent >= 100.0 { // Si toute la map est explorée
                self.robot.done_exploring = true;
            }
        } else if self.robot.position != self.station.position {
            // Diriger le robot vers la station si ce n'est pas déjà fait
            self.robot.return_to_station(&self.station.position);
        } else {
            println!("Robot has returned to the station.");
            if self.extractors.is_empty() {
                self.deploy_extractors();
            } else {
                self.update_extractors();
            }
        }
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
                // Dessiner les robots ici...
        for extractor in &self.extractors {
            let (x, y) = extractor.position;
            let rect = graphics::Rect::new((x * 20) as f32, (y * 20) as f32, 20.0, 20.0);
            let color = graphics::Color::from_rgb(0, 0, 255); // Bleu pour les extracteurs
            let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }
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
