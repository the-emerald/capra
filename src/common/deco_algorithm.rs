use crate::common::deco_stop;
use crate::common::gas;

pub trait DecoAlgorithm {
    fn add_bottom_time(&mut self, depth: usize, time: usize, gas: &gas::Gas);
    fn get_stops(&self, gas: &gas::Gas) -> Vec<deco_stop::DecoStop>;
}