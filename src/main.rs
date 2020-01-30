use rustdeco::zhl16;
use rustdeco::common;
use rustdeco::common::deco_algorithm::DecoAlgorithm;

fn main() {
    let ean32 = common::gas::Gas::new(0.68, 0.32, 0.0).unwrap();
    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();
    let pure_o2 = common::gas::Gas::new(0.0, 1.0, 0.0).unwrap();

    let mut deco = zhl16::ZHL16::new();
    deco.initialise_tissues(&air);
    let depth = 30;
    let time = 60;

    println!("At surface: {:?}\n", deco);
    deco.add_bottom_time(depth, time, &air);
    println!("Dive to {}m for {}min on {:?}:: {:?}\n", depth, time, &air, deco);

    let deco2 = deco.clone();

    println!("Decompression stops using {:?}: {:?}", air, deco.get_stops(&air));
    println!("Decompression stops using {:?}: {:?}", pure_o2, deco2.get_stops(&pure_o2));

}