use bitvec::{ prelude::* };

fn rol(half: &BitSlice<u64, Msb0>) {
    let mut clone = half.clone().to_owned();
    let first_bit = half.get(0).unwrap();

    for (i, bit) in half.iter().enumerate() {
        if (i == 31) {
            clone.set(
                31,
                first_bit
                    .as_ref()
                    .to_owned()
                );
            continue;
        }

        clone.set(
            i,
            half
                .get(i + 1)
                .unwrap()
                .as_ref()
                .to_owned()
            );
    }

    println!("ORIGINAL: {:?}", half);
    println!("PERMUTED: {:?}", clone);
}

fn main() {
    let mut data: u64 = 11;
    let bits: &mut BitSlice<u64, Msb0> = data.view_bits_mut::<Msb0>();
    let (l, r) = bits.split_at(32);
    rol(r);
}