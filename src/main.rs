use rustdeco::zhl16;
use rustdeco::common;
use rustdeco::common::deco_algorithm::DecoAlgorithm;

fn main() {
    let ean32 = common::gas::Gas::new(0.68, 0.32, 0.0).unwrap();
    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();
    let pure_o2 = common::gas::Gas::new(0.0, 1.0, 0.0).unwrap();

    let mut deco = zhl16::ZHL16::new(common::DEFAULT_ASCENT_RATE,
                                     common::DEFAULT_DESCENT_RATE);
    deco.initialise_tissues(&air);
    let depth = 60;
    let time = 30;

    println!("At surface: {:?}\n", deco);
    deco.add_bottom_time(depth, time, &air);
    println!("Dive to {}m for {}min on {:?}:: {:?}\n", depth, time, &air, deco);

    let deco2 = deco.clone();

    //println!("Decompression stops using {:?}: {:?}", air, deco.get_stops(&air));
    println!("Dive to {}m for {}min on {:?}", depth, time, &air);
    println!("Descent rate: {}m/min, ascent rate: {}m/min\n", common::DEFAULT_DESCENT_RATE,
             common::DEFAULT_ASCENT_RATE);

    for x in deco.get_stops(&air) {
        println!("Stop: {}m for {}min", x.get_depth(), x.get_time())
    }

    //println!("Decompression stops using {:?}: {:?}", pure_o2, deco2.get_stops(&pure_o2));

}