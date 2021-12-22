use std::cell::RefCell;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use cidr::{Ipv4Cidr, Ipv6Cidr};

use crate::config::factory::*;
use crate::config::*;

thread_local! {
    /// To be instantiated by a VPN entrypoint after parsing.
    pub static ON_VPNTUN: RefCell<Option<Box<dyn FnOnce(&VpnTunFactory) -> Arc<dyn crate::flow::Tun>>>> = RefCell::new(None);
}

#[derive(Clone, Deserialize)]
pub struct VpnTunFactory<'a> {
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
    pub ipv4_route: Vec<Ipv4Cidr>,
    pub ipv6_route: Vec<Ipv6Cidr>,
    pub dns: Vec<IpAddr>,
    pub web_proxy: Option<&'a str>,
}

impl<'de> VpnTunFactory<'de> {
    pub(in super::super) fn parse(plugin: &'de Plugin) -> ConfigResult<ParsedPlugin<'de, Self>> {
        let Plugin { name, param, .. } = plugin;
        let config: Self =
            parse_param(param).ok_or_else(|| ConfigError::ParseParam(name.to_string()))?;
        Ok(ParsedPlugin {
            factory: config,
            requires: vec![],
            provides: vec![Descriptor {
                descriptor: name.to_string() + ".tun",
                r#type: AccessPointType::TUN,
            }],
        })
    }
}
impl<'de> Factory for VpnTunFactory<'de> {
    fn load(&mut self, plugin_name: String, set: &mut PartialPluginSet) -> LoadResult<()> {
        let tun = (ON_VPNTUN.with(|cb| cb.borrow_mut().take())).ok_or_else(|| {
            ConfigError::TooManyPlugin {
                plugin: plugin_name.clone(),
                r#type: "vpn-tun",
            }
        })?(self);
        set.fully_constructed.tun.insert(plugin_name, tun);
        Ok(())
    }
}