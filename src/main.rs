use rustdeco::zhl16;
use rustdeco::common;
use rustdeco::common::deco_algorithm::DecoAlgorithm;
use rustdeco::common::dive_segment::{DiveSegment, SegmentType};
use rustdeco::common::gas::Gas;

fn pretty_print_deco_stops(stops: Vec<DiveSegment>) {
    for stop in stops {
        match stop.get_segment_type() {
            SegmentType::NoDeco => {
                if stop.get_time() == std::usize::MAX {
                    println!("{:?}: unlimited", stop.get_segment_type())
                }
                else {
                    println!("{:?}: {}min", stop.get_segment_type(), stop.get_time())
                }
            },
            SegmentType::DecoStop => println!("{:?}: {}m for {}min", stop.get_segment_type(),
                                              stop.get_depth(), stop.get_time()),
            _ => {}
        }
    }
}

fn pretty_print_segment_deco(depth: usize, time: usize, gas: &Gas,
                             seg_deco: Option<Vec<DiveSegment>>) {
    match seg_deco {
        Some(T) => {
            pretty_print_deco_stops(T);
            println!("Dive at {}m for {}min on {:?}\n", depth, time, gas);
        },

        None => {
            println!("Dive to {}m for {}min on {:?}\n", depth, time, gas);
        }
    };
}
fn main() {
    let ean32 = common::gas::Gas::new(0.68, 0.32, 0.0).unwrap();
    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();
    let pure_o2 = common::gas::Gas::new(0.0, 1.0, 0.0).unwrap();

    let mut dive = zhl16::ZHL16::new(&air);

    let ascent_rate = -18;
    let descent_rate = 30;

    println!("Descent rate: {}m/min, ascent rate: {}m/min\n", descent_rate,
             ascent_rate);

    let depth_1 = 45;
    let time_1 = 60;
    let first_segment = DiveSegment::new(SegmentType::DiveSegment, depth_1,
                                         time_1, ascent_rate,
                                         descent_rate);
    let depth_2 = 10;
    let time_2 = 20;
    let second_segment = DiveSegment::new(SegmentType::DiveSegment,
                                          depth_2, time_2, ascent_rate, descent_rate);

    let first_segment_deco = dive.add_bottom_time(&first_segment, &air);
    pretty_print_segment_deco(depth_1, time_1, &air, first_segment_deco);


    let second_segment_deco = dive.add_bottom_time(&second_segment, &air);
    pretty_print_segment_deco(depth_2, time_2, &air, second_segment_deco);

    let final_deco = dive.get_stops(ascent_rate, descent_rate, &air);
    pretty_print_deco_stops(final_deco);
}