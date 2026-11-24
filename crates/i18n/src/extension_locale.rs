use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use extension::{ExtensionHostProxy, ExtensionLocaleProxy};
use gpui::App;

use crate::{LocaleId, LocaleRegistry};

/// Initializes `crates/i18n` and connects [`LocaleRegistry`] to [`ExtensionHostProxy`].
pub fn init(cx: &mut App) {
    let registry = LocaleRegistry::default_global(cx);
    let proxy = ExtensionHostProxy::default_global(cx);
    proxy.register_locale_proxy(LocaleRegistryProxy { registry });
}

struct LocaleRegistryProxy {
    registry: Arc<LocaleRegistry>,
}

impl ExtensionLocaleProxy for LocaleRegistryProxy {
    fn register_locale_pack(&self, extension_id: Arc<str>, locale: Arc<str>, locale_path: PathBuf) -> Result<()> {
        self.registry
            .register_locale_pack_file(extension_id, LocaleId::new(locale.as_ref()), locale_path)
    }

    fn unregister_locale_pack(&self, extension_id: &str) {
        self.registry.unregister_locale_pack(extension_id);
    }
}
