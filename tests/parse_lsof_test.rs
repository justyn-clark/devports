use devports::scan::lsof::parse_lsof_listeners;

#[test]
fn parse_lsof_fixture() {
    let input = include_str!("fixtures/lsof_macos.txt");
    let parsed = parse_lsof_listeners(input);

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].pid, 12345);
    assert_eq!(parsed[0].port, 3000);
    assert_eq!(parsed[1].pid, 22334);
    assert_eq!(parsed[1].port, 3001);
}
