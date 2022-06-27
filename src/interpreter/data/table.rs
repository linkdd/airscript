use std::collections::HashMap;
use crate::interpreter::Cell;

pub(crate) type Dict<UD> = HashMap<String, Cell<UD>>;
pub(crate) type Array<UD> = Vec<Cell<UD>>;
