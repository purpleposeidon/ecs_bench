# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | {pos_vel_build_calx_ecs}      | {pos_vel_update_calx_ecs}      | {parallel_build_calx_ecs}      | {parallel_update_calx_ecs}
 [constellation] | 3,197 µs/iter (+/- 522) | 73 µs/iter (+/- 1) | {parallel_build_constellation} | {parallel_update_constellation}
 [ecs]           | 16,672 µs/iter (+/- 464)           | 1,968 µs/iter (+/- 28)           | {parallel_build_ecs}           | {parallel_update_ecs}
 [froggy]        | 6,097 µs/iter (+/- 134)        | 87 µs/iter (+/- 2)        | {parallel_build_froggy}        | {parallel_update_froggy}
 [specs]         | 4,252 µs/iter (+/- 69)         | 37 µs/iter (+/- 1)         | {parallel_build_specs}         | {parallel_update_specs}
 [trex]          | 12,536 µs/iter (+/- 933)          | 2,242 µs/iter (+/- 52)          | {parallel_build_trex}          | {parallel_update_trex}

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs
[froggy]: https://github.com/kvark/froggy
[specs]: https://github.com/slide-rs/specs
[trex]: https://github.com/rcolinray/trex


Visualization of benchmarks, smaller is better.
![update benchmarks graph](./graph/update.png)
![build benchmarks graph](./graph/build.png)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
 * systems could be run in parallel

## Notes
 * the benchmarks explore a limited subset of ECS use-cases and do not necessarily reflect the peformance of large-scale applications
 * [froggy](https://github.com/kvark/froggy) is technically not an ECS, but a Component Graph System (CGS)
