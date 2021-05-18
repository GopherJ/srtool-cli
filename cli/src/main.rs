mod opts;
use clap::{crate_version, Clap};
use opts::*;
use srtool_lib::*;
use std::process::Command;
use std::{env, fs};

fn handle_exit() {
	println!("Killing srtool container, your build was not finished...");
	let cmd = format!("docker rm -f srtool");
	let _ = Command::new("sh").arg("-c").arg(cmd).spawn().expect("failed to execute cleaning process").wait();
	println!("Exiting");
	std::process::exit(0);
}

fn main() {
	let opts: Opts = Opts::parse();

	ctrlc::set_handler(move || {
		handle_exit();
	})
	.expect("Error setting Ctrl-C handler");

	match opts.subcmd {
		SubCommand::Build(build_opts) => {
			println!("Running srtool-cli v{}", crate_version!());
			let tag = fetch_image_tag();
			let path = fs::canonicalize(&build_opts.path).unwrap();
			let cmd = format!(
				"docker run --name srtool --rm -e PACKAGE={package} -v {dir}:/build -v {tmpdir}cargo:/cargo-home {image}:{tag}",
				package = build_opts.package,
				dir = path.display(),
				tmpdir = env::temp_dir().display(),
				image = "chevdor/srtool",
				tag = tag,
			);
			// println!("cmd = {:?}", cmd);

			if cfg!(target_os = "windows") {
				Command::new("cmd").args(&["/C", cmd.as_str()]).output().expect("failed to execute process");
			} else {
				let _res = Command::new("sh")
					.arg("-c")
					.arg(cmd)
					.spawn()
					.expect("failed to execute process")
					.wait_with_output();
			};
		}
	};
}

#[cfg(test)]
mod test {
	use assert_cmd::Command;

	#[test]
	#[ignore = "assert_cmd bug, see https://github.com/assert-rs/assert_cmd/issues/117"]
	fn it_shows_help() {
		let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
		let assert = cmd.arg("--help").assert();
		assert.success().code(0);
	}
}