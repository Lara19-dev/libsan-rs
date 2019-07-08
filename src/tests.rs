
use super::*;
#[test]
fn test_castle_short() {
    let m = Move::parse("O-O").unwrap();
    assert_eq!(m.move_type, MoveType::Castle(CastleType::Kingside));
    assert_eq!(m.piece, None);
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_castle_long() {
    let m = Move::parse("O-O-O").unwrap();
    assert_eq!(m.move_type, MoveType::Castle(CastleType::Queenside));
    assert_eq!(m.piece, None);
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_pawn() {
    let m = Move::parse("e4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, super::POS_NONE);
            assert_eq!(dst, Position::of(4, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Pawn));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_pawn_long() {
    let m = Move::parse("e2e4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::of(4, 6));
            assert_eq!(dst, Position::of(4, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Pawn));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece() {
    let m = Move::parse("Qe4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, super::POS_NONE);
            assert_eq!(dst, Position::of(4, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Queen));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_file() {
    let m = Move::parse("Qbe4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(Some(1), None));
            assert_eq!(dst, Position::of(4, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Queen));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_rank() {
    let m = Move::parse("Q1e4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(None, Some(7)));
            assert_eq!(dst, Position::of(4, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Queen));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_long() {
    let m = Move::parse("Qb1e4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(Some(1), Some(7)));
            assert_eq!(dst, Position::of(4, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Queen));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_pawn_capture() {
    let m = Move::parse("exd4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(Some(4), None));
            assert_eq!(dst, Position::of(3, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Pawn));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_pawn_capture_promotion() {
    let m = Move::parse("exd8=Q").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(Some(4), None));
            assert_eq!(dst, Position::of(3, 0));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Pawn));
    assert_eq!(m.promotion, Some(Piece::Queen));
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_pawn_capture_long() {
    let m = Move::parse("e3xd4").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(Some(4), Some(5)));
            assert_eq!(dst, Position::of(3, 4));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Pawn));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_capture() {
    let m = Move::parse("Rxh3").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(None, None));
            assert_eq!(dst, Position::of(7, 5));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Rook));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_capture_file() {
    let m = Move::parse("Rexh3").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(Some(4), None));
            assert_eq!(dst, Position::of(7, 5));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Rook));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_capture_rank() {
    let m = Move::parse("R1xh3").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(None, Some(7)));
            assert_eq!(dst, Position::of(7, 5));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Rook));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_piece_capture_long() {
    let m = Move::parse("Re3xh3").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::of(4, 5));
            assert_eq!(dst, Position::of(7, 5));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Rook));
    assert_eq!(m.promotion, None);
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, true);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}
#[test]
fn test_pawn_promotion() {
    let m = Move::parse("d8=Q").unwrap();
    match m.move_type {
        MoveType::Normal(src, dst) => {
            assert_eq!(src, Position::new(None, None));
            assert_eq!(dst, Position::of(3, 0));
        },
        _ => assert!(false)
    }
    assert_eq!(m.piece, Some(Piece::Pawn));
    assert_eq!(m.promotion, Some(Piece::Queen));
    assert_eq!(m.annotation, None);
    assert_eq!(m.is_capture, false);
    assert_eq!(m.is_check, false);
    assert_eq!(m.is_check_mate, false);
}

#[test]
fn test_to_string() {
    let s = (Move {
        move_type: MoveType::Normal(Position::new(Some(4), None), Position::of(3,0)),
        piece: Some(Piece::Pawn),
        promotion: Some(Piece::Queen),
        annotation: Some(Annotation::Interesting),
        is_capture: true,
        is_check: true,
        is_check_mate: false
    }).to_string();

    assert_eq!(s, "exd8=Q+?!");
}