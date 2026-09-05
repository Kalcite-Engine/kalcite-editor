//! Distribution metadata builder for Kalcite Editor.
//!
//! It emits freedesktop metadata for Linux and a self-contained `.app` bundle
//! for macOS. Keeping this in the editor repository makes file associations a
//! versioned part of every release instead of an installer-specific detail.

use std::{env, fs, io, path::Path, process::ExitCode};

const DESKTOP_ENTRY: &str = "[Desktop Entry]\n\
Type=Application\n\
Name=Kalcite Editor\n\
Comment=Native graphical editor for Kalcite projects\n\
Exec=kalcite-editor %F\n\
TryExec=kalcite-editor\n\
Terminal=false\n\
Categories=Development;IDE;\n\
MimeType=application/x-kalcite-project;application/x-kalcite-scene;application/x-kalcite-script;\n\
StartupNotify=true\n";

const MIME_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/x-kalcite-project">
    <comment>Kalcite project manifest</comment>
    <glob pattern="kalcite.toml"/>
  </mime-type>
  <mime-type type="application/x-kalcite-scene">
    <comment>Kalcite scene</comment>
    <glob pattern="*.kscn"/>
  </mime-type>
  <mime-type type="application/x-kalcite-script">
    <comment>Kalcite script</comment>
    <glob pattern="*.klc"/>
  </mime-type>
</mime-info>
"#;

const INFO_PLIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key><string>en</string>
  <key>CFBundleDisplayName</key><string>Kalcite Editor</string>
  <key>CFBundleExecutable</key><string>kalcite-editor</string>
  <key>CFBundleIdentifier</key><string>org.kalcite.editor</string>
  <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
  <key>CFBundleName</key><string>Kalcite Editor</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0.14.0</string>
  <key>CFBundleVersion</key><string>0.14.0</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>CFBundleDocumentTypes</key>
  <array>
    <dict>
      <key>CFBundleTypeName</key><string>Kalcite Project</string>
      <key>CFBundleTypeRole</key><string>Editor</string>
      <key>LSHandlerRank</key><string>Owner</string>
      <key>LSItemContentTypes</key><array><string>org.kalcite.project</string></array>
    </dict>
    <dict>
      <key>CFBundleTypeName</key><string>Kalcite Scene</string>
      <key>CFBundleTypeRole</key><string>Editor</string>
      <key>LSHandlerRank</key><string>Owner</string>
      <key>LSItemContentTypes</key><array><string>org.kalcite.scene</string></array>
    </dict>
    <dict>
      <key>CFBundleTypeName</key><string>Kalcite Script</string>
      <key>CFBundleTypeRole</key><string>Editor</string>
      <key>LSHandlerRank</key><string>Owner</string>
      <key>LSItemContentTypes</key><array><string>org.kalcite.script</string></array>
    </dict>
  </array>
  <key>UTExportedTypeDeclarations</key>
  <array>
    <dict>
      <key>UTTypeIdentifier</key><string>org.kalcite.project</string>
      <key>UTTypeDescription</key><string>Kalcite Project</string>
      <key>UTTypeConformsTo</key><array><string>public.data</string></array>
      <key>UTTypeTagSpecification</key><dict><key>public.filename-extension</key><array><string>kalcite</string></array></dict>
    </dict>
    <dict>
      <key>UTTypeIdentifier</key><string>org.kalcite.scene</string>
      <key>UTTypeDescription</key><string>Kalcite Scene</string>
      <key>UTTypeConformsTo</key><array><string>public.text</string></array>
      <key>UTTypeTagSpecification</key><dict><key>public.filename-extension</key><array><string>kscn</string></array></dict>
    </dict>
    <dict>
      <key>UTTypeIdentifier</key><string>org.kalcite.script</string>
      <key>UTTypeDescription</key><string>Kalcite Script</string>
      <key>UTTypeConformsTo</key><array><string>public.source-code</string></array>
      <key>UTTypeTagSpecification</key><dict><key>public.filename-extension</key><array><string>klc</string></array></dict>
    </dict>
  </array>
</dict>
</plist>
"#;

fn usage() {
    eprintln!(
        "usage:\n  kalcite-editor-info linux <prefix>\n  kalcite-editor-info macos <editor-binary> <Kalcite Editor.app>"
    );
}

fn write_linux(prefix: &Path) -> io::Result<()> {
    let applications = prefix.join("share/applications/kalcite-editor.desktop");
    let mime = prefix.join("share/mime/packages/kalcite-editor.xml");
    write_file(&applications, DESKTOP_ENTRY)?;
    write_file(&mime, MIME_TYPES)
}

fn write_macos(binary: &Path, app: &Path) -> io::Result<()> {
    if !binary.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("editor binary does not exist: {}", binary.display()),
        ));
    }
    let contents = app.join("Contents");
    let executable = contents.join("MacOS/kalcite-editor");
    write_file(&contents.join("Info.plist"), INFO_PLIST)?;
    fs::create_dir_all(executable.parent().expect("MacOS parent"))?;
    fs::copy(binary, &executable)?;
    set_executable(&executable)
}

fn write_file(path: &Path, contents: &str) -> io::Result<()> {
    fs::create_dir_all(path.parent().expect("metadata file parent"))?;
    fs::write(path, contents)
}

#[cfg(unix)]
fn set_executable(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn set_executable(_: &Path) -> io::Result<()> {
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().collect();
    let result = match args.as_slice() {
        [_, command, prefix] if command == "linux" => write_linux(Path::new(prefix)),
        [_, command, binary, app] if command == "macos" => {
            write_macos(Path::new(binary), Path::new(app))
        }
        _ => {
            usage();
            return ExitCode::FAILURE;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("kalcite-editor-info: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temporary_path(name: &str) -> PathBuf {
        let unique = format!("kalcite-editor-info-{name}-{}", std::process::id());
        env::temp_dir().join(unique)
    }

    #[test]
    fn linux_metadata_declares_all_editor_file_types() {
        let root = temporary_path("linux");
        write_linux(&root).unwrap();
        let desktop =
            fs::read_to_string(root.join("share/applications/kalcite-editor.desktop")).unwrap();
        let mime = fs::read_to_string(root.join("share/mime/packages/kalcite-editor.xml")).unwrap();
        assert!(desktop.contains("application/x-kalcite-project"));
        assert!(desktop.contains("application/x-kalcite-scene"));
        assert!(desktop.contains("application/x-kalcite-script"));
        assert!(mime.contains("*.kscn"));
        assert!(mime.contains("*.klc"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn checked_in_desktop_entry_matches_the_builder() {
        assert_eq!(
            include_str!("../resources/kalcite-editor.desktop"),
            DESKTOP_ENTRY
        );
    }

    #[test]
    fn macos_bundle_contains_document_associations() {
        let root = temporary_path("macos");
        let binary = root.join("kalcite-editor");
        fs::create_dir_all(&root).unwrap();
        fs::write(&binary, b"editor").unwrap();
        let app = root.join("Kalcite Editor.app");
        write_macos(&binary, &app).unwrap();
        let plist = fs::read_to_string(app.join("Contents/Info.plist")).unwrap();
        assert!(plist.contains("org.kalcite.project"));
        assert!(plist.contains("org.kalcite.scene"));
        assert!(plist.contains("org.kalcite.script"));
        assert!(app.join("Contents/MacOS/kalcite-editor").is_file());
        fs::remove_dir_all(root).unwrap();
    }
}
