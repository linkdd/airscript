use crate::prelude::*;
use crate::interpreter::*;

pub fn to_string<UD>(vm: &mut VM<UD>) -> Result<usize> where UD: Clone + 'static {
  let cell = vm.pop_cell()?;

  let s = match cell {
    Cell::Nil => "nil".to_string(),
    Cell::Boolean(val) => {
      format!("{val}")
    },
    Cell::Integer(val) => {
      format!("{val}")
    },
    Cell::Float(val) => {
      format!("{val}")
    },
    Cell::String(val) => {
      val
    },
    Cell::UserData(val) => {
      format!("[userdata {:p}]", &val)
    },
    Cell::Dict(val) => {
      format!("[dict {:p}]", &val)
    },
    Cell::Array(val) => {
      format!("[array {:p}]", &val)
    },
    Cell::RustFunction(func) => {
      format!("[rust-function {:p}]", &func)
    },
    Cell::AirFunction(func) => {
      format!("[air-function {:p}]", &func)
    },
  };

  vm.push_string(s);
  Ok(1)
}

pub fn print<UD>(vm: &mut VM<UD>) -> Result<usize> where UD: Clone + 'static {
  let msg = vm.pop_string()?;
  print!("{msg}");
  Ok(0)
}
