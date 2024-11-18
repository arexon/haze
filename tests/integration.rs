use std::{env, fs, path::PathBuf};

use fs_extra::dir::{self, CopyOptions};
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin, Command};

const COM_MOJANG: &str = "com.mojang";
const MINECRAFT_WORLDS: &str = "minecraftWorlds";

macro_rules! fn_name {
    () => {{
        // ¯\_(ツ)_/¯
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
            .split("::")
            .enumerate()
            .find(|(i, _)| *i == 1)
            .map(|(_, s)| s)
            .unwrap()
    }};
}

struct HazeTest {
    temp_dir: PathBuf,
    command: Command,
}

impl HazeTest {
    fn new<'a>(
        name: &'static str,
        args: impl IntoIterator<Item = &'a str>,
        com_mojang: Option<&str>,
    ) -> Self {
        let temp_dir = env::temp_dir().join(name);
        fs::create_dir(&temp_dir).expect("should create temp dir");
        let testdata = env::current_dir()
            .unwrap()
            .join("tests")
            .join("testdata")
            .join(name);
        let options = CopyOptions::new().content_only(true);
        dir::copy(&testdata, &temp_dir, &options).expect("should copy testdata to temp dir");

        std::fs::read_dir(&testdata).unwrap();

        let mut command = Command::new(get_cargo_bin("haze"));
        command.args(args).current_dir(&temp_dir);

        if let Some(com_mojang) = com_mojang {
            command.env("COM_MOJANG", com_mojang);
        }

        Self { temp_dir, command }
    }
}

impl Drop for HazeTest {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.temp_dir).unwrap();
    }
}

#[test]
#[cfg(unix)]
fn cannot_find_com_mojang_env_var() {
    let mut test = HazeTest::new(fn_name!(), ["export", "foo"], None);

    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: could not find the COM_MOJANG environment variable
      `-> environment variable not found
      help: setting this variable is required on non-Windows systems
    "#);
}

#[test]
fn com_mojang_does_not_exist() {
    let mut test = HazeTest::new(fn_name!(), ["export", "foo"], Some("other-com.mojang"));

    #[cfg(unix)]
    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: the `com.mojang` directory does not exist in `other-com.mojang/
      | minecraftWorlds`
    "#);

    #[cfg(windows)]
    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: the `com.mojang` directory does not exist in `other-
      | com.mojang\minecraftWorlds`
    "#);
}

#[test]
fn invalid_world_glob() {
    let mut test = HazeTest::new(fn_name!(), ["export", "foo"], Some(COM_MOJANG));

    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: invalid world glob pattern `./worlds/***`
      `-> Pattern syntax error near position 11: wildcards are either regular `*`
          or recursive `**`
    "#);
}

#[test]
fn local_world_name_conflict() {
    let mut test = HazeTest::new(fn_name!(), ["export", "foo"], Some(COM_MOJANG));

    #[cfg(unix)]
    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: two local worlds have conflicting names `worlds_other/foo` <-> `worlds/
      | foo`
      help: worlds in different directories must have unique names so they are
            easily identifiable
    "#);

    #[cfg(windows)]
    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: two local worlds have conflicting names `worlds_other\foo` <->
      | `worlds\foo`
      help: worlds in different directories must have unique names so they are
            easily identifiable
    "#);
}

#[test]
fn no_matching_local_worlds() {
    let mut test = HazeTest::new(fn_name!(), ["export", "foo"], Some(COM_MOJANG));

    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: no worlds matching `foo` were found
    "#);
}

#[test]
fn export_without_overwrite_allowed() {
    let mut test = HazeTest::new(fn_name!(), ["export", "foo"], Some(COM_MOJANG));

    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: attempting to export `foo` when one already exists in `com.mojang`
      help: use --overwrite to bypass
    "#);
}

#[test]
fn export() {
    let world_to_export = "foo";

    let mut test = HazeTest::new(fn_name!(), ["export", world_to_export], Some(COM_MOJANG));

    #[cfg(unix)]
    assert_cmd_snapshot!(test.command, @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: exported `worlds/foo` to `com.mojang/minecraftWorlds/foo`
    "#);

    #[cfg(windows)]
    assert_cmd_snapshot!(test.command, @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: exported `worlds\foo` to `com.mojang\minecraftWorlds\foo`
    "#);

    let exported_world = test
        .temp_dir
        .join(COM_MOJANG)
        .join(MINECRAFT_WORLDS)
        .join(world_to_export);
    assert!(
        exported_world.exists(),
        "expected world `{}` to have been exported",
        exported_world.display()
    );
}

#[test]
fn export_with_overwrite() {
    let mut test = HazeTest::new(
        fn_name!(),
        ["export", "--overwrite", "foo"],
        Some(COM_MOJANG),
    );

    #[cfg(unix)]
    assert_cmd_snapshot!(test.command, @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: exported `worlds/foo` to `com.mojang/minecraftWorlds/foo`
    "#);

    #[cfg(windows)]
    assert_cmd_snapshot!(test.command, @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: exported `worlds\foo` to `com.mojang\minecraftWorlds\foo`
    "#);
}

#[test]
fn no_matching_com_mojang_worlds() {
    let mut test = HazeTest::new(fn_name!(), ["import", "foo"], Some(COM_MOJANG));

    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: no worlds matching `foo` were found
    "#);
}

#[test]
fn import_without_local_match() {
    let mut test = HazeTest::new(fn_name!(), ["import", "foo"], Some(COM_MOJANG));

    assert_cmd_snapshot!(test.command, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: attempting to import `foo` when there is no local world matching it
      help: worlds must be manually imported to a desired local location for
            first-time setup
    "#);
}

#[test]
fn import() {
    let world_to_import = "foo";
    let mut test = HazeTest::new(fn_name!(), ["import", world_to_import], Some(COM_MOJANG));

    #[cfg(unix)]
    assert_cmd_snapshot!(test.command, @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: imported `com.mojang/minecraftWorlds/foo` to `worlds/foo`
    "#);

    #[cfg(windows)]
    assert_cmd_snapshot!(test.command, @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    info: imported `com.mojang\minecraftWorlds\foo` to `worlds\foo`
    "#);

    let imported_world = test.temp_dir.join("worlds").join(world_to_import);
    assert!(
        imported_world.exists(),
        "expected world `{}` to have been imported",
        imported_world.display()
    );
}
