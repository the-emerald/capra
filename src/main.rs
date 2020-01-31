use rustdeco::zhl16;
use rustdeco::common;
use rustdeco::common::deco_algorithm::DecoAlgorithm;
use rustdeco::common::dive_segment::{DiveSegment, SegmentType};

fn main() {
    let ean32 = common::gas::Gas::new(0.68, 0.32, 0.0).unwrap();
    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();
    let pure_o2 = common::gas::Gas::new(0.0, 1.0, 0.0).unwrap();

    let mut dive = zhl16::ZHL16::new(&air);
    let depth = 35;
    let time = 60;

    let ascent_rate = -10;
    let descent_rate = 30;

    let first_segment = DiveSegment::new(SegmentType::DiveSegment, depth,
                                         time, ascent_rate,
                                         descent_rate);
    dive.add_bottom_time(&first_segment, &air);

    println!("Dive to {}m for {}min on {:?}", depth, time, &air);
    println!("Descent rate: {}m/min, ascent rate: {}m/min\n", descent_rate,
             ascent_rate);

    for x in dive.get_stops(ascent_rate,
                            descent_rate, &air) {
        match x.get_segment_type() {
            SegmentType::NoDeco => {
                if x.get_time() == std::usize::MAX {
                    println!("{:?}: unlimited", x.get_segment_type())
                }
                else {
                    println!("{:?}: {}min", x.get_segment_type(), x.get_time())
                }
            },
            SegmentType::DecoStop => println!("{:?}: {}m for {}min", x.get_segment_type(),
                                              x.get_depth(), x.get_time()),
            _ => {}
        }
    }
}