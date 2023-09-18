pub fn cropping_modulo(value: i64, modulo: i64) -> u64 {
    ((modulo + (value % modulo)) % modulo) as u64
}
