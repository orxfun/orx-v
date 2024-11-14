mod card;
mod d1;
mod d2_rectangular;
mod d2_variable;
mod d3_rectangular;
mod d3_variable;
mod d4_rectangular;
mod d4_variable;
mod empty_card;
mod is_rectangular;
mod udd;

// out-of-bound errors
#[allow(clippy::panic)]
fn panic_d1(i: usize, card_idx0: usize) -> ! {
    panic!(
        "Required bound condition i < vec.card([]) fails for i={} and vec.card([])={}",
        i, card_idx0
    )
}

#[allow(clippy::panic)]
fn panic_d2(i: usize, j: usize, card_idx1: usize) -> ! {
    panic!(
        "Required bound condition j < vec.card([i]) fails for j={} and vec.card([{}])={}",
        j, i, card_idx1
    )
}

#[allow(clippy::panic)]
fn panic_d3(i: usize, j: usize, k: usize, card_idx2: usize) -> ! {
    panic!(
        "Required bound condition k < vec.card([i, j]) fails for k={} and vec.card([{}, {}])={}",
        k, i, j, card_idx2
    )
}

pub(crate) use card::panic_on_all_when_udd;
pub(crate) use is_rectangular::IsRectangular;

pub use card::Card;
pub use d1::CardD1;
pub use d2_rectangular::RectangularCardD2;
pub use d2_variable::VariableCardD2;
pub use d3_rectangular::RectangularCardD3;
pub use d3_variable::VariableCardD3;
pub use d4_rectangular::RectangularCardD4;
pub use d4_variable::VariableCardD4;
pub use empty_card::EmptyCard;
pub use udd::UnboundedCard;
