use tauri_build::WindowsAttributes;

// v2: build plugins
// https://github.com/tauri-apps/tauri/blob/dev/examples/api/src-tauri/build.rs#L9-L12

fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .windows_attributes(WindowsAttributes::new_without_app_manifest())
            .plugin(
                "bridge",
                tauri_build::InlinedPlugin::new().commands(&["connect"]),
            )
            .app_manifest(tauri_build::AppManifest::new().commands(&[
                "log_operation",
                "perform_request",
                "echo",
            ])),
    )
    .expect("failed to run tauri-build");
}
