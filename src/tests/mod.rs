use std::sync::Arc;

use semaphore::Semaphore;

use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::run;

mod semaphore;

#[test]
fn runner() -> Result<(), Box<dyn std::error::Error>> {
  let subscriber = Registry::default().with(EnvFilter::from_env("RUST_LOG"));

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let sema = Arc::new(Semaphore::new(10));

  let mut handles = vec![];

  for file_path in std::fs::read_dir("src/tests/test_cases")? {
    let file_path = file_path.unwrap();

    if file_path.file_type()?.is_dir() {
      panic!("test cases folder should contain only files that aren't directories");
    }

    let path = file_path.path().to_str().map(|s| s.to_owned()).unwrap();
    if !path.ends_with(".input") {
      continue;
    }

    let sema_clone = Arc::clone(&sema);

    if !path.ends_with("basic_1.input") {
      continue;
    }

    // TODO: this is bad since many threads could crash the system,
    // we should acquire the semaphore before spawning the thread.
    handles.push(std::thread::spawn(move || {
      let _guard = sema_clone.acquire(1);

      let actual = run(&path).unwrap();

      let output_file_path = path.replace(".input", ".json");

      let expected = std::fs::read_to_string(&output_file_path).unwrap();

      assert_eq!(
        expected, actual,
        "\nexpected: {} \ngot: {}",
        expected, actual
      );
    }));
  }

  for handle in handles.into_iter() {
    handle.join().unwrap();
  }

  Ok(())
}
