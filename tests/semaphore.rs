use std::sync::{Condvar, Mutex};

pub struct Semaphore {
  current_weight: Mutex<usize>,
  max_weight: usize,
  cond_var: Condvar,
}

impl Semaphore {
  pub fn new(max_weight: usize) -> Self {
    Self {
      current_weight: Mutex::new(0),
      cond_var: Condvar::new(),
      max_weight,
    }
  }

  pub fn acquire(&self, weight: usize) -> Acquired<'_> {
    if weight > self.max_weight {
      panic!(
        "tried to acquire more weight thant the semaphore has. weight={} max_weight={}",
        weight, self.max_weight
      );
    }

    let current_weight = self.current_weight.lock().unwrap();
    let max_weight = self.max_weight;

    let mut current_weight = self
      .cond_var
      .wait_while(current_weight, |current_weight| {
        max_weight - *current_weight < weight
      })
      .unwrap();

    *current_weight += weight;

    Acquired {
      weight,
      semaphore: self,
    }
  }

  /// Note that this is a private method that should be called only by
  /// the Acquired Drop implementation.
  fn release(&self, weight: usize) {
    if weight > self.max_weight {
      panic!(
        "tried to release more weight thant the semaphore has. weight={} max_weight={}",
        weight, self.max_weight
      );
    }

    let mut current_weight = self.current_weight.lock().unwrap();

    *current_weight -= weight;

    self.cond_var.notify_all();
  }
}

pub struct Acquired<'a> {
  weight: usize,
  semaphore: &'a Semaphore,
}

impl<'a> Drop for Acquired<'a> {
  fn drop(&mut self) {
    self.semaphore.release(self.weight);
  }
}
