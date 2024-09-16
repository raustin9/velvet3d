use velvet3D::engine::*;


fn main() {
    let mut engine = Engine::new();
    engine.create_window("Test Window", 800, 600);
    engine.run();
}