use devports::proc::ps::parse_ps_line;

#[test]
fn parse_ps_fixture_line() {
    let line = include_str!("fixtures/ps_sample.txt").trim();
    let info = parse_ps_line(line).expect("parse ps line");

    assert_eq!(info.pid, 12345);
    assert_eq!(info.ppid, 1);
    assert_eq!(info.pgid, 12345);
    assert!(info.command.contains("server.js"));
}
