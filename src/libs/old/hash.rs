pub fn murmur3_32(key: &[u8], seed: u32) -> u32 {
    let mut hash = seed;
    for chunk in key.chunks(4) {
        let mut c = [0; 4];
        c.copy_from_slice(chunk);
        let k = u32::from_le_bytes(c);
        let k = k.wrapping_mul(0xcc9e2d51).rotate_left(15).wrapping_mul(0x1b873593);
        hash ^= k;
        if chunk.len() == 4 {
            hash = hash.rotate_left(13).wrapping_mul(5).wrapping_add(0xe6546b64);
        }
    }
    hash ^= key.len() as u32;
    hash = (hash ^ hash >> 16).wrapping_mul(0x85ebca6b);
    hash = (hash ^ hash >> 13).wrapping_mul(0xc2b2ae35);
    hash ^ hash >> 16
}