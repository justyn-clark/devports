use clap::CommandFactory;
use devports::cli::Cli;

fn render_help(command: &mut clap::Command) -> String {
    let mut buf = Vec::new();
    command.write_long_help(&mut buf).expect("write help");
    String::from_utf8(buf).expect("utf8 help")
}

#[test]
fn root_help_includes_banner_and_examples() {
    let mut command = Cli::command();
    let help = render_help(&mut command);

    assert!(help.contains("██████╗"));
    assert!(help.contains("Examples:"));
    assert!(help.contains("devports config init"));
    assert!(help.contains("devports tui"));
}

#[test]
fn doctor_help_includes_guidance_and_exit_status() {
    let mut command = Cli::command();
    let help = {
        let doctor = command.find_subcommand_mut("doctor").expect("doctor subcommand");
        render_help(doctor)
    };

    assert!(help.contains("duplicate ports"));
    assert!(help.contains("devports doctor --json"));
    assert!(help.contains("Returns non-zero when the report contains errors."));
}
