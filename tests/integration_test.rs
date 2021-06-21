#[tokio::test]
async fn test_basic_true() {
	let name = String::from("A");
	let success = zero_knowledge::prove_and_verify::verify(name, 11).await;
    assert_eq!(success, true);
}