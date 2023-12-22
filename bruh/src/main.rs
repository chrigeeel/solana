use {
    crate::{
        shred::{
            max_ticks_per_n_shreds, ErasureSetId, ProcessShredsStats, ReedSolomonCache,
            Shred, ShredData, ShredId, ShredType, Shredder,
        },
    },
};

fn main() {
    println!("Hello, world!");

    Shredder::deshred()
}
