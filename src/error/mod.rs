pub use snafu::prelude::*;
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GwError {
    #[snafu(display("There was an error with the file: {}", source))]
    Ars { source: ArsError },

    #[snafu(display("Could not read file: {}", source))]
    Read { source: std::io::Error },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ArsError {
    #[snafu(display("Could not found upstream server, id={}", id))]
    NoUpstream { id: u32 },
}
