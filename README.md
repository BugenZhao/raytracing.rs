# raytracing.rs

An implementation of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html) in 2.1 kLoC of Rust.

## Features
- **Basic** textures, materials, objects and transformations
- **Simple** BVH for object organization
- **Naive** one-line pixel-level parallelism
- **Trivial** macro- and generic-based static dispatches
- **Childish** interactive GUI (see `interactive` branch)

## Gallery
- Weekend Final
    ![weekend_final_bvh](out/weekend_final_bvh.png)
- Rectangle Light
    ![rect_light](out/rect_light.png)
- Cornell Box
    ![cornell_box](out/cornell_box.png)
- Smoke Sphere in Cornell Box
    ![cornell_smoke_sphere](out/cornell_smoke_sphere.png)
- Checker
    ![checker](out/checker.png)

## Roadmap
- [ ] SIMD support
- [ ] PDF support
