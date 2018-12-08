#[macro_use]
extern crate lazy_static;
extern crate rand;

use hlt::command::Command;
use hlt::direction::Direction;
use hlt::game::Game;
use hlt::log::Log;
use hlt::navi::Navi;
use rand::Rng;
use rand::SeedableRng;
use rand::XorShiftRng;
use std::env;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

mod hlt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rng_seed: u64 = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    };
    let seed_bytes: Vec<u8> = (0..16).map(|x| ((rng_seed >> (x % 8)) & 0xFF) as u8).collect();
    let mut rng: XorShiftRng = SeedableRng::from_seed([
        seed_bytes[0], seed_bytes[1], seed_bytes[2], seed_bytes[3],
        seed_bytes[4], seed_bytes[5], seed_bytes[6], seed_bytes[7],
        seed_bytes[8], seed_bytes[9], seed_bytes[10], seed_bytes[11],
        seed_bytes[12], seed_bytes[13], seed_bytes[14], seed_bytes[15]
    ]);

    let mut game = Game::new();
    let mut navi = Navi::new(game.map.width, game.map.height);
    // At this point "game" variable is populated with initial map data.
    // This is a good place to do computationally expensive start-up pre-processing.
    // As soon as you call "ready" function below, the 2 second per turn timer will start.
    Game::ready("MyRustBot");

    Log::log(&format!("Successfully created bot! My Player ID is {}. Bot rng seed is {}.", game.my_id.0, rng_seed));

    loop {
        game.update_frame();
        navi.update_frame(&game);

        let me = &game.players[game.my_id.0];
        let map = &mut game.map;

        let mut command_queue: Vec<Command> = Vec::new();

        for ship_id in &me.ship_ids {
            let ship = &game.ships[ship_id];
            let cell = map.at_entity(ship);

            let command = if cell.halite < game.constants.max_halite / 10 || ship.is_full() {
                let random_direction = Direction::get_all_cardinals()[rng.gen_range(0, 4)];
                ship.move_ship(random_direction)
            } else {
                ship.stay_still()
            };
            command_queue.push(command);
        }

        if
            game.turn_number <= 200 &&
            me.halite >= game.constants.ship_cost &&
            navi.is_safe(&me.shipyard.position)
        {
            command_queue.push(me.shipyard.spawn());
        }


        Game::end_turn(&command_queue);
    }
}

#[cfg(test)]
mod tests {
    use hlt::game_map::GameMap;
    use hlt::position::Position;
    use std::time::{SystemTime};

    #[test]
    fn it_works() {
        let start = SystemTime::now();
        let dimension = 10;
        let mut map = GameMap::new(dimension, dimension);
        for y in 0..dimension {
            for x in 0..dimension {
                map.cells[y][x].halite = x * y
            }
        }
        assert_eq!(2 + 2, 4);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Duration failed");
        println!("Duration: {}", duration.as_secs() * 1000 * 1000 + duration.subsec_micros() as u64);
    }

    fn calculate_vector_wrapped(map: &GameMap, target: &Position, source: &Position, halite: i32) -> (i32, i32) {
        let normalized_source = map.normalize(source);
        let normalized_target = map.normalize(target);
        let dx = normalized_target.x - normalized_source.x;
        let dy = normalized_target.y - normalized_source.y;
        let abs_dx = dx.abs();
        let abs_dy = dy.abs();
        let wrapped_dx = map.width as i32 - abs_dx;
        let wrapped_dy = map.height as i32 - abs_dy;
        let dx_sign = dx / dx.abs();
        let dy_sign = dy / dy.abs();
        let actual_dx = if abs_dx < wrapped_dx { dx } else { wrapped_dx * dx_sign * -1 };
        let actual_dy = if abs_dy < wrapped_dy { dy } else { wrapped_dy * dy_sign * -1 };

        let magnitude = actual_dx.abs() + actual_dy.abs();
        if magnitude == 0 {
            return (0, 0)
        }

        let halite_scaling = 0.8_f32.powi(magnitude) * halite as f32 / magnitude as f32;
        let x_halite_influence = (actual_dx as f32 * halite_scaling) as i32;
        let y_halite_influence = (actual_dy as f32 * halite_scaling) as i32;
        return (x_halite_influence, y_halite_influence)
    }
}
