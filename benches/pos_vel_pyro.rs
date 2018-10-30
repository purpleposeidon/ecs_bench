#![feature(test)]

extern crate test;
use test::Bencher;

extern crate specs;

extern crate ecs_bench;
extern crate pyro;

use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_PER_VEL};
use pyro::*;

fn update(world: &mut World) {
    world
        .matcher::<All<(Write<Position>, Read<Velocity>)>>()
        .for_each(|(p, v)| {
            p.x += v.dx;
            p.y += v.dy;
        });
}

fn build() -> World {
    let mut world = World::<SoaStorage>::new();
    let only_pos = (0..N_POS).map(|_| {
        (
            Position { x: 0.0, y: 0.0 },
        )
    });
    let pos_vel = (0..N_POS)
        .filter(|i| i % N_POS_PER_VEL == 0)
        .map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));

    world.append_components(only_pos);
    world.append_components(pos_vel);
    world
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();

    b.iter(|| {
        update(&mut world);
    });
}
