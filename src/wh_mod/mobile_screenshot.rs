
use image::{GenericImageView, ImageBuffer};
use imagesize;
const MOBILE_SCREENSHOT_SIZE: [[usize; 2]; 40] = [
[1290,2796],
[1179,2556],
[1284,2778],
[1170,2532],
[750,1334],
[1080,2340],
[1125,2436],
[1242,2688],
[828,1792],
[1242,2208],
[640,1136],
[1344,2772],
[1080,2376],
[1200,2640],
[1080,2310],
[1080,2312],
[1176,2400],
[1440,3120],
[1080,2244],
[1080,2240],
[1080,1920],
[1440,2560],
[480,854],
[1080,2160],
[1080,2270],
[1080,2246],
[720,1280],
[1440,3200],
[1080,2400],
[1080,2600],
[1440,3040],
[720,1560],
[1080,2280],
[1440,2960],
[1080,2040],
[720,1520],
[720,1544],
[1080,2460],
[1080,2242],
[1440,2560],

];


pub fn has_mobile_screenshot(data:&Vec<u8>)-> bool{
    match imagesize::blob_size(data) {
        Ok(size) =>{

            for [width,height] in MOBILE_SCREENSHOT_SIZE {
                if size.width == width  && size.height == height {
                    return true;
                }
               
            }
            return false
        },
        Err(why) => {
            return  false
        },
    }
    return  false
}