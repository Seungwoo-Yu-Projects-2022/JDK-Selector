use std::fs::{copy, create_dir_all, File, OpenOptions, read_dir, remove_dir_all, remove_file};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

enum StrPtrOrString {
    StrPtr(&'static str),
    String(String),
}

fn main() -> ExitCode {
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    match build_deb(AUTHOR, DESCRIPTION, VERSION) {
        Ok(..) => {},
        Err(e) => {
            println!("{}", e);
            println!("Debian package build failed.");
            return ExitCode::FAILURE;
        }
    };

    match build_rpm(DESCRIPTION, VERSION) {
        Ok(..) => {},
        Err(e) => {
            println!("{}", e);
            println!("Redhat package build failed.");
            return ExitCode::FAILURE;
        }
    }

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
            "Architecture: amd64\n", // TODO: Read rust output path to determine architecture
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
        "{}{}",
        "rm /usr/bin/java\n",
        "rm /usr/bin/jdk_selector_cli\n",
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

fn build_rpm(description: &str, version: &str) -> Result<ExitCode, Error> {
    let root_folder_path = PathBuf::from("./jdk_selector_installer/linux/.rpm-build");
    let source_folder_path = &root_folder_path.join("SOURCES");
    let spec_folder_path = &root_folder_path.join("SPECS");
    let build_root_folder_path = &root_folder_path.join("BUILD");

    let spec_file_path = &spec_folder_path.join("jdk-selector.spec");

    let _ = remove_dir_all(&root_folder_path);
    create_dir_all(source_folder_path)?;
    create_dir_all(spec_folder_path)?;
    create_dir_all(build_root_folder_path)?;

    let absolute_root_folder_path = &root_folder_path.canonicalize()?;
    let absolute_spec_file_path = &absolute_root_folder_path.join("SPECS")
        .join("jdk-selector.spec");

    copy_dir_all(&Path::new(".build"), &build_root_folder_path)?;

    let mut _raw_script: Vec<StrPtrOrString> = vec![];
    _raw_script.push(StrPtrOrString::StrPtr("%define _binary_payload w3.zstdio"));
    _raw_script.push(StrPtrOrString::String(
        format!(
            "%define _topdir {}",
            &root_folder_path.canonicalize()?.to_string_lossy(),
        )
    ));
    _raw_script.push(StrPtrOrString::StrPtr("Name: jdk-selector"));
    _raw_script.push(StrPtrOrString::String(format!("Version: {}", version)));
    _raw_script.push(StrPtrOrString::StrPtr("Release: 1"));
    _raw_script.push(StrPtrOrString::String(
        format!(
            "Summary: {}",
            if description == "" { "N/A" } else { description.lines().nth(0).unwrap() },
        ),
    ));
    // TODO: Read rust output path to determine architecture
    _raw_script.push(StrPtrOrString::StrPtr("BuildArch: x86_64"));
    _raw_script.push(StrPtrOrString::StrPtr("License: ASL 2.0"));

    _raw_script.push(StrPtrOrString::StrPtr("%description"));
    _raw_script.push(StrPtrOrString::String(
        format!(
            "{}\n",
            if description == "" { "N/A" } else { description },
        ),
    ));

    _raw_script.push(StrPtrOrString::StrPtr("%install"));
    _raw_script.push(StrPtrOrString::StrPtr("rm -rf $RPM_BUILD_ROOT"));
    _raw_script.push(StrPtrOrString::StrPtr("mkdir -p $RPM_BUILD_ROOT/%{_libdir}/jdk-selector"));
    _raw_script.push(StrPtrOrString::StrPtr("cp -r $RPM_BUILD_DIR/. $RPM_BUILD_ROOT/%{_libdir}/jdk-selector\n"));

    _raw_script.push(StrPtrOrString::StrPtr("%clean"));
    _raw_script.push(StrPtrOrString::StrPtr("rm -rf $RPM_BUILD_ROOT\n"));

    _raw_script.push(StrPtrOrString::StrPtr("%files"));
    _raw_script.push(StrPtrOrString::StrPtr("%{_libdir}/jdk-selector/*"));

    _raw_script.push(StrPtrOrString::StrPtr("%post"));
    _raw_script.push(StrPtrOrString::StrPtr("chmod 755 -R %{_libdir}/jdk-selector"));
    _raw_script.push(StrPtrOrString::StrPtr("ln -s %{_libdir}/jdk-selector/java %{_bindir}/java"));
    _raw_script.push(StrPtrOrString::StrPtr("ln -s %{_libdir}/jdk-selector/jdk_selector_cli %{_bindir}/jdk_selector_cli\n"));

    _raw_script.push(StrPtrOrString::StrPtr("%preun"));
    _raw_script.push(StrPtrOrString::StrPtr("rm -rf %{_libdir}/jdk-selector"));
    _raw_script.push(StrPtrOrString::StrPtr("rm %{_bindir}/java"));
    _raw_script.push(StrPtrOrString::StrPtr("rm %{_bindir}/jdk_selector_cli"));

    let raw_script: Vec<String> = _raw_script.iter().map(| value | {
        match value {
            StrPtrOrString::StrPtr(value) => value.to_string(),
            StrPtrOrString::String(value) => value.to_string(),
        }
    }).collect();

    File::create(spec_file_path)?.write_all(&raw_script.join("\n").as_bytes())?;

    let mut _build_package_cmd = Command::new("rpmbuild").args(&[
        "-ba",
        &absolute_spec_file_path.to_string_lossy(),
    ]).current_dir(&root_folder_path).spawn();

    let build_package_cmd = match &mut _build_package_cmd {
        Ok(value) => value,
        Err(e) => {
            return Err(Error::from(e.kind()));
        },
    };

    match build_package_cmd.wait() {
        Ok(..) => {},
        Err(e) => {
            return Err(e);
        },
    }

    Ok(ExitCode::SUCCESS)
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