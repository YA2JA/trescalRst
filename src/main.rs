use std::fs::File;

use clap:: Parser;

mod gpib;
mod procedure;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 22)]
    multi: u8,

    #[arg(long, default_value_t = 24)]
    power: u8,

    #[arg(short, long, default_value_t = format!("E3632A"))]
    procedure: String,
}

fn main()
{
    File::create("output.csv").expect("Can't create file");
    let args = Args::parse();
    let multi = gpib::Communication::new(args.multi);
    let power_sup = gpib::Communication::new(args.power);
    let mut test = procedure::Process::new(
                                format!(r".\procedure\{}.txt", args.procedure).as_str()
                                        );
    __reset_all__(&multi, &power_sup);
    test.exec(&multi, &power_sup);
    __reset_all__(&multi, &power_sup);
}

fn __reset_all__(multi:&gpib::Communication, power_sup:&gpib::Communication)
{
    multi.set(b"*RST");
    power_sup.set(b"*RST");
}