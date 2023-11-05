static mut rand_pool: u128 = 0;

pub fn random_u128() -> u128 {
    unsafe {
        rand_pool = rand_pool.overflowing_mul(13192485271492728137413).0.overflowing_add(1817515137252438237).0;

    rand_pool
    }
}

pub fn random_reset() {
    unsafe {
        rand_pool = 0;
    }
}
