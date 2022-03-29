

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the transaction payment pallet.

use codec::{Codec, Decode};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, MaybeDisplay},
};
use std::sync::Arc;
use pallet_template_rpc_runtime_api::SumStorageApi as SumStorageRuntimeApi;
use pallet_template::Store;
use pallet_template::Student;
use pallet_template::StudentAccount;
#[rpc]
pub trait SumStorageApi<BlockHash, Balance, Account>  
where Balance: codec::Codec + std::fmt::Display +std::str::FromStr {
	#[rpc(name = "sumStorage_get")]
    fn get_sum(&self, at: Option<BlockHash>) -> Result<u32>;
	#[rpc(name ="template_getStore")]
	fn get_store(&self, at: Option<BlockHash>) -> Result<Store>;
	#[rpc(name ="template_getStudent")]
	fn get_student(&self, at: Option<BlockHash>) -> Result<Student<Balance>>;

	#[rpc(name ="template_getAccount")]
	fn get_student_account(&self, at: Option<BlockHash>) -> Result<StudentAccount<Account>>;


}

/// A struct that implements the [`TransactionPaymentApi`].
pub struct SumStorage<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> SumStorage<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i64 {
	fn from(e: Error) -> i64 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, Balance,Account> SumStorageApi<<Block as BlockT>::Hash, Balance, Account>
	for SumStorage<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: SumStorageRuntimeApi<Block, Balance, Account>,
	Balance : codec::Codec + std::fmt::Display +std::str::FromStr,
	pallet_template::Student<Balance> : sp_api::Decode,
	pallet_template::StudentAccount<Account>:sp_api::Decode,
{
	fn get_sum(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> Result<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        
        let result_api = api.get_sum(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

	fn get_store(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Store> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        
        let result_api = api.get_store(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

	fn get_student(&self, at: Option<<Block as BlockT>::Hash>) -> Result<Student<Balance>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        
        let result_api = api.get_student(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}

	fn get_student_account(&self, at: Option<<Block as BlockT>::Hash>) -> Result<StudentAccount<Account>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));
        
        let result_api = api.get_student_account(&at);

		result_api.map_err(|e| RpcError {
			code: ErrorCode::ServerError(Error::RuntimeError.into()),
			message: "Unable to query dispatch info.".into(),
			data: Some(e.to_string().into()),
		})
	}
}