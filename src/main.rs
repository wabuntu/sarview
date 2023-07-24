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

fn exec(cmd: &str, args: &[&str]) -> String
{
   let output = Command::new(cmd)
   .args(args)
   .output()
   .expect("failed to execute cmd");

   // println!("status: {}", output.status);
   // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
   // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
   assert!(output.status.success());

   String::from_utf8(output.stdout).unwrap()
}


fn main() {

    // Get target node, port, user
    // let args = Args::parse();
    // let host: String = format!("{}:22", args.pos_arg);
    // let user: String = env::var("USER").expect("$USER is not set");

    let ret = exec("sh", &["-c", "for file in `ls /var/log/sysstat/* | \
                                            grep -E \"sa[0-9]+$\" | sort`; do echo $file; \
                                            LANG=C /usr/bin/sar -P ALL -C -f $file | \
                                            grep Average | tail -n 1; done; \
                                            "] );

    println!("exec: <{}>", ret);

}
