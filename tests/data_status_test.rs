extern crate plugin_launcher;

use plugin_launcher::plugin::api::modules::*;
use plugin_launcher::plugin::data::status::*;

#[test]
fn it_status_change_test() {
    let mut status = PluginManagerStatusData::new();

    assert_eq!(PluginStatusEnum::Inactive, status.get_plugin_status(&PluginId::new("unpublished-module", "unpublished-plugin")));
    assert_eq!(PluginStatusEnum::Inactive, status.get_plugin_status(&PluginId::new("module", "plugin")));

    status.set_plugin_status(&PluginId::new("module", "plugin"), PluginStatusEnum::Active);

    assert_eq!(PluginStatusEnum::Inactive, status.get_plugin_status(&PluginId::new("unpublished-module", "unpublished-plugin")));
    assert_eq!(PluginStatusEnum::Active, status.get_plugin_status(&PluginId::new("module", "plugin")));

    status.set_plugin_status(&PluginId::new("module", "plugin"), PluginStatusEnum::Inactive);

    assert_eq!(PluginStatusEnum::Inactive, status.get_plugin_status(&PluginId::new("unpublished-module", "unpublished-plugin")));
    assert_eq!(PluginStatusEnum::Inactive, status.get_plugin_status(&PluginId::new("module", "plugin")));
}
