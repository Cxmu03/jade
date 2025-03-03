pub mod common;
pub mod emulators;

use crate::common::{traits::*, types::*};
use jade_programs::*;

pub fn validate<G: Generator, V: Validator, P: JadeProgram>(generator: G, validator: V) {}
