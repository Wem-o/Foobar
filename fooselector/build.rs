use vergen::{Build, Cargo, Emitter};

fn main() {
    let build = Build::all_build();
    let cargo = Cargo::all_cargo();
    
    Emitter::default()
        .add_instructions(&build)
        .unwrap()
        .add_instructions(&cargo)
        .unwrap()
        .emit()
        .unwrap();
}