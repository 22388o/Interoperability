pub struct ClientConfig<Block: BlockT> {
    pub offchain_worker_enabled: bool,
    pub offchain_indexing_api: bool,
    pub wasm_runtime_overrides: Option<PathBuf>,
    pub no_genesis: bool,
    pub wasm_runtime_substitutes: HashMap<Block::Hash, Vec<u8>>,
  
  offchain_worker_enabled: bool
Enable the offchain worker db.

offchain_indexing_api: bool
If true, allows access from the runtime to write into offchain worker db.

wasm_runtime_overrides: Option<PathBuf>
Path where WASM files exist to override the on-chain WASM.

no_genesis: bool
Skip writing genesis state on first start.

wasm_runtime_substitutes: HashMap<Block::Hash, Vec<u8>>
Map of WASM runtime substitute starting at the child of the given block until the runtime version doesn’t match anymore.
  
}
  
