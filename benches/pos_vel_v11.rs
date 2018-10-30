#![feature(test)]

extern crate test;
use test::Bencher;
extern crate ecs_bench;

#[macro_use] extern crate v11;
#[macro_use] extern crate v11_macros;

use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_PER_VEL};

domain! { BENCH }

table! {
    #[kind = "consistent"]
    [BENCH/positions] {
        p: [Position; VecCol<Position>],
    }
}
table! {
    #[kind = "consistent"]
    [BENCH/velocities] {
        #[foreign_auto]
        #[index]
        posid: [positions::RowId; VecCol<positions::RowId>],
        v: [Velocity; VecCol<Velocity>],
    }
}

use v11::Universe;

fn build() -> Universe {
    static ONCE: ::std::sync::Once = ::std::sync::Once::new();
    ONCE.call_once(|| {
        BENCH.register();
        positions::register();
        velocities::register();
    });
    let universe = Universe::new(&[BENCH]);
    {
        let mut positions = positions::write(&universe);
        let mut velocities = velocities::write(&universe);
        positions.push_all((0..N_POS).map(|_| positions::Row {
            p: Position {
                x: 0.0,
                y: 0.0,
            },
        }));
        velocities.push_all(
            (0..N_POS)
            .filter(|i| i % N_POS_PER_VEL == 0)
            .map(|i| {
                let posid = positions::RowId::from_usize(i);
                velocities::Row {
                    posid,
                    v: Velocity {
                        dx: 0.0,
                        dy: 0.0,
                    },
                }
            })
        );
        velocities.flush(&universe, ::v11::event::CREATE);
        positions.flush(&universe, ::v11::event::CREATE);
    }
    universe
}

fn update(universe: &Universe) {
    let mut positions = positions::write(universe);
    let velocities = velocities::read(universe);
    for vid in velocities.iter() {
        let posid = velocities.posid[vid];
        let p = &mut positions.p[posid];
        p.x += velocities.v[vid].dx;
        p.y += velocities.v[vid].dy;
    }
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let universe = build();

    b.iter(|| {
        update(&universe);
    });
}
