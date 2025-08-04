use felt::Felt;
use stark::stark_proof::stark_commit::GenerateInteractionElements;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

#[test]
fn test_generate_interaction_elements() {
    let mut stack = BidirectionalStackAccount::default();

    // Setup transcript state
    let digest =
        Felt::from_hex("0x1b9182dce9dc1169fcd00c1f8c0b6acd6baad99ce578370ead5ca230b8fb8c6")
            .unwrap();
    let counter = Felt::ONE;
    let elements_count = 6;

    // Push transcript state (counter, digest)
    stack.push_front(&counter.to_bytes_be()).unwrap();
    stack.push_front(&digest.to_bytes_be()).unwrap();

    let interaction_task = GenerateInteractionElements::new(elements_count);
    stack.push_task(interaction_task);

    // Execute until completion
    let mut steps = 0;
    while !stack.is_empty_back() {
        stack.execute();
        steps += 1;
    }

    //interaction elements are in reverse order
    let element0 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element1 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element2 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element3 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element4 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element5 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();
    let element6 = Felt::from_bytes_be_slice(stack.borrow_front());
    stack.pop_front();

    println!("element6: {:?}", element6);
    println!("element5: {:?}", element5);
    println!("element4: {:?}", element4);
    println!("element3: {:?}", element3);
    println!("element2: {:?}", element2);
    println!("element1: {:?}", element1);
    println!("element0: {:?}", element0);

    assert_eq!(stack.front_index, 0, "Stack should be empty");
    assert_eq!(stack.back_index, 65536, "Stack should be empty");
    assert!(steps > 0, "Should have executed at least one step");

    assert_eq!(
        element5,
        Felt::from_hex("0x17314236e645ae09f56d65ddac33407692a77621eb0081a9658ed83099dc4d").unwrap()
    );
    assert_eq!(
        element4,
        Felt::from_hex("0x4777789571ddf7a8c984641aeb028d650984ebee3fc87882dffa1fca47a58a4")
            .unwrap()
    );
    assert_eq!(
        element3,
        Felt::from_hex("0x2dd82d142de05c4142ebecea2865210c5957dc0194fc0606161b382cef15cca")
            .unwrap()
    );
    assert_eq!(
        element2,
        Felt::from_hex("0xf12d7ec57d88d87a1f80035e8beb69e23eaad3178a70d1b4b043d7a19857c4").unwrap()
    );
    assert_eq!(
        element1,
        Felt::from_hex("0x4f7a19717bb413dcd205d088855044c80ee99e58bab23218b045ff222a24a6a")
            .unwrap()
    );
    assert_eq!(
        element0,
        Felt::from_hex("0x6f027527205d5923422d155c91627477c1af7d73370c17e00247e7fe35b1992")
            .unwrap()
    );
}
