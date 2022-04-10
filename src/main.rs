//! # Hexed vintage puzzle solution finder
//! 
//! Craig McQueen 2022
//! 
//! Board is 6 across, 10 down.
//! Pieces are all 5 units in some shape.
//! 
//!      #          1 rotation
//!     ###
//!      #
//! 
//!     #####       2 rotations
//! 
//!     #           2 rotations, same again mirrored
//!     ###
//!       #
//! 
//!     ##          4 rotations
//!      ##
//!       #
//! 
//!     ###         4 rotations
//!     # #
//! 
//!     ###         4 rotations
//!      #
//!      #
//! 
//!     ###         4 rotations
//!     #
//!     #
//! 
//!     ####        4 rotations, same again mirrored
//!        #
//!
//!     ####        4 rotations, same again mirrored
//!       #
//! 
//!     ###         4 rotations, same again mirrored
//!       ##
//! 
//!      #          4 rotations, same again mirrored
//!     ###
//!       #
//! 
//!     ###         4 rotations, same again mirrored
//!     ##
//! 

struct Coord(usize, usize);

struct PieceInfo {
    width: usize,
    height: usize,
    p: [ Coord; 5 ],
}

/// Pieces

///      #          1 rotation
///     ###
///      #
const PIECE_0_0: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(1, 2) ],
};

///     #####       2 rotations
const PIECE_1_0: PieceInfo = PieceInfo {
    width: 5,
    height: 1,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0), Coord(4, 0) ],
};
const PIECE_1_1: PieceInfo = PieceInfo {
    width: 1,
    height: 5,
    p: [ Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3), Coord(0, 4) ],
};

///     #           2 rotations, same again mirrored
///     ###
///       #
const PIECE_2_0: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(2, 2) ],
};
const PIECE_2_1: PieceInfo = PieceInfo { // rotate clockwise
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(0, 2) ],
};
const PIECE_2_2: PieceInfo = PieceInfo { // mirror
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(2, 1), Coord(1, 1), Coord(0, 1), Coord(0, 2) ],
};
const PIECE_2_3: PieceInfo = PieceInfo { // mirror and rotate
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(2, 2) ],
};

///     ##          4 rotations
///      ##
///       #
const PIECE_3_0: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(2, 1), Coord(2, 2) ],
};
const PIECE_3_1: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(2, 1), Coord(1, 1), Coord(1, 2), Coord(0, 2) ],
};
const PIECE_3_2: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2), Coord(2, 2) ],
};
const PIECE_3_3: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(1, 0), Coord(1, 1), Coord(0, 1), Coord(0, 2) ],
};

///     ###         4 rotations
///     # #
const PIECE_4_0: PieceInfo = PieceInfo {
    width: 3,
    height: 2,
    p: [ Coord(0, 1), Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(2, 1) ],
};
const PIECE_4_1: PieceInfo = PieceInfo {
    width: 2,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(0, 2) ],
};
const PIECE_4_2: PieceInfo = PieceInfo {
    width: 3,
    height: 2,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(2, 0) ],
};
const PIECE_4_3: PieceInfo = PieceInfo {
    width: 2,
    height: 3,
    p: [ Coord(1, 0), Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 2) ],
};

///     ###         4 rotations
///      #
///      #
const PIECE_5_0: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(1, 1), Coord(1, 2) ],
};
const PIECE_5_1: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(2, 1), Coord(2, 2), Coord(1, 1), Coord(0, 1) ],
};
const PIECE_5_2: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(1, 1), Coord(0, 2), Coord(1, 2), Coord(2, 2) ],
};
const PIECE_5_3: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 1), Coord(2, 1) ],
};

///     ###         4 rotations
///     #
///     #
const PIECE_6_0: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(1, 0), Coord(0, 0), Coord(0, 1), Coord(0, 2) ],
};
const PIECE_6_1: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(2, 1), Coord(2, 2) ],
};
const PIECE_6_2: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(2, 1), Coord(2, 2), Coord(1, 2), Coord(0, 2) ],
};
const PIECE_6_3: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 2), Coord(2, 2) ],
};

