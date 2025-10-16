## bevy_editor_assets

Backend asset discovery utilities for the modular Bevy editor. The crate keeps
the asset browser logic headless so UI layers can choose how to visualise data.

### Features

- `AssetBrowser` resource tracks discovered textures, selection, and metadata.
- `AssetBrowserPlugin` wires the resource plus the automatic scanning system.
- `AssetBrowserCorePlugin` registers the resource without scheduling the scan,
  useful for custom ordering or test harnesses.
- `TextureHandleProvider` trait lets consumers decide how image handles are
  produced (defaulting to Bevy's `AssetServer`). Tests and alternate frontends
  can supply lightweight fakes.

### Example

```rust
use bevy::prelude::*;
use bevy_editor_assets::AssetBrowserPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetBrowserPlugin)
        .run();
}
```

To coordinate with project management, set the asset directory once you know the
active project root and flip the `needs_rescan` flag. The helper system added in
`bevy_editor::project_browser::sync_asset_browser_root` demonstrates one way to
approach this integration.

### Rescan & File Watching Strategy

- The backend keeps file discovery deterministic through explicit rescans.
  Update `AssetBrowser::assets_directory`, set `needs_rescan = true`, and let the
  scheduled scan run in `AssetBrowserSet`.
- When running inside the editor shell we rely on project events to trigger the
  rescan. Automatic OS file watching will live in a follow-up helper crate that
  wraps `notify` so backends remain headless-friendly.
- The `TextureHandleProvider` abstraction means a future watcher can enqueue
  background refreshes without needing a real `AssetServer` handle until the UI
  asks for it.

### Testing

`cargo test -p bevy_editor_assets` executes smoke tests that stage a temporary
assets directory, perform scans, and validate the resource state without
touching Bevy's runtime asset loader.
