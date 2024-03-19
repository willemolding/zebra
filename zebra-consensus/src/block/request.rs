//! Block verifier request type.

use std::sync::Arc;

use zebra_chain::block::Block;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A request to the chain or block verifier
pub enum Request {
    /// Performs semantic validation, then asks the state to perform contextual validation and commit the block
    Commit(Arc<Block>),

    #[cfg(feature = "getblocktemplate-rpcs")]
    /// Performs semantic validation but skips checking proof of work,
    /// then asks the state to perform contextual validation.
    /// Does not commit the block to the state.
    CheckProposal(Arc<Block>),

    /// Performs partial semantic validation but skips:
    ///     - checking proof of work
    /// It then asks the state to do contextual validation and commits the block
    TinyCash(Arc<Block>),
}

impl Request {
    /// Returns inner block
    pub fn block(&self) -> Arc<Block> {
        Arc::clone(match self {
            Request::Commit(block) => block,

            #[cfg(feature = "getblocktemplate-rpcs")]
            Request::CheckProposal(block) => block,
            Request::TinyCash(block) => block,
        })
    }

    /// Returns `true` if the request is a proposal
    pub fn is_proposal(&self) -> bool {
        match self {
            Request::Commit(_) => false,

            #[cfg(feature = "getblocktemplate-rpcs")]
            Request::CheckProposal(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the request is a tinycash request
    pub fn is_tinycash(&self) -> bool {
        match self {
            Request::TinyCash(_) => true,
            _ => false,
        }
    }
}
