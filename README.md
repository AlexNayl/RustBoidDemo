# RustBoidDemo

View the demo [HERE](https://alexnayl.github.io/RustBoidDemo/)

Made with Rust, Using Web Assembly and the Bevy Game Engine.

This demo shows a model for boid behaviour, seen in the real world in structures such as schools of fish.

# Known Issues
- Coheasion behavour breaks down when going through periodic boundaries due to the teleportation messing with the average position of the seen entities. Could be fixed by using average relative position instead, since I made a difference function that handles the repeating boundary properly.
- Delay on page load due to the wasm loading, could use a loading notification.
