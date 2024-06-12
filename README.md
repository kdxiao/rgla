# rgla - Rust Graphical Linear Algebra 
I wrote this linear algebra library to familiarize myself with the graphics pipeline. It's quite unoptimized at the moment. 

## Features
- [X] Mat4: 4x4 Matrices
- [ ] Mat3: 3x3 Matrices
- [ ] Mat2: 2x2 Matrices
- [X] Vec4: 4D Vectors
- [X] Vec3: 3D Vectors
- [X] Vec2: 2D Vectors
- [ ] Rotors
- [X] Bivectors
- [ ] Transformations

## Novelty
The novelty of this library is that it implements [rotors](https://marctenbosch.com/quaternions/) (instead of quaternions) to represent rotation. They do pretty much the same thing and have almost the same interface, but rotors make much more sense to think about. 

## Credits
Heavily inspired by [`glam-rs`](https://github.com/bitshifter/glam-rs). 
