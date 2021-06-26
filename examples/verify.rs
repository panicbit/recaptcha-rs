use std::net::IpAddr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(long, short)]
    secret: String,
    #[structopt(long, short)]
    response: String,
    #[structopt(long = "ip", short = "i")]
    remote_ip: Option<IpAddr>,
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();

    let result = recaptcha::verify(&args.secret, &args.response, args.remote_ip.as_ref()).await;

    match result {
        Ok(()) => println!("Recaptcha passed successfully!"),
        Err(err) => println!("Recaptcha failed: {}", err),
    }
}
