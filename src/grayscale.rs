use crate::vector::*;

pub struct Grayscale {
    w: isize,
    h: isize,
    data: Vec<f32>,
}
impl Grayscale {
    pub fn zero(w: isize, h: isize) -> Self {
        Grayscale { w, h, data: vec![0.0; w as usize * h as usize] }
    }
    pub fn get_px(&mut self, x: isize, y: isize) -> f32 {
        let x = (x % self.w) as usize;
        let y = (y % self.h) as usize;
        self.data[y * self.w as usize + x]
    }
    pub fn set_px(&mut self, x: isize, y: isize, val: f32) {
        let x = (x % self.w) as usize;
        let y = (y % self.h) as usize;
        self.data[y * self.w as usize + x] = val;
    }
    
    pub fn cone_kernel(w: usize, h: usize, zh: f32) -> Self {
        let center_x = (w / 2) as f32;
        let center_y = (h / 2) as f32;
        
        let data = (0..h)
            .flat_map(|y| {
                (0..w).map(move |x| {
                    let distance_squared = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                    if distance_squared <= zh {
                        1.0
                    } else {
                        0.0
                    }
                })
            })
            .collect();
    
        Self { w, data }
    }
    // this would have to be eroded in a way thats volume preserving like a sand pile
    // in a way its all just fluids lol
    // erode_critical_gradient(1.0) so its like 45 degrees max
    // critical angles of repose
    // slide(angle) where its fairly recursive. for bonus points allow angle to be set based on noise or something
    // so maybe you wuold have a queue where you did slide at a point, where its able to offload a portion thats above the CAoR down, then it has to recurse down
    // OK so but does this shit go on Grayscale or does it go on heightmap.
    
    pub fn wrapping_blit(&mut self, other: Grayscale, x: usize, y: usize) {

    }
    pub fn wrapping_convolve(&mut self, other: Grayscale) {

    }
    pub fn wrapping_translate(&mut self, offset: Vec2) {

    }
    pub fn median(&self) -> f32 {
        let mut sorted_data = self.data.clone(); // Clone the data to avoid modifying the original vector.
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sort the data in ascending order.
    
        let middle = sorted_data.len() / 2;
        if sorted_data.len() % 2 == 0 {
            // Even number of elements, so take the average of the two middle values.
            (sorted_data[middle - 1] + sorted_data[middle]) / 2.0
        } else {
            // Odd number of elements, so the middle value is the median.
            sorted_data[middle]
        }
    }
    pub fn average(&self) -> f32 {
        let sum: f32 = self.data.iter().sum(); // Sum all elements in the data vector.
        let count = self.data.len() as f32; // Get the count of elements as a floating-point number.
    
        sum / count // Calculate and return the average.
    }
}