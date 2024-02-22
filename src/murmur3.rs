const C1: u32 = 0x85eb_ca6b;
const C2: u32 = 0xc2b2_ae35;
const R1: u32 = 16;
const R2: u32 = 13;
const M: u32 = 5;
const N: u32 = 0xe654_6b64;

/// Custom: Return the last intermediate state if the input only contains full 32 bit (4-byte) chunks.
pub fn get_intermediate(input: &[u8]) -> Option<(u32, usize)> {
    let mut state = 0;
    let mut processed = 0;
    for buffer in input.chunks(4) {
        // println!("buffer: {:?}", buffer);
        match buffer.len() {
            4 => {
                processed += 4;
                let k = u32::from_le_bytes(buffer.try_into().unwrap());
                state ^= calc_k(k);
                state = state.rotate_left(R2);
                state = (state.wrapping_mul(M)).wrapping_add(N);
            }
            _ => return None,
        }
    }

    Some((state, processed))
}

/// Use the 32 bit variant of murmur3 to hash some [Read] implementation.
///
/// # Example
/// ```
/// use std::io::Cursor;
/// use murmur3::murmur3_32;
/// let hash_result = murmur3_32(&mut Cursor::new("hello world"), 0);
/// ```
pub fn murmur3_32(input: &[u8], seed: u32, mut processed: usize) -> u32 {
    // let mut processed = 0;
    let mut state = seed;
    for buffer in input.chunks(4) {
        // println!("buffer: {:?}", buffer);
        match buffer.len() {
            4 => {
                processed += 4;
                let k = u32::from_le_bytes(buffer.try_into().unwrap());
                state ^= calc_k(k);
                state = state.rotate_left(R2);
                state = (state.wrapping_mul(M)).wrapping_add(N);
                // dbg!(state);
                // println!("state: {state} after {processed} bytes");
            }
            3 => {
                processed += 3;
                let k: u32 =
                    ((buffer[2] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
            }
            2 => {
                processed += 2;
                let k: u32 = ((buffer[1] as u32) << 8) | (buffer[0] as u32);
                state ^= calc_k(k);
            }
            1 => {
                processed += 1;
                let k: u32 = buffer[0] as u32;
                state ^= calc_k(k);
            }
            _ => unreachable!(),
        }
    }
    finish(state, processed)
}

fn finish(state: u32, processed: usize) -> u32 {
    // dbg!("finish", state, processed);
    let mut hash = state;
    hash ^= processed as u32;
    hash ^= hash.wrapping_shr(R1);
    hash = hash.wrapping_mul(C1);
    hash ^= hash.wrapping_shr(R2);
    hash = hash.wrapping_mul(C2);
    hash ^= hash.wrapping_shr(R1);
    hash
}

fn calc_k(k: u32) -> u32 {
    const C1: u32 = 0xcc9e_2d51;
    const C2: u32 = 0x1b87_3593;
    const R1: u32 = 15;
    k.wrapping_mul(C1).rotate_left(R1).wrapping_mul(C2)
}
