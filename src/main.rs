mod app;
mod component;
mod entity_builder;
mod prelude;
mod query;
mod scheduler;
mod tests;
mod world;
use crate::prelude::*;
// RPG Example
use raylib::prelude::*;

const TILE_SIZE: i32 = 64;
const TILE_X_COUNT: i32 = 20;
const TILE_Y_COUNT: i32 = 15;
const WIDTH: i32 = TILE_SIZE * TILE_X_COUNT;
const HEIGHT: i32 = TILE_SIZE * TILE_Y_COUNT;

make_component! {
    struct Handle(RaylibHandle);
}

make_component! {
    struct Thread(RaylibThread);
}

make_component! {
    #[derive(Copy, Clone, PartialEq)]
    struct Position {
        x: i32,
        y: i32,
    }
}

make_component! {
    struct Render(Color);
}

make_component! {
    struct Player;
}

make_component! {
    struct Monster;
}

make_component! {
    struct MoveTimer(f32);
}

make_component! {
    #[derive(PartialEq)]
    enum Turn {
        Player,
        Monster,
    }
}

fn rand_bool() -> bool {
    get_random_value::<i32>(0, 1) == 0
}

fn get_move_result(mover: Position, obstacle: Position, step: Position) -> Position {
    let mut result = Position { x: 0, y: 0 };
    result.x = mover.x + step.x;
    result.y = mover.y + step.y;

    if result != obstacle {
        result
    } else {
        Position { x: 0, y: 0 }
    }
}

fn add_raylib(world: &mut World) {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("RPG Example")
        .build();
    rl.set_target_fps(60);

    world.add_resource(Turn::Player);
    world.add_resource(Handle(rl));
    world.add_resource(Thread(thread));
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Render>();
    world.register::<Player>();
    world.register::<Monster>();
}

fn add_player(world: &mut World) {
    let _player = world
        .new_entity()
        .with(Position { x: 0, y: 0 })
        .with(Render(Color::BLUE))
        .with(Player)
        .build();
}

fn add_monsters(world: &mut World) {
    for _ in 0..10 {
        let position = Position {
            x: get_random_value::<i32>(0, TILE_X_COUNT) * TILE_SIZE,
            y: get_random_value::<i32>(0, TILE_Y_COUNT) * TILE_SIZE,
        };

        let _monster = world
            .new_entity()
            .with(position)
            .with(Monster)
            .with(Render(Color::RED))
            .build();
    }
}

fn move_player(world: &mut World) {
    let mut turn = world.get_resource_mut::<Turn>().unwrap();

    if *turn == Turn::Monster {
        return;
    }

    let (_, mut pos) = world.query_single_mut::<(Player, Position)>().unwrap();
    let mut monsters = world.query::<(Monster, Position)>();
    let rl = world.get_resource::<Handle>().unwrap();

    let mut movement = Position { x: 0, y: 0 };
    if rl.0.is_key_pressed(KeyboardKey::KEY_W) {
        movement.y -= TILE_SIZE;
    }
    if rl.0.is_key_pressed(KeyboardKey::KEY_S) {
        movement.y += TILE_SIZE;
    }
    if rl.0.is_key_pressed(KeyboardKey::KEY_A) {
        movement.x -= TILE_SIZE;
    }
    if rl.0.is_key_pressed(KeyboardKey::KEY_D) {
        movement.x += TILE_SIZE;
    }

    if movement.x == 0 && movement.y == 0 {
        return;
    }

    let collide = monsters.any(|(_, m_pos)| {
        let new_x = pos.x + movement.x;
        let new_y = pos.y + movement.y;

        let b_x = m_pos.x;
        let b_y = m_pos.y;

        (new_x == b_x) && (new_y == b_y)
    });

    if !collide {
        pos.x += movement.x;
        pos.y += movement.y;
    }
    *turn = Turn::Monster;
}

fn move_monsters(world: &mut World) {
    let mut turn = world.get_resource_mut::<Turn>().unwrap();

    if *turn == Turn::Player {
        return;
    }

    let monsters = world.query_mut::<(Monster, Position)>();
    let (_, player_pos) = world.query_single::<(Player, Position)>().unwrap();

    for (_, mut pos) in monsters {
        if rand_bool() {
            let x = get_random_value::<i32>(-1, 1) * TILE_SIZE;
            let y = get_random_value::<i32>(-1, 1) * TILE_SIZE;

            // if pos.x + rand_x != player_pos.x {
            //     pos.x += rand_x;
            // }
            // if pos.y + rand_y != player_pos.y {
            //     pos.y += rand_y;
            // }
            *pos = get_move_result(*pos, *player_pos, Position { x, y });
        }
    }

    *turn = Turn::Player;
}

fn draw_system(world: &mut World) {
    let mut rl = world.get_resource_mut::<Handle>().unwrap();
    let thread = world.get_resource::<Thread>().unwrap();
    let query = world.query::<(Position, Render)>();

    let mut d = rl.0.begin_drawing(&thread.0);
    d.clear_background(Color::RAYWHITE);
    for (p, render) in query {
        d.draw_rectangle(p.x, p.y, TILE_SIZE, TILE_SIZE, render.0);
    }

    d.draw_fps(0, 0);
}

fn close_system(world: &mut World) {
    let shutdown = {
        let rl = world.get_resource::<Handle>().unwrap();

        rl.0.window_should_close()
    };

    if shutdown {
        world.shutdown();
    }
}

fn main() {
    App::new()
        .add_startup_system(add_raylib)
        .add_startup_system(register_components)
        .add_startup_system(add_player)
        .add_startup_system(add_monsters)
        .add_system(move_player)
        .add_system(move_monsters)
        .add_system(draw_system)
        .add_system(close_system)
        .run();
}
