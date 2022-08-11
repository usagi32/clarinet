pub mod digestion;
pub mod ingestion;
use orchestra_types::BlockIdentifier;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DigestingCommand {
    DigestSeedBlock(BlockIdentifier),
    Terminate,
}
