pub struct BuildNetworkParams<'a, TBl: BlockT, TExPool, TImpQu, TCl> {
    pub config: &'a Configuration,
    pub client: Arc<TCl>,
    pub transaction_pool: Arc<TExPool>,
    pub spawn_handle: SpawnTaskHandle,
    pub import_queue: TImpQu,
    pub block_announce_validator_builder: Option<Box<dyn FnOnce(Arc<TCl>) -> Box<dyn BlockAnnounceValidator<TBl> + Send> + Send>>,
    pub warp_sync: Option<Arc<dyn WarpSyncProvider<TBl>>>,
}
