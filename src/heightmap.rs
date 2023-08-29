use crate::vector::*;
use crate::grayscale::*;
use crate::rgb::*;



pub struct Heightmap {
    height: Grayscale,
    normals: RGB,
}

impl Heightmap {
    // closed form expression for mantle hotspots. This can just be a fixed noise thing for now. For fun later, let it circulate and cycel around!
    // maybe takes t
    pub fn mantle_temp(&mut self, p: Vec2) -> f32 {
        
    }
    // closed form expression for critical angle, it can even just be a constant! sand would be like 30 degrees so thats tan30 = 0.577. Say 0.5.
    // Like it depends on the material, amount of vegetation, water, etc. Dry sand vs a vegetated hillside yaknow
    // ha ha and water has a critical angle of 0 xD
    // so sand can have gradient 0.5, thats the minimum
    // i guess rocks can have infinity or more than infinity
    // I could theoratically do the same shit for water as this
    // but yea 0.5- infinity, maybe its easier to go uniform and then tan it so 30 degrees to 89 degrees. but yeah sand is like 30, dirt could be 45, dirt w/ veg could be 60, rock could be 90
    pub fn critical_gradient(&mut self, p: Vec2) -> f32 {
        0.577
    }

    // harnessing chaos
    // so it would need to check each pixel, check the delta to neighbouring pixels, comparing that with the distance. Its got to shunt an amount of material downhill such that the angle is brought in line. the thing is
    // its just gonna need to go again once that one goes. So you dont want to go exactly, maybe you bring it halfway down or something, idk
    // you do share it among all the downhills too

    // returns if shunting was done
    // already not a good splittings because we care about shuntings where.
    // or if true it can just put more slidings for every neighbour
    // so we could make it so that ME and all downhills of me are SAME height such that VOLUME IS PRESERVED ALWAYS
    // so like add me and lower neighbours.  then avg. then all = that. then return true.
    // lower is a bool[8]
    // get_px
    // set_px
    // this shit always do wrappin yo
    pub fn shunt_material(&mut self, x: isize, y: isize, crit: f32) -> bool {
        let h = self.height.get_px(x, y);
        let nx = [x - 1, x - 1, x - 1, x, x, x + 1, x + 1, x + 1];
        let ny = [y - 1, y, y + 1, y - 1, y + 1, y - 1, y, y + 1];
        // delta =
        let nh = nx.iter().zip(ny.iter()).map(|(x, y)| self.height.get_px(*x, *y)).collect();
        let mut lower_count = 0;
        let mut lower = [false; 8];
        let mut acc_lower = h;
        for i in 0..8 {
            if nh[i] < h {
                lower[i] = true;
                lower_count += 1;
                acc_lower += nh[i];
            }
        }
        // then set all to be avg and return true lmao
        // oh but muh root2s
        // mabye it should jsut go in a random direction or in lowest direction, so just define a pdf based on heights.
        // ye imn doing lower not needed for delta

        // wats better: always go lowest
        // always go lowest but in random order maybe
        // vecdeque where one str8 downhill happens first then others happen
        // or its random
        // or like update coords go inna a hashtable so that its never duplicated and take one at a time random order

        // or pdf where its fair
        // or pdf where its pretty biased to lowest but not always? we shall c

        // these ideas relate to hydraulic erosion too, like wat pdf to use, and its also a fn of the direction of travel and momentum , etc


    }


    pub fn micro_weathering(&mut self) {

    }
    pub fn volcano(&mut self, p: Vec2, h: Vec2) {

    }
    pub fn volcano_crater(&mut self, p: Vec2, r: Vec2) {

    }
    pub fn rain_drop(&mut self, pos: Vec2) {

    }
    pub fn rain_storm(&mut self, start: Vec2, end: Vec2) {

    }
    pub fn shift(&mut self) {

    }
    pub fn height_ensurer(&mut self) {

    }
    // variation ensurer...
    
}