use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use tiny_http::{Response, Server};