///     ####        4 rotations, same again mirrored
///        #
const PIECE_7_0: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0), Coord(3, 1) ],
};
const PIECE_7_1: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(1, 3), Coord(0, 3) ],
};
const PIECE_7_2: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(3, 1) ],
};
const PIECE_7_3: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(1, 0), Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3) ],
};
const PIECE_7_4: PieceInfo = PieceInfo { // mirror
    width: 4,
    height: 2,
    p: [ Coord(0, 1), Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0) ],
};
const PIECE_7_5: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(1, 3) ],
};
const PIECE_7_6: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(3, 0), Coord(3, 1), Coord(2, 1), Coord(1, 1), Coord(0, 1) ],
};
const PIECE_7_7: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3), Coord(1, 3) ],
};

///     ####        4 rotations, same again mirrored
///       #
const PIECE_8_0: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0), Coord(2, 1) ],
};
const PIECE_8_1: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(1, 3), Coord(0, 2) ],
};
const PIECE_8_2: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(3, 1) ],
};
const PIECE_8_3: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(1, 1), Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3) ],
};
const PIECE_8_4: PieceInfo = PieceInfo { // mirror
    width: 4,
    height: 2,
    p: [ Coord(1, 1), Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0) ],
};
const PIECE_8_5: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(0, 1), Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(1, 3) ],
};
const PIECE_8_6: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(2, 0), Coord(3, 1), Coord(2, 1), Coord(1, 1), Coord(0, 1) ],
};
const PIECE_8_7: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(0, 3), Coord(1, 2) ],
};

///     ###         4 rotations, same again mirrored
///       ##
const PIECE_9_0: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(2, 1), Coord(3, 1) ],
};
const PIECE_9_1: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(1, 0), Coord(1, 1), Coord(1, 2), Coord(0, 2), Coord(0, 3) ],
};
const PIECE_9_2: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(2, 1), Coord(3, 1) ],
};
const PIECE_9_3: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(1, 0), Coord(1, 1), Coord(0, 1), Coord(0, 2), Coord(0, 3) ],
};
const PIECE_9_4: PieceInfo = PieceInfo { // mirror
    width: 4,
    height: 2,
    p: [ Coord(0, 1), Coord(1, 1), Coord(1, 0), Coord(2, 0), Coord(3, 0) ],
};
const PIECE_9_5: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2), Coord(1, 3) ],
};
const PIECE_9_6: PieceInfo = PieceInfo {
    width: 4,
    height: 2,
    p: [ Coord(3, 0), Coord(2, 0), Coord(2, 1), Coord(1, 1), Coord(0, 1) ],
};
const PIECE_9_7: PieceInfo = PieceInfo {
    width: 2,
    height: 4,
    p: [ Coord(0, 0), Coord(0, 1), Coord(0, 2), Coord(1, 2), Coord(1, 3) ],
};

///     #           4 rotations, same again mirrored
///     ###
///      #
const PIECE_10_0: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(1, 2) ],
};
const PIECE_10_1: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(2, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2) ],
};
const PIECE_10_2: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(2, 2) ],
};
const PIECE_10_3: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(1, 1), Coord(2, 1), Coord(0, 2), Coord(1, 2) ],
};
const PIECE_10_4: PieceInfo = PieceInfo { // mirrored
    width: 3,
    height: 3,
    p: [ Coord(2, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(1, 2) ],
};
const PIECE_10_5: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2), Coord(2, 2) ],
};
const PIECE_10_6: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1), Coord(0, 2) ],
};
const PIECE_10_7: PieceInfo = PieceInfo {
    width: 3,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(2, 1), Coord(1, 2) ],
};

