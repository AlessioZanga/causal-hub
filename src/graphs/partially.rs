use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::FusedIterator,
};

use serde::{Deserialize, Serialize};

pub trait PartiallyGraph {}
