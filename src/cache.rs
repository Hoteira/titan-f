use crate::Map;
use crate::Vec;
use crate::render::Metrics;

pub struct Cache (Map<(u32, usize), (Metrics, Vec<u8>)>);

impl Cache {
    pub fn new() -> Self {
        Cache(Map::new())
    }

    pub fn flush(&mut self) {
        self.0.clear();
    }

    pub fn get(&self, id: u32, size: usize) -> Option<&(Metrics, Vec<u8>)> {
         self.0.get(&(id, size))
    }

    pub fn set(&mut self, size: u32, scale: usize, metrics: Metrics, data: Vec<u8>) {
        self.0.insert((size, scale), (metrics, data));
    }
}