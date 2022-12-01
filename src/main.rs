use bitvec::{prelude::*, slice::SplitInclusive};

// enum OddSizedBlock {
//     BitSlice<>
// }

/// Rols a 32(u64) bit &BitSlice
///
/// Given a reference to BitSlice<u64, Msb0> moves every bit to left
/// Example:
/// ```rust
/// let data: u64 = 9896123456;
/// let bits: &mut BitSlice<u64, Msb0> = data.view_bits_mut::<Msb0>();
/// let (l: &mut BitSlice<u64, Msb0>, r: &mut BitSlice<u64, Msb0>) = bits.split_at(32);
/// let permuted_r: BitSlice<u64, Msb0> = rol(r);
/// ```
fn rol(half: &BitSlice<u64, Msb0>) -> BitVec<u64, Msb0> {
    let mut clone = half.clone().to_owned();
    let first_bit = half.get(0).unwrap();

    for (i, _bit) in half.iter().enumerate() {
        if i == 31 {
            clone.set(31, first_bit.as_ref().to_owned());
            continue;
        }
        let right_bit = half.get(i + 1).unwrap().as_ref().to_owned();
        clone.set(i, right_bit);
    }

    return clone;
}

/// Transforms a BitSlice<u64, Msb0> into a Vec<u8>
///
/// Example:
/// ```rust
/// let rolled: BitSlice<u64, Msb0> = rol(r);
/// println!("{:?}", rolled);
/// ```
fn bits_to_ut8_vec(bit_block: &BitSlice<u64, Msb0>) -> Vec<u8> {
    let mut pseudo_bit_arr: Vec<u8> = vec![];
    for bit in bit_block.iter() {
        pseudo_bit_arr.push(bit.as_ref().to_owned() as u8);
    }

    return pseudo_bit_arr;
}

fn text_to_64b_blocks(text: String) {
    let block: BitVec<u8, Msb0> = text.as_bits().to_bitvec().to_owned();
    let _iter = block.split_inclusive(|pos, _bit| (pos.to_owned() as i32 + 1) % 64 == 0);
}

trait BlockIterator<T> {
    fn split_bits_into_odd_blocks() -> SplitInclusive<'static, u8, Msb0, fn(usize, &bool) -> bool>;
}

fn splitter(pos: usize, _bit: &bool) -> bool {
    return (pos.to_owned() as i32 + 1) % 32 == 0;
}

fn split_bits_into_odd_blocks(
    source: BitVec<u8, Msb0>,
    size: i32,
) -> SplitInclusive<'a, u8, Msb0, fn(usize, &bool) -> bool> {
    let _iter = source.split_inclusive(splitter);
    return _iter.clone();
}

fn text_to_bits(text: String) -> BitVec<u8, Msb0> {
    return text.as_bits().to_bitvec().to_owned();
}

fn main() {
    text_to_64b_blocks("text 8AUusd aksjdflskjdf iw ejemplo".to_string());
    let mut data: u64 = 9896123456;
    let bits: &mut BitSlice<u64, Msb0> = data.view_bits_mut::<Msb0>();
    let (_l, r) = bits.split_at(32);
    let permuted_r = rol(r);

    println!("ORIGINAl: {:?}", bits_to_ut8_vec(r));
    println!("PREMUTED: {:?}", bits_to_ut8_vec(&permuted_r));
}
