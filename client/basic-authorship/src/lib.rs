// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Basic implementation of block-authoring logic.
//!
//! # Example
//!
//! ```no_run
//! # use sc_basic_authorship::ProposerFactory;
//! # use sp_consensus::{Environment, Proposer, RecordProof};
//! # use sp_runtime::generic::BlockId;
//! # use sc_keystore::Store;
//! # use std::{sync::Arc, time::Duration};
//! # use substrate_test_runtime_client::{
//! #     runtime::{Extrinsic, Transfer}, AccountKeyring,
//! #     DefaultTestClientBuilderExt, TestClientBuilderExt,
//! # };
//! # use sc_transaction_pool::{BasicPool, FullChainApi};
//! # let client = Arc::new(substrate_test_runtime_client::new());
//! # let spawner = sp_core::testing::TaskExecutor::new();
//! # let txpool = BasicPool::new_full(
//! #     Default::default(),
//! #     None,
//! #     spawner,
//! #     client.clone(),
//! # );
//! // The first step is to create a `ProposerFactory`.
//! let mut proposer_factory = ProposerFactory::new(client.clone(), txpool.clone(), None, Store::new_in_memory());
//!
//! // From this factory, we create a `Proposer`.
//! let proposer = proposer_factory.init(
//! 	&client.header(&BlockId::number(0)).unwrap().unwrap(),
//! );
//!
//! // The proposer is created asynchronously.
//! let proposer = futures::executor::block_on(proposer).unwrap();
//!
//! // This `Proposer` allows us to create a block proposition.
//! // The proposer will grab transactions from the transaction pool, and put them into the block.
//! let future = proposer.propose(
//! 	Default::default(),
//! 	Default::default(),
//! 	Duration::from_secs(2),
//! 	RecordProof::Yes,
//! );
//!
//! // We wait until the proposition is performed.
//! let block = futures::executor::block_on(future).unwrap();
//! println!("Generated block: {:?}", block.block);
//! ```

mod basic_authorship;

pub use crate::basic_authorship::{ProposerFactory, Proposer};
