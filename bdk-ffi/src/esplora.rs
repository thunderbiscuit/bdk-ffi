use std::ops::{Deref};
use crate::error::EsploraError;
use crate::wallet::{Update};
use crate::bitcoin::Transaction;
use crate::types::FullScanRequest;

use bdk::bitcoin::Transaction as BdkTransaction;
use bdk::chain::spk_client::FullScanResult as BdkFullScanResult;
use bdk_esplora::esplora_client::{BlockingClient, Builder};
use bdk_esplora::EsploraExt;

use std::sync::Arc;
use bdk::KeychainKind;

pub struct EsploraClient(BlockingClient);

impl EsploraClient {
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking();
        Self(client)
    }

    pub fn full_scan(
        &self,
        request: Arc<FullScanRequest>,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<Arc<Update>, EsploraError> {

        // let previous_tip = wallet.latest_checkpoint();
        // let keychain_spks = wallet.all_unbounded_spk_iters().into_iter().collect();

        // let (update_graph, last_active_indices) = self
        //     .0
        //     .full_scan(keychain_spks, stop_gap as usize, parallel_requests as usize)
        //     .map_err(|e| EsploraError::from(*e))?;
        //
        // let missing_heights = update_graph.missing_heights(wallet.local_chain());
        // let chain_update = self
        //     .0
        //     .update_local_chain(previous_tip, missing_heights)
        //     .map_err(|e| EsploraError::from(*e))?;
        //
        // let update = BdkUpdate {
        //     last_active_indices,
        //     graph: update_graph,
        //     chain: Some(chain_update),
        // };

        // let update = self.0.full_scan(
        //     request.0.clone(),
        //     stop_gap as usize,
        //     parallel_requests as usize,
        // );

        let result: BdkFullScanResult<KeychainKind> = self.0.full_scan(
            // throw appropriate error if request is empty
            // request.0.take().expect("Request is empty"),
            request.0.lock().unwrap().deref(),
            // request.0.lock().unwrap().deref().take(),
            stop_gap as usize,
            parallel_requests as usize,
        )?;

        let update = bdk::wallet::Update {
            last_active_indices: result.last_active_indices,
            graph: result.graph_update,
            chain: Some(result.chain_update),
        };

        Ok(Arc::new(Update(update)))
    }

    // pub fn sync();

    pub fn broadcast(&self, transaction: &Transaction) -> Result<(), EsploraError> {
        let bdk_transaction: BdkTransaction = transaction.into();
        self.0
            .broadcast(&bdk_transaction)
            .map_err(EsploraError::from)
    }
}
