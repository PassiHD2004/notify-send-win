use clap::Parser;
use win_toast_notify::*;

const SOURCE: &str = "https://github.com/PassiHD2004/notify-send-win";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "notify-send-win", help = "\"App Name\" for the Toast")]
    app: String,
    #[arg(short, long, default_value = "This is a default title", help = "\"Title\" for the Toast")]
    title: String,
    #[arg(short, long, default_value = "This is a default notification\nrun \"notify-send.exe --help\" for more information", help = "\"Message\" for the Toast")]
    message: String,
    #[arg(short, long, default_value = SOURCE, help = "Link to go to, when clicked")]
    link: String,
    #[arg(short, long, default_value = "", help = "Logo for the notification")]
    notify_logo: String,
    #[arg(short, long, default_value = "", help = "Banner image to display")]
    image: String,
    #[arg(short, long, default_value = "short", help = "How long to display the Toast. Options: \"short\" or \"long\"")]
    duration: String,
}

fn main() {
    let args = Cli::parse();

    let duration: Duration = if args.duration == "short" {
        Duration::Short
    } else if args.duration == "long" {
        Duration::Long
    } else {
        println!("Invalid duration. Possible values: \"short\" or \"long\"");
        Duration::Short
    };

    WinToastNotify::new()
        .set_app_id(&args.app)
        .set_notif_open(&args.link)
        .set_duration(duration)
        .set_title(&args.title)
        .set_messages(vec![&args.message])
        .set_logo(&args.notify_logo, CropCircle::True)
        .set_image(&args.image, ImagePlacement::Top)
        .show()
        .expect("Failed to show toast notification")
}