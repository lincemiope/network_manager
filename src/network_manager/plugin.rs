//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.VPN.Plugin`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.VPN.Plugin.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
/*
TOOD:
use zbus::{Connection, Result, proxy};

impl PluginProxy<'_> {
    pub async fn new_from_path(device_path: zbus::zvariant::OwnedObjectPath, connection: &Connection) -> Result<PluginProxy<'_>> {
        PluginProxy::builder(connection)
        .path(device_path)?
        .build()
        .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/VPN/Plugin",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.VPN.Plugin",
    assume_defaults = true
)]
pub(crate) trait Plugin {
    /// Connect method
    fn connect(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<()>;

    /// ConnectInteractive method
    fn connect_interactive(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        details: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// NeedSecrets method
    fn need_secrets(
        &self,
        settings: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<String>;

    /// NewSecrets method
    fn new_secrets(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<()>;

    /// SetConfig method
    fn set_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetFailure method
    fn set_failure(&self, reason: &str) -> zbus::Result<()>;

    /// SetIp4Config method
    fn set_ip4_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetIp6Config method
    fn set_ip6_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Config signal
    #[zbus(signal)]
    fn config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Failure signal
    #[zbus(signal)]
    fn failure(&self, reason: u32) -> zbus::Result<()>;

    /// Ip4Config signal
    #[zbus(signal)]
    fn ip4_config(
        &self,
        ip4config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Ip6Config signal
    #[zbus(signal)]
    fn ip6_config(
        &self,
        ip6config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// LoginBanner signal
    #[zbus(signal)]
    fn login_banner(&self, banner: &str) -> zbus::Result<()>;

    /// SecretsRequired signal
    #[zbus(signal)]
    fn secrets_required(&self, message: &str, secrets: &[&str]) -> zbus::Result<()>;

    /// StateChanged signal
    #[zbus(signal, name="state_changed")]
    fn plugin_state_changed(&self, state: u32) -> zbus::Result<()>;

    /// State property
    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;
}
*/
