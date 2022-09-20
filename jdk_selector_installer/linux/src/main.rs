use std::fs::{copy, create_dir_all, File, OpenOptions, read_dir, remove_dir_all};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    match build_deb(AUTHOR, DESCRIPTION, VERSION) {
        Ok(..) => (),
        Err(e) => {
            println!("{}", e);
            println!("Debian package build failed.");
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

fn build_deb(author: &str, description: &str, version: &str) -> Result<ExitCode, Error> {
    let root_folder_path = PathBuf::from("./jdk_selector_installer/linux/.deb-build")
        .join(format!("jdk_selector-{}-noarch", version));
    let install_folder_path = &root_folder_path.join("usr/lib/jdk-selector");
    let debian_folder_path = &root_folder_path.join("DEBIAN");

    let _ = remove_dir_all(&root_folder_path);
    create_dir_all(&install_folder_path)?;
    create_dir_all(&debian_folder_path)?;
    copy_dir_all(Path::new("./.build"), &install_folder_path)?;

    File::create(&debian_folder_path.join("control"))?
        .write_all(format!(
            "{}{}{}{}{}",
            "Package: jdk-selector\n",
            format!("Version: {}\n", version),
            "Architecture: amd64\n", // TODO: Read output path to determine architecture
            format!("Maintainer: {}\n", author),
            format!("Description: {}\n", if description == "" { "N/A" } else { description }),
        ).as_bytes())?;

    #[cfg(unix)]
    set_permission_with_path(&root_folder_path)?;

    let postinst_path = &debian_folder_path.join("postinst");
    let mut postinst = create_file(postinst_path)?;
    postinst.write_all(format!(
        "{}{}",
        "ln -s /usr/lib/jdk-selector/java /usr/bin/java\n",
        "ln -s /usr/lib/jdk-selector/jdk_selector_cli /usr/bin/jdk_selector_cli\n",
    ).as_bytes())?;

    #[cfg(unix)]
    set_file_permission(&postinst, 0o755)?;

    let prerm_path = &debian_folder_path.join("prerm");
    let mut prerm = create_file(prerm_path)?;
    prerm.write_all(format!(
        "rm /usr/bin/java",
    ).as_bytes())?;

    #[cfg(unix)]
    set_file_permission(&prerm, 0o755)?;

    let mut _cmd = Command::new("dpkg-deb").args(&[
        "--build",
        "--root-owner-group",
        format!("jdk_selector-{}-noarch", version).as_str(),
    ]).current_dir(&root_folder_path.join("..")).spawn();

    let cmd = match &mut _cmd {
        Ok(value) => value,
        Err(e) => {
            return Err(Error::from(e.kind()));
        },
    };

    match cmd.wait() {
        Ok(..) => Ok(ExitCode::SUCCESS),
        Err(e) => {
            return Err(e);
        },
    }
}

fn copy_dir_all(src: &Path, dist: &Path) -> Result<(), Error> {
    create_dir_all(dist)?;
    for _entry in read_dir(src)? {
        let entry = _entry.unwrap();
        let entry_type = entry.file_type()?;

        if entry_type.is_dir() {
            copy_dir_all(&entry.path(), &dist.join(&entry.file_name()))?;
        } else if entry_type.is_symlink() {
            continue;
        } else {
            copy(&entry.path(), &dist.join(&entry.file_name()))?;
        }
    }

    Ok(())
}

#[cfg(not(unix))]
fn create_file(path: &Path) -> Result<File, Error> {
    OpenOptions::new().read(true).write(true).create_new(true).open(&path)
}

#[cfg(unix)]
fn create_file(path: &Path) -> Result<File, Error> {
    use std::os::unix::fs::OpenOptionsExt;

    OpenOptions::new().read(true).write(true).create_new(true).mode(0o644).open(&path)
}

#[cfg(unix)]
fn set_permission_with_path(path: &Path) -> Result<(), Error> {
    use std::fs::{Permissions, set_permissions};
    use std::os::unix::fs::PermissionsExt;

    for _entry in read_dir(path)? {
        let entry = _entry.unwrap();
        let entry_type = entry.file_type()?;

        if entry_type.is_symlink() {
            continue;
        }

        set_permissions(&entry.path(), Permissions::from_mode(0o755))?;
    }

    Ok(())
}

#[cfg(unix)]
fn set_file_permission(file: &File, permission: u32) -> Result<(), Error> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;

    file.set_permissions(Permissions::from_mode(permission))
}