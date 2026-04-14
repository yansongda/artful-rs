use artful::direction::{Destination, DirectionKind};

#[test]
fn test_direction_kind_default() {
    let kind = DirectionKind::CollectionDirection;
    assert!(matches!(kind, DirectionKind::CollectionDirection));
}

#[test]
fn test_destination_default() {
    let dest = Destination::default();
    assert!(matches!(dest, Destination::None));
}
