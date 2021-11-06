use std::env;
use std::process::{Command, Stdio};

fn cmd_stdout(cmd: &str, args: Vec<&str>) -> String {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();
    return output_str;
}

fn cmd(cmd: &str, args: Vec<&str>) {
    Command::new(cmd).args(args).output().expect("error");
}

fn notify_dunst(sink: &str) {
    let vol_cmd = cmd_stdout("pactl", vec!["get-sink-volume", "@DEFAULT_SINK@"]);
    let vol_split: Vec<&str> = vol_cmd.split("%").collect();
    let vol_str = vol_split[0].to_string();
    let vol_len = vol_str.len();
    let vol_str_val = vol_str[vol_len - 3..vol_len].trim();
    let vol_int = vol_str_val.parse::<i32>().unwrap();
    let mute_cmd = cmd_stdout("pactl", vec!["get-sink-mute", "@DEFAULT_SINK@"]);
    let mute_split: Vec<&str> = mute_cmd.split(" ").collect();
    let mute_str_ = mute_split[1].to_string();
    let mute_str = mute_str_.trim();
    let mut icon_path: String =
        "~/.icons/la-capitaine-icon-theme/status/scalable-dark/".to_string();
    if mute_str == "yes" {
        icon_path += "audio-volume-muted.svg";
    } else if vol_int <= 33 {
        icon_path += "audio-volume-low.svg";
    } else if vol_int <= 66 {
        icon_path += "audio-volume-medium.svg";
    } else if vol_int > 67 {
        icon_path += "audio-volume-high.svg";
    }
    let dunst_str = format!("\"{}\n{}%\"", sink, vol_str_val);
    cmd(
        "dunstify",
        vec![
            "-a",
            "\"System Volume\"",
            "-r",
            "506",
            "-u",
            "low",
            "-i",
            &icon_path,
            &dunst_str,
        ],
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let sink_cmd = cmd_stdout("pactl", vec!["get-default-sink"]);
    let sink_str: &str = sink_cmd.split(".").last().unwrap().trim();
    if args[1] == "up" {
        cmd("pactl", vec!["set-sink-volume", "@DEFAULT_SINK@", "+5%"]);
    } else if args[1] == "down" {
        cmd("pactl", vec!["set-sink-volume", "@DEFAULT_SINK@", "-5%"]);
    } else if args[1] == "mute" {
        cmd("pactl", vec!["set-sink-mute", "@DEFAULT_SINK@", "toggle"]);
    }
    notify_dunst(sink_str);
}
