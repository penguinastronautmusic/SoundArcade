//! Module used to separate the backend data from Bevy.
//! This was added in case Bevy needed to be changed later.

use std::time::Duration;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct StemResources {
    
}

#[derive(Resource, Clone, Debug)]
pub struct AppStartSelections {
    pub tick_len: Duration,
    pub is_dummy_backend: bool
}

