use ipfs_embed::{generate_keypair, Config, Ipfs};
use libipld::store::DefaultParams;

#[async_std::main]
async fn main() {

    let mut config = Config::new("out".as_ref(), generate_keypair());
    config.network.kad = None;
    let a = Ipfs::<DefaultParams>::new(config).await.unwrap();
}
