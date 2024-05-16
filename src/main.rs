use ggez::{event, graphics, Context, GameResult};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

enum RobotType {
    Explorer,
    Collector,
    Analyzer,
}

struct Robot {
    robot_type: RobotType,
    position: (usize, usize),
    // Autres champs selon les besoins
}

struct Station {
    robots: Vec<Robot>,
    data: Vec<ScientificData>,
    energy: usize, // Champ energy ajouté
    minerals: usize, // Champ minerals ajouté
    // Autres champs selon les besoins
}

struct ScientificData {
    chemical_composition: String,
    // Ajoutez d'autres champs au besoin
}

impl ScientificData {
    fn new(chemical_composition: &str) -> Self {
        ScientificData {
            chemical_composition: String::from(chemical_composition),
        }
    }
}

struct Map {
    obstacles: Vec<Vec<bool>>,
    energy_spots: Vec<(usize, usize)>,
    mineral_spots: Vec<(usize, usize)>,
}

impl Map {
    fn generate(seed: u64, width: usize, height: usize) -> Map {
        let mut rng = Pcg32::seed_from_u64(seed);
        let obstacles: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| rng.gen::<f32>() > 0.7).collect())
            .collect();
        Map { obstacles, energy_spots: Vec::new(), mineral_spots: Vec::new() }
    }

    fn render(&self, ctx: &mut Context) -> GameResult<()> {
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
        Ok(())
    }
}

impl Robot {
    fn new(robot_type: RobotType, position: (usize, usize)) -> Robot {
        Robot { robot_type, position }
    }

    fn behave(&mut self, map: &mut Map, station: &mut Station) {
        match self.robot_type {
            RobotType::Explorer => self.explore(map),
            RobotType::Collector => self.collect_resources(map, station),
            RobotType::Analyzer => self.analyze(map, station),
        }
    }

    fn explore(&mut self, map: &Map) {
        let (x, y) = self.position;
        
        // Par exemple, déplacer le robot aléatoirement
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1); // Déplacement horizontal aléatoire (-1, 0 ou 1)
        let dy = rng.gen_range(-1..=1); // Déplacement vertical aléatoire (-1, 0 ou 1)
        
        let new_x = (x as isize + dx) as usize;
        let new_y = (y as isize + dy) as usize;
        
        // Vérifier si le nouveau déplacement est valide (pas en dehors de la carte ou sur un obstacle)
        if new_x < map.obstacles[0].len() && new_y < map.obstacles.len() && !map.obstacles[new_y][new_x] {
            self.position = (new_x, new_y);
        }
    }

    fn collect_resources(&mut self, map: &mut Map, station: &mut Station) {
        let (x, y) = self.position;
        
        // Exemple: collecter des ressources d'énergie
        if let Some(energy_index) = map.energy_spots.iter().position(|&(ex, ey)| ex == x && ey == y) {
            // Supprimer la ressource d'énergie de la carte
            map.energy_spots.remove(energy_index);
            
            // Ajouter de l'énergie à la station
            station.energy += 1; // Exemple: ajouter 1 unité d'énergie
        }
        
        // Exemple: collecter des minerais
        if let Some(mineral_index) = map.mineral_spots.iter().position(|&(mx, my)| mx == x && my == y) {
            // Supprimer le minerai de la carte
            map.mineral_spots.remove(mineral_index);
            
            // Ajouter des minerais à la station
            station.minerals += 1; // Exemple: ajouter 1 unité de minerai
        }
    }
    

    fn analyze(&mut self, map: &Map, station: &mut Station) {
        // Logique pour analyser les échantillons collectés et transmettre les données à la station
    }
}

impl Station {
    fn process_data(&mut self) {
        // Logique pour traiter les données scientifiques collectées par les robots
    }

    fn create_robot(&mut self) {
        // Logique pour décider du type de robot à créer en fonction des besoins de la mission
    }
}

struct GameState {
    map: Map,
    robots: Vec<Robot>,
    station: Station,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        let map = Map::generate(1234, 20, 15);
        let robots = vec![Robot::new(RobotType::Explorer, (5, 5)), Robot::new(RobotType::Collector, (10, 10))];
        let station = Station { robots: Vec::new(), data: Vec::new(), energy: 0, minerals: 0 };
        Ok(GameState { map, robots, station })
    }
}
impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Mettre à jour les robots
        for robot in &mut self.robots {
            robot.behave(&mut self.map, &mut self.station);
        }
        
        // Traiter les données à la station
        self.station.process_data();
        
        // Créer de nouveaux robots si nécessaire
        self.station.create_robot();
    
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Dessiner la carte et les obstacles
        graphics::clear(ctx, graphics::WHITE);
        self.map.render(ctx)?;

        // Dessiner les robots
        for robot in &self.robots {
            let (x, y) = robot.position;
            let rect = graphics::Rect::new((x * 20) as f32, (y * 20) as f32, 20.0, 20.0);
            let robot_color = match robot.robot_type {
                RobotType::Explorer => graphics::Color::from_rgb(255, 0, 0),
                RobotType::Collector => graphics::Color::from_rgb(0, 255, 0),
                RobotType::Analyzer => graphics::Color::from_rgb(0, 0, 255),
            };
            let robot_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, robot_color)?;
            graphics::draw(ctx, &robot_mesh, graphics::DrawParam::default())?;
        }

        // Dessiner la station
        let station_rect = graphics::Rect::new(100.0, 100.0, 50.0, 50.0); // Position et taille de la station
        let station_color = graphics::Color::from_rgb(255, 255, 0); // Couleur jaune pour la station
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
