use rustdeco::zhl16;
use rustdeco::common;
use rustdeco::common::deco_algorithm::DecoAlgorithm;

fn main() {
    let fr_n2 = 0.50;
    let fr_o2 = 0.30;
    let fr_he = 0.20;
    let mix = common::gas::Gas::new(fr_n2, fr_o2, fr_he).unwrap();

    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();

    let mut deco = zhl16::ZHL16::new();
    deco.initialise_tissues(&air);
    let depth = 25;
    let time = 20;

    println!("At surface: {:?}\n", deco);
    deco.add_bottom_time(depth, time, &air);
}