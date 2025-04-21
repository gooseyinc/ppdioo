// test.rs
// can test data structure with cargo test command

use ppdioo::{SwitchData, Backbone, Spine, AccessLayer};

#[test]
fn test_manual_switch_data() {
    let test_data = SwitchData {
        backbone: Some(Backbone {
            backbone_switch: Some("TestCore".into()),
        }),
        spine: Some(Spine {
            spine_switch: Some("TestSpine".into()),
        }),
        access: Some(AccessLayer {
            tor: Some("TestTOR".into()),
            aor: Some("TestAOR".into()),
        }),
    };

    // Unwrapped AccessLayer
    assert_eq!(
        test_data.backbone.unwrap().backbone_switch.unwrap(),
        "TestCore"
    );
    assert_eq!(
        test_data.spine.unwrap().spine_switch.unwrap(),
        "TestSpine"
    );
    assert_eq!(
        test_data.access.as_ref().unwrap().tor.as_ref().unwrap(),
        "TestTOR"
    );
    assert_eq!(
        test_data.access.as_ref().unwrap().aor.as_ref().unwrap(),
        "TestAOR"
    );
}
