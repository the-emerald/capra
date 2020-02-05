use capra::zhl16;
use capra::common;
use capra::common::deco_algorithm::DecoAlgorithm;
use capra::common::dive_segment::{DiveSegment, SegmentType};
use capra::common::gas::Gas;

fn gas_string(gas: &Gas) -> String {
    format!("{}/{}", (gas.fr_o2()*100.0) as usize, (gas.fr_he()*100.0) as usize)
}

fn pretty_print_deco_stops(stops: Vec<DiveSegment>, gas: &Gas) {
    //println!("{:?}", gas);
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
            SegmentType::DecoStop => println!("{:?}: {}m for {}min on {}", stop.get_segment_type(),
                                              stop.get_depth(), stop.get_time(), gas_string(gas)),
            _ => {}
        }
    }
}

fn pretty_print_segment_deco(depth: usize, time: usize, gas: &Gas,
                             seg_deco: Option<Vec<DiveSegment>>) {
    match seg_deco {
        Some(t) => {
            pretty_print_deco_stops(t, gas);
            println!("Dive at {}m for {}min on {}\n", depth, time, gas_string(gas));
        },

        None => {
            println!("Dive to {}m for {}min on {}\n", depth, time, gas_string(gas));
        }
    };
}

fn main() {
    let ean32 = common::gas::Gas::new(0.68, 0.32, 0.0).unwrap();
    let air = common::gas::Gas::new(0.79, 0.21, 0.0).unwrap();
    let pure_o2 = common::gas::Gas::new(0.0, 1.0, 0.0).unwrap();
    let trimix_18_45 = common::gas::Gas::new(0.37, 0.18, 0.45).unwrap();
    let trimix_21_35 = common::gas::Gas::new(0.44, 0.21, 0.35).unwrap();
    let trimix_31_25 = common::gas::Gas::new(0.44, 0.31, 0.25).unwrap();


    let half_o2 = common::gas::Gas::new(0.5, 0.5, 0.0).unwrap();

    let gf_low = 50;
    let gf_high = 85;
    let mut dive = zhl16::ZHL16::new(&air,
                                     zhl16::util::ZHL16B_N2_A,
                                     zhl16::util::ZHL16B_N2_B,
                                     zhl16::util::ZHL16B_N2_HALFLIFE,
                                     zhl16::util::ZHL16B_HE_A,
                                     zhl16::util::ZHL16B_HE_B,
                                     zhl16::util::ZHL16B_HE_HALFLIFE,
                                     gf_low, gf_high);

    let ascent_rate = -10;
    let descent_rate = 20;

    println!("Descent rate: {}m/min, ascent rate: {}m/min", descent_rate,
             ascent_rate);
    println!("GFL: {}, GFH: {}\n", gf_low, gf_high);

    let depth_1 = 60;
    let time_1 = 60;
    let first_segment = DiveSegment::new(SegmentType::DiveSegment, depth_1,
                                         time_1, ascent_rate,
                                         descent_rate);

    let first_segment_deco = dive.add_bottom_time(&first_segment, &trimix_18_45);
    pretty_print_segment_deco(depth_1, time_1, &trimix_18_45, first_segment_deco);

    let deco_stop_1_depth = 21;
    let deco_stop_1_time = 10;
    let deco_stop_1 = DiveSegment::new(SegmentType::DecoStop, deco_stop_1_depth,
                                       deco_stop_1_time, ascent_rate,
                                       descent_rate);

    let deco_stop_1_segment = dive.add_bottom_time(&deco_stop_1, &trimix_18_45);
    pretty_print_segment_deco(deco_stop_1_depth, deco_stop_1_time, &trimix_18_45, deco_stop_1_segment);

    let deco_stop_2_depth = 9;
    let deco_stop_2_time = 21;
    let deco_stop_2 = DiveSegment::new(SegmentType::DecoStop, deco_stop_2_depth,
                                       deco_stop_2_time, ascent_rate,
                                       descent_rate);

    let deco_stop_2_segment = dive.add_bottom_time(&deco_stop_2, &half_o2);
    pretty_print_segment_deco(deco_stop_2_depth, deco_stop_2_time, &half_o2, deco_stop_2_segment);

    let final_deco = dive.get_stops(ascent_rate, descent_rate, &pure_o2);
    pretty_print_deco_stops(final_deco, &pure_o2);
}