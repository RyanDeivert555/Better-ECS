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

const TILE_SIZE: f32 = 64.0;
const TILE_X_COUNT: i32 = 20;
const TILE_Y_COUNT: i32 = 15;
const WIDTH: i32 = TILE_SIZE as i32 * TILE_X_COUNT;
const HEIGHT: i32 = TILE_SIZE as i32 * TILE_Y_COUNT;
const MAX_HEALTH: f32 = 100.0;

make_component! {
    struct Handle(RaylibHandle);
}

make_component! {
    struct Thread(RaylibThread);
}

make_component! {
    struct Position(Vector2);
}

make_component! {
    struct Direction(Vector2);
}

make_component! {
    struct Speed(f32);
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
    struct Health(f32);
}

fn rand_bool() -> bool {
    get_random_value::<i32>(0, 1) == 0
}

fn add_raylib(world: &mut World) {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("RPG Example")
        .build();
    rl.set_target_fps(60);

    world.add_resource(Handle(rl));
    world.add_resource(Thread(thread));
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Direction>();
    world.register::<Speed>();
    world.register::<Render>();
    world.register::<Player>();
    world.register::<Monster>();
    world.register::<Health>();
}

fn add_player(world: &mut World) {
    let _player = world
        .new_entity()
        .with(Position(Vector2::zero()))
        .with(Direction(Vector2::zero()))
        .with(Speed(200.0))
        .with(Render(Color::BLUE))
        .with(Player)
        .with(Health(MAX_HEALTH))
        .build();
}

fn add_monsters(world: &mut World) {
    for _ in 0..10 {
        let position = Position(Vector2::new(
            get_random_value::<i32>(0, TILE_X_COUNT) as f32 * TILE_SIZE,
            get_random_value::<i32>(0, TILE_Y_COUNT) as f32 * TILE_SIZE,
        ));

        let _monster = world
            .new_entity()
            .with(position)
            .with(Direction(Vector2::zero()))
            .with(Speed(100.0))
            .with(Monster)
            .with(Render(Color::RED))
            .with(Health(MAX_HEALTH))
            .build();
    }
}

fn change_player_velocity(world: &mut World) {
    let (_, mut dir) = world.query_single_mut::<(Player, Direction)>().unwrap();
    let rl = world.get_resource::<Handle>().unwrap();

    if rl.0.is_key_down(KeyboardKey::KEY_W) {
        dir.0.y -= 1.0;
    }
    if rl.0.is_key_down(KeyboardKey::KEY_S) {
        dir.0.y += 1.0;
    }
    if rl.0.is_key_down(KeyboardKey::KEY_A) {
        dir.0.x -= 1.0;
    }
    if rl.0.is_key_down(KeyboardKey::KEY_D) {
        dir.0.x += 1.0;
    }
}

fn change_monsters_velocity(world: &mut World) {
    let monsters = world.query_mut::<(Monster, Direction)>();

    for (_, mut dir) in monsters {
        if rand_bool() {
            let x = get_random_value::<i32>(-1, 1) as f32;
            let y = get_random_value::<i32>(-1, 1) as f32;

            dir.0 += Vector2::new(x, y);
        }
    }
}

fn change_color(world: &mut World) {
    let query = world.query_mut::<(Render, Health)>();

    for (mut render, health) in query {
        let percent = health.0 / MAX_HEALTH;

        render.0 = render.0.fade(percent);
    }
}

fn cull_entities(world: &mut World) {
    let query = world.query::<(EntityId, Health)>();
    // this kinda sucks
    let ids = query
        .filter(|(_, health)| health.0 <= 0.0)
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    ids.into_iter().for_each(|id| world.remove_entity(id));
}

fn move_system(world: &mut World) {
    let query = world.query_mut::<(Position, Direction, Speed)>();
    let rl = world.get_resource::<Handle>().unwrap();
    let dt = rl.0.get_frame_time();

    for (mut pos, mut dir, speed) in query {
        pos.0 += dir.0 * speed.0 * dt;
        dir.0 = Vector2::zero();
    }
}

fn draw_system(world: &mut World) {
    let mut rl = world.get_resource_mut::<Handle>().unwrap();
    let thread = world.get_resource::<Thread>().unwrap();
    let query = world.query::<(Position, Render)>();

    let mut d = rl.0.begin_drawing(&thread.0);
    d.clear_background(Color::RAYWHITE);
    for (p, render) in query {
        d.draw_rectangle_v(p.0, Vector2::new(TILE_SIZE, TILE_SIZE), render.0);
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
        .add_system(change_player_velocity)
        .add_system(change_monsters_velocity)
        .add_system(change_color)
        .add_system(cull_entities)
        .add_system(move_system)
        .add_system(draw_system)
        .add_system(close_system)
        .run();
}
