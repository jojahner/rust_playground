extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;

static USAGE: &'static str = "
Tasks

Usage:
  rdo [--file FILE] list
  rdo [--file FILE] add <task>...
  rdo [--file FILE] (remove | check | uncheck) <task_id>
  rdo -h | --help
  rdo --version

Options:
  -h, --help        Show this screen.
  --version     Show version.
  -f, --file FILE   Specify filename [default: ~/.rdo_tasks].
";

#[derive(RustcDecodable)]
struct Args {
    arg_task: Vec<String>,
    arg_task_id: i32,
    arg_file: String,
    flag_version: bool,
}

fn main() {
    // let args = Docopt::new(USAGE)
    //                   .and_then(|dopt| dopt.parse())
    //                   .unwrap_or_else(|e| e.exit());
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.argv(argv().into_iter()).decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    // You can conveniently access values with `get_{bool,count,str,vec}`
    // functions. If the key doesn't exist (or if, e.g., you use `get_str` on
    // a switch), then a sensible default value is returned.
    println!("\nSome values:");
    println!("  filename: {}", args.get_str("--file"));
    println!("  task_id: {:?}", args.get_str("<task_id>"));
    println!("  task: {:?}", args.get_vec("<task>"));
}
