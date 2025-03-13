mod api;
mod handler;
mod header;
use crate::api::Api;
use std::io::Result;

fn main() -> Result<()> {
    let api = Api::new();
    api.start()
}
