//! This file is executed during compilation.
//! It builds shell completion scripts and the man page
//!
//! Note: to see the eprintln messages, run cargo with
//!     cargo -vv build --release
use {
    clap::CommandFactory,
    clap_complete::{
        Generator,
        Shell,
    },
    dysk_cli::args::Args,
    serde::Deserialize,
    std::{
        env,
        ffi::OsStr,
        fs,
        path::PathBuf,
    },
};

fn write_completions_file<G: Generator + Copy, P: AsRef<OsStr>>(
    generator: G,
    out_dir: P,
) {
    let mut args = Args::command();
    clap_complete::generate_to(generator, &mut args, "dysk".to_string(), &out_dir)
        .expect("clap complete generation failed");
}

/// write the shell completion scripts which will be added to
/// the release archive
fn build_completion_scripts() {
    let out_dir = env::var_os("OUT_DIR").expect("out dir not set");
    write_completions_file(Shell::Bash, &out_dir);
    write_completions_file(Shell::Elvish, &out_dir);
    write_completions_file(Shell::Fish, &out_dir);
    write_completions_file(Shell::Zsh, &out_dir);
    eprintln!("completion scripts generated in {out_dir:?}");
}

/// generate the man page from the Clap configuration
fn build_man_page() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").expect("out dir not set");
    let out_dir = PathBuf::from(out_dir);
    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::<u8>::default();
    man.render(&mut buffer)?;
    let file_path = out_dir.join("dysk.1");
    std::fs::write(&file_path, buffer)?;
    eprintln!("map page generated in {file_path:?}");
    Ok(())
}

/// Check that all dysk versions are the same
///
/// See <https://github.com/Canop/dysk/issues/65>
fn check_version_consistency() {
    #[derive(Deserialize)]
    struct Package {
        version: String,
    }
    #[derive(Deserialize)]
    struct DependencyRef {
        version: String,
    }
    #[derive(Deserialize)]
    struct Dependencies {
        #[serde(alias = "dysk-cli")]
        dysk_cli: DependencyRef,
    }
    #[derive(Deserialize)]
    struct MainCargo {
        package: Package,
        dependencies: Dependencies,
        #[serde(alias = "build-dependencies")]
        build_dependencies: Dependencies,
    }
    #[derive(Deserialize)]
    struct CliCargo {
        package: Package,
    }
    let version = env::var("CARGO_PKG_VERSION").expect("cargo pkg version not available");
    let s = fs::read_to_string("Cargo.toml").unwrap();
    let main_cargo: MainCargo = toml::from_str(&s).unwrap();
    let Ok(s) = fs::read_to_string("cli/Cargo.toml") else {
        // won't be visible unless run with -vv
        eprintln!("No local cli/Cargo.toml -- Assuming a cargo publish compilation");
        return;
    };
    let cli_cargo: CliCargo = toml::from_str(&s).unwrap();
    let ok = (version == main_cargo.package.version)
        && (version == main_cargo.dependencies.dysk_cli.version)
        && (version == main_cargo.build_dependencies.dysk_cli.version)
        && (version == cli_cargo.package.version);
    if ok {
        eprintln!("Checked consistency of dysk and dysk-cli versions: OK");
    } else {
        panic!("VERSION MISMATCH - All dysk and dysk-cli versions must be the same");
    }
}

fn main() -> std::io::Result<()> {
    check_version_consistency();
    build_completion_scripts();
    build_man_page()?;
    Ok(())
}
