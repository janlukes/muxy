use std::path::Path;

use muxy_protocol::{
    ErrorCode, ProjectDescriptor, ServerPath, ServerSettingsDoc, validate_folder_path,
    validate_uploaded,
};

fn path(value: &str) -> ServerPath {
    ServerPath(value.as_bytes().to_vec())
}

#[cfg(windows)]
#[test]
fn windows_wire_paths_work_with_unmodified_unix_clients() {
    let mut root = Vec::new();
    minicbor::Encoder::new(&mut root)
        .bytes(b"/C:")
        .expect("drive root");
    assert_eq!(
        minicbor::decode::<ServerPath>(&root).expect("Windows root"),
        path(r"C:\")
    );
    for (native, wire) in [
        (r"C:\Users\dév\project", "/C:/Users/dév/project"),
        (r"\\nas\share\project", "/UNC/nas/share/project"),
        (r"\\?\C:\Users\dev", "/UNC/?/C:/Users/dev"),
        (r"/Users/dev/a\b", r"/Users/dev/a\b"),
        (r"a\b", r"a\b"),
    ] {
        let encoded = minicbor::to_vec(path(native)).expect("encode");
        // The stable Unix client reads ServerPath as an opaque byte string.
        let received = minicbor::Decoder::new(&encoded)
            .bytes()
            .expect("Unix decode");
        assert_eq!(received, wire.as_bytes());
        if path(native).is_absolute() {
            assert!(
                received.starts_with(b"/"),
                "stable project/folder validation"
            );
        }
        // A Unix client echoes these bytes when opening a folder or terminal.
        let mut echoed = Vec::new();
        minicbor::Encoder::new(&mut echoed)
            .bytes(received)
            .expect("Unix encode");
        let decoded: ServerPath = minicbor::decode(&echoed).expect("Windows decode");
        assert_eq!(decoded, path(native));
    }
}

const ABSOLUTE: [&str; 6] = [
    "/home/dev",
    "C:\\Users\\dev",
    "c:/Users/dev",
    "\\\\server\\share",
    "\\\\server\\share\\dev",
    "/",
];

const RELATIVE: [&str; 9] = [
    "",
    "dev",
    "C:",
    "C:dev",
    "1:\\dev",
    "\\dev",
    "\\\\server",
    "\\\\\\share",
    "\\\\server\\",
];

#[test]
fn unix_and_windows_absolute_paths_validate_on_every_build() {
    for value in ABSOLUTE {
        assert!(path(value).is_absolute(), "{value}");
        assert_eq!(validate_folder_path(&path(value)), Ok(()), "{value}");
        assert_eq!(validate_uploaded(Some(&path(value))), Ok(()), "{value}");
        let settings = ServerSettingsDoc {
            default_shell: Some(path(value)),
            history_budget_bytes: 0,
            shell_integration: false,
        };
        assert_eq!(settings.validate(), Ok(()), "{value}");
        assert_eq!(
            ProjectDescriptor::new(path(value), []).validate(),
            Ok(()),
            "{value}"
        );
    }
    for value in RELATIVE {
        assert!(!path(value).is_absolute(), "{value}");
        assert_eq!(
            validate_folder_path(&path(value)),
            Err(ErrorCode::BadPath),
            "{value}"
        );
        assert_eq!(
            validate_uploaded(Some(&path(value))),
            Err(ErrorCode::BadPath),
            "{value}"
        );
    }
}

#[test]
fn new_projects_are_named_after_the_last_component_of_either_separator() {
    for (directory, name) in [
        ("/home/dev/app/", "app"),
        ("C:\\Users\\dev\\app", "app"),
        ("C:\\Users\\dev\\app\\", "app"),
        ("C:/Users/dev/app", "app"),
        ("\\\\server\\share\\app", "app"),
    ] {
        assert_eq!(ProjectDescriptor::new(path(directory), []).name, name);
    }
}

#[test]
fn local_paths_round_trip_through_the_wire_form() {
    let local = if cfg!(windows) {
        Path::new("C:\\Users\\dév\\app")
    } else {
        Path::new("/home/dév/app")
    };
    let wire = ServerPath::from_path(local);
    assert_eq!(wire, Ok(path(local.to_str().unwrap_or_default())));
    assert_eq!(wire.and_then(|wire| wire.to_path_buf()), Ok(local.into()));
}

#[cfg(unix)]
#[test]
fn unix_paths_keep_bytes_that_are_not_utf8() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let local = Path::new(OsStr::from_bytes(b"/tmp/\xff"));
    let wire = ServerPath::from_path(local);
    assert_eq!(wire, Ok(ServerPath(b"/tmp/\xff".to_vec())));
    assert_eq!(wire.and_then(|wire| wire.to_path_buf()), Ok(local.into()));
}

#[cfg(windows)]
#[test]
fn windows_refuses_paths_that_are_not_unicode() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    let unpaired_surrogate = OsString::from_wide(&[u16::from(b'C'), u16::from(b':'), 0xD800]);
    assert_eq!(
        ServerPath::from_path(Path::new(&unpaired_surrogate)),
        Err(ErrorCode::BadPath)
    );
    assert_eq!(
        ServerPath(b"C:\\\xff".to_vec()).to_path_buf(),
        Err(ErrorCode::BadPath)
    );
}