///     ###         4 rotations, same again mirrored
///     ##
const PIECE_11_0: PieceInfo = PieceInfo {
    width: 3,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(0, 1), Coord(1, 1) ],
};
const PIECE_11_1: PieceInfo = PieceInfo {
    width: 2,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(1, 2) ],
};
const PIECE_11_2: PieceInfo = PieceInfo {
    width: 3,
    height: 2,
    p: [ Coord(1, 0), Coord(2, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1) ],
};
const PIECE_11_3: PieceInfo = PieceInfo {
    width: 2,
    height: 3,
    p: [ Coord(0, 0), Coord(0, 1), Coord(1, 1), Coord(0, 2), Coord(1, 2) ],
};
const PIECE_11_4: PieceInfo = PieceInfo { // mirrored
    width: 3,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(1, 1), Coord(2, 1) ],
};
const PIECE_11_5: PieceInfo = PieceInfo {
    width: 2,
    height: 3,
    p: [ Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(0, 2), Coord(1, 2) ],
};
const PIECE_11_6: PieceInfo = PieceInfo {
    width: 3,
    height: 2,
    p: [ Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(2, 1) ],
};
const PIECE_11_7: PieceInfo = PieceInfo {
    width: 2,
    height: 3,
    p: [ Coord(0, 0), Coord(1, 0), Coord(0, 1), Coord(1, 1), Coord(0, 2) ],
};

const PIECE_0: &[PieceInfo] = &[ PIECE_0_0 ];
const PIECE_1: &[PieceInfo] = &[ PIECE_1_0, PIECE_1_1 ];
const PIECE_2: &[PieceInfo] = &[ PIECE_2_0, PIECE_2_1, PIECE_2_2, PIECE_2_3 ];
const PIECE_3: &[PieceInfo] = &[ PIECE_3_0, PIECE_3_1, PIECE_3_2, PIECE_3_3 ];
const PIECE_4: &[PieceInfo] = &[ PIECE_4_0, PIECE_4_1, PIECE_4_2, PIECE_4_3 ];
const PIECE_5: &[PieceInfo] = &[ PIECE_5_0, PIECE_5_1, PIECE_5_2, PIECE_5_3 ];
const PIECE_6: &[PieceInfo] = &[ PIECE_6_0, PIECE_6_1, PIECE_6_2, PIECE_6_3 ];
const PIECE_7: &[PieceInfo] = &[ PIECE_7_0, PIECE_7_1, PIECE_7_2, PIECE_7_3, PIECE_7_4, PIECE_7_5, PIECE_7_6, PIECE_7_7 ];
const PIECE_8: &[PieceInfo] = &[ PIECE_8_0, PIECE_8_1, PIECE_8_2, PIECE_8_3, PIECE_8_4, PIECE_8_5, PIECE_8_6, PIECE_8_7 ];
const PIECE_9: &[PieceInfo] = &[ PIECE_9_0, PIECE_9_1, PIECE_9_2, PIECE_9_3, PIECE_9_4, PIECE_9_5, PIECE_9_6, PIECE_9_7 ];
const PIECE_10: &[PieceInfo] = &[ PIECE_10_0, PIECE_10_1, PIECE_10_2, PIECE_10_3, PIECE_10_4, PIECE_10_5, PIECE_10_6, PIECE_10_7 ];
const PIECE_11: &[PieceInfo] = &[ PIECE_11_0, PIECE_11_1, PIECE_11_2, PIECE_11_3, PIECE_11_4, PIECE_11_5, PIECE_11_6, PIECE_11_7 ];

const PIECES: [ &[PieceInfo]; 12 ] = [ PIECE_0, PIECE_1, PIECE_2, PIECE_3, PIECE_4, PIECE_5, PIECE_6, PIECE_7, PIECE_8, PIECE_9, PIECE_10, PIECE_11 ];

fn main() {
    for piece in PIECES.iter() {
        for piece_variant in piece.iter() {
            let mut piece_array = vec![vec![' '; piece_variant.width]; piece_variant.height];
            for p in piece_variant.p.iter() {
                piece_array[p.1][p.0] = '#';
            }
            for row in 0..piece_variant.height {
                let row_text: String = piece_array[row].iter().collect();
                println!("{}", row_text);
            }
            print!("\n");
        }
        print!("------------------------------\n");
    }
}
