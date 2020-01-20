use crate::btest;
use biome_butterfly::{client::Client,
                        rumor::{ConstIdRumor as _,
                                ServiceConfig}};
use biome_core::service::ServiceGroup;

#[test]
fn two_members_share_service_config() {
    let mut net = btest::SwimNet::new_rhw(2);
    net.mesh_mlw_smr();
    net.add_service_config(0, "witcher", "tcp-backlog = 128");
    net.wait_for_gossip_rounds(1);
    assert!(net[1].service_config_store
                  .lock_rsr()
                  .service_group("witcher.prod")
                  .contains_id(ServiceConfig::const_id()));
}

#[test]
fn service_config_via_client() {
    let mut net = btest::SwimNet::new_rhw(2);
    net.mesh_mlw_smr();

    net.wait_for_gossip_rounds(1);
    let mut client =
        Client::new(&net[0].gossip_addr().to_string(), None).expect("Cannot create Butterfly \
                                                                     Client");
    let payload = b"I want to get lost in you, tokyo";
    client.send_service_config(ServiceGroup::new("witcher", "prod", None).unwrap(),
                               0,
                               payload,
                               false)
          .expect("Cannot send the service configuration");
    net.wait_for_gossip_rounds(1);
    assert!(net[1].service_config_store
                  .lock_rsr()
                  .service_group("witcher.prod")
                  .contains_id(ServiceConfig::const_id()));
}
