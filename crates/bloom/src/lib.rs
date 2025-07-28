mod atoms;
mod container;
mod filter;
mod nif;
mod options;

rustler::init!(
    "bloom_nif",
    load = nif::on_load
);
