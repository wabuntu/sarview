//! saview
//! # saview
//!
//! saview prints monthly statistic graph of sysstat data.

use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "host", help = "Target compute node name, or IP address")]
    pos_arg: String,
}

fn main() {

    // Get target node, port, user
    // let args = Args::parse();
    // let host: String = format!("{}:22", args.pos_arg);
    // let user: String = env::var("USER").expect("$USER is not set");

    let output = Command::new("for file in `ls /var/log/sysstat/* | \
                                                grep -E \"sa[0-9]+$\" | sort`; do echo $file; sar -P ALL -C -f $file | \
                                                grep Average | tail -n 1; done; \
                                                ")
                                    .output()
                                    .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

}
