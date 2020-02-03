use capra::zhl16;
use capra::common;
use capra::common::deco_algorithm::DecoAlgorithm;
use capra::common::dive_segment::{DiveSegment, SegmentType};
use capra::common::gas::Gas;

fn pretty_print_deco_stops(stops: Vec<DiveSegment>, gas: &Gas) {
    println!("{:?}", gas);
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
        Some(t) => {
            pretty_print_deco_stops(t, gas);
            println!("Dive at {}m for {}min\n", depth, time);
        },

        None => {
            println!("{:?}", gas);
            println!("Dive to {}m for {}min\n", depth, time);
        }
    };
}
fn main() {
    let ean32 = common::gas::Gas::new(0.68, 0.32, 0.0).unwrap();
    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();
    let pure_o2 = common::gas::Gas::new(0.0, 1.0, 0.0).unwrap();
    let trimix_21_35 = common::gas::Gas::new(0.44, 0.21, 0.35).unwrap();
    let half_o2 = common::gas::Gas::new(0.5, 0.5, 0.0).unwrap();

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

    let first_segment_deco = dive.add_bottom_time(&first_segment, &trimix_21_35);
    pretty_print_segment_deco(depth_1, time_1, &trimix_21_35, first_segment_deco);

    let depth_2 = 10;
    let time_2 = 20;
    let second_segment = DiveSegment::new(SegmentType::DiveSegment,
                                          depth_2, time_2, ascent_rate, descent_rate);

    let second_segment_deco = dive.add_bottom_time(&second_segment, &half_o2);
    pretty_print_segment_deco(depth_2, time_2, &half_o2, second_segment_deco);

    let final_deco = dive.get_stops(ascent_rate, descent_rate, &half_o2);
    pretty_print_deco_stops(final_deco, &half_o2);
}