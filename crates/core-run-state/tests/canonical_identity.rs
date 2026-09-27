use core_run_state::CanonicalFingerprint;

#[test]
fn canonical_fingerprint_public_value_round_trips() {
    let fingerprint = CanonicalFingerprint::new(
        "dcf425cbda34f08eb813cf25954f83cb7c33169f1f330719fb502c7e5d337a10",
    )
    .expect("canonical fingerprint has valid lowercase hexadecimal form");
    let encoded = serde_json::to_string(&fingerprint).expect("serialize fingerprint value");
    let decoded: CanonicalFingerprint =
        serde_json::from_str(&encoded).expect("deserialize fingerprint value");

    assert_eq!(decoded, fingerprint);
    assert_eq!(
        decoded.as_str(),
        "dcf425cbda34f08eb813cf25954f83cb7c33169f1f330719fb502c7e5d337a10"
    );
}
