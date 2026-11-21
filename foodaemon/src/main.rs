use std::fs::TryLockError;

use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> Result<(), i8> {
    let wayland_display = std::env::var("WAYLAND_DISPLAY").expect(
        "Failed to run Foodaemon because WAYLAND_DISPLAY environment \
            variable is not set or is not a valid utf-8 string!"
    );
    
    let mut runtime_dir = std::env::var("XDG_RUNTIME_DIR").expect(
        "Failed to run Foodaemon because XDG_RUNTIME_DIR environment \
            variable is not set or is not a valid utf-8 string!"
    );
    // Remove the last character if it is /
    if let Some(lastchar) = runtime_dir.chars().last() && lastchar == '/' {
        runtime_dir.pop().unwrap();
    }

    let soc_path = format!("{runtime_dir}/{wayland_display}-foodaemon");
    let lock_path = format!("{soc_path}.lock");

    // Create lockfile or check whether another instance 
    // of the daemon is using the socket
    use std::os::unix::fs::OpenOptionsExt;
    const O_CREAT: i32 = 0x40;
    const O_CLOEXEC: i32 = 0x80000;
    const O_RDWR: i32 = 2;
    
    // Same as open(path, O_CREAT | O_CLOEXEC | O_RDWR, 0660);
    let lock_file = std::fs::File::
        options()
        .write(true)
        .mode(0o660)
        .custom_flags(O_CREAT | O_CLOEXEC | O_RDWR)
        .open(lock_path)
        .expect("Failed to create socket lock file");

    if let Err(TryLockError::WouldBlock) = lock_file.try_lock() {
        println!("Foobar daemon is already open! (socket lockfile is locked)");
        return Err(-1);
    }

    // We don't care whether this succeeded or not
    let _ = std::fs::remove_file(&soc_path);
    
    let listener = match UnixListener::bind(&soc_path) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to create socket at {soc_path}: {e}");
            return Err(-1);
        }
    };

    loop {
        match listener.accept().await {
            Ok((_stream, addr)) => {
                println!("New client with address: {addr:?}");
            }
            Err(e) => println!("Error accepting client: {e}")
        }
    }
}