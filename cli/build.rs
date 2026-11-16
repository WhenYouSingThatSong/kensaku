//! Windows application manifest for `kensaku-daemon`.
//!
//! The indexer pulls the Tauri runtime in, which imports `TaskDialogIndirect`
//! from `comctl32.dll`. That export only exists in Common Controls v6; without
//! a manifest the loader binds v5.82 and the process dies before `main` with
//! STATUS_ENTRYPOINT_NOT_FOUND (0xC0000139). `kensaku-cli` never reaches that
//! code, so only the daemon binary needs the dependency.
//!
//! `link.exe` splits `/MANIFESTINPUT:` values on spaces, so the manifest is
//! written to a space-free temp path; a checkout or `TEMP` that contains a
//! space gets a warning instead of a broken link.
const MANIFEST: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity type="win32" name="Kensaku.Daemon" version="1.0.0.0" processorArchitecture="*"/>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls"
                        version="6.0.0.0" processorArchitecture="*"
                        publicKeyToken="6595b64144ccf1df" language="*"/>
    </dependentAssembly>
  </dependency>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <!-- Windows 10/11 -->
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
    </application>
  </compatibility>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#;

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    let path = std::env::temp_dir().join("kensaku-daemon.manifest");
    if path.to_string_lossy().contains(' ') {
        println!(
            "cargo:warning=temp dir {:?} contains a space; kensaku-daemon will miss the Common Controls v6 manifest. \
             Set TEMP to a space-free directory or embed it with mt.exe.",
            path.parent().unwrap_or(&path)
        );
        return;
    }
    if let Err(error) = std::fs::write(&path, MANIFEST) {
        println!(
            "cargo:warning=could not write {}: {error}; kensaku-daemon will miss the Common Controls v6 manifest",
            path.display()
        );
        return;
    }

    println!("cargo:rustc-link-arg-bin=kensaku-daemon=/MANIFEST:EMBED");
    println!(
        "cargo:rustc-link-arg-bin=kensaku-daemon=/MANIFESTINPUT:{}",
        path.display()
    );
}
