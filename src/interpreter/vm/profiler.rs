use std::collections::BTreeMap;
use std::time::{Instant, Duration};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct ProfilingEntry {
  count: u64,
  duration: Duration,
}

pub(crate) struct ProfilingContext {
  begin: Instant,
  id: &'static str,
}

pub(crate) struct Profiler {
  data: RefCell<BTreeMap<&'static str, ProfilingEntry>>,
}

impl Profiler {
  pub fn new() -> Self {
    Self { data: RefCell::new(BTreeMap::new()) }
  }

  pub fn merge(&self, other: &Profiler) {
    let mut data = self.data.borrow_mut();

    for (key, other_entry) in other.data.borrow().iter() {
      match data.get_mut(key) {
        None => {
          data.insert(key, other_entry.clone());
        },
        Some(entry) => {
          entry.count += other_entry.count;
          entry.duration += other_entry.duration;
        }
      }
    }
  }

  pub fn start_profiling(&self, id: &'static str) -> ProfilingContext {
    ProfilingContext { id, begin: Instant::now() }
  }

  pub fn stop_profiling(&self, ctx: ProfilingContext) {/*
    let mut data = self.data.borrow_mut();
    let duration = ctx.begin.elapsed();

    match data.get_mut(ctx.id) {
      None => {
        data.insert(ctx.id, ProfilingEntry { count: 1, duration });
      },
      Some(entry) => {
        entry.count += 1;
        entry.duration += duration;
      }
    }*/
  }

  pub fn show(&self) {
    println!("Function,Calls,Duration");

    for (key, entry) in self.data.borrow().iter() {
      println!("{},{},{:?}", key, entry.count, entry.duration);
    }
  }
}
