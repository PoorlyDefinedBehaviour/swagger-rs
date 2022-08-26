use std::{fs::File, sync::Arc};

use semaphore::Semaphore;

mod semaphore;

#[test]
fn runner() -> Result<(), Box<dyn std::error::Error>> {
  let sema = Arc::new(Semaphore::new(10));

  let mut handles = vec![];

  for file_path in std::fs::read_dir("./")? {
    let file_path = file_path.unwrap();

    if file_path.file_type()?.is_dir() {
      panic!("test cases folder should contain only files that aren't directories");
    }

    let sema_clone = Arc::clone(&sema);

    // TODO: this is bad since many threads could crash the system,
    // we should acquire the semaphore before spawning the thread.
    handles.push(std::thread::spawn(move || {
      let _guard = sema_clone.acquire(1);

      run(file_path.path().as_ref())
    }));
  }

  for handle in handles.into_iter() {
    handle.join()?;
  }

  Ok(())
}
