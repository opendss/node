use thiserror::Error;

#[derive(Error, Debug)]
pub enum GossipError {
    #[error("unknown gossip error")]
    Unknown,
}
