use warren::get_report;

#[test]
fn get_report_fn_returns_plausible_data() {
    let rep = get_report("AAPL").unwrap();
    assert_eq!(rep.symbol, "AAPL", "wrong symbol");
    assert!(0.0 < rep.history.current, "missing data");
    assert!(rep.history.low <= rep.history.high, "low higher than high");
    assert!(rep.history.low <= rep.history.current, "current lower than low ");
    assert!(rep.history.current <= rep.history.high, "current higher than high ");
    assert!(0.0 <= rep.position, "bad position: negative");
    assert!(rep.position <= 100.0, "bad position: > 100");
    if rep.position < 50.0 {
        assert_eq!(rep.recommendation, "Buy");
    } else {
        assert_eq!(rep.recommendation, "Don't buy");
    }
}