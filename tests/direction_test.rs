use artful::direction::{Destination, DirectionKind};

#[test]
fn test_direction_kind_default() {
    let kind = DirectionKind::JsonDirection;
    assert!(matches!(kind, DirectionKind::JsonDirection));
}

#[test]
fn test_destination_default() {
    let dest = Destination::default();
    assert!(matches!(dest, Destination::None));
}
