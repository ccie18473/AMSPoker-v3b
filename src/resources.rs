use crate::prelude::*;
use std::collections::HashMap;
pub struct TResources {
    pub files: HashMap<usize, String>,
    pub images: Vec<graphics::Image>,
}
impl TResources {
    pub fn new_resources(ctx: &mut Context) -> Self {
        let mut files = HashMap::new();

        files.insert(0, "c01.png".to_string());
        files.insert(1, "c02.png".to_string());
        files.insert(2, "c03.png".to_string());
        files.insert(3, "c04.png".to_string());
        files.insert(4, "c05.png".to_string());
        files.insert(5, "c06.png".to_string());
        files.insert(6, "c07.png".to_string());
        files.insert(7, "c08.png".to_string());
        files.insert(8, "c09.png".to_string());
        files.insert(9, "c10.png".to_string());
        files.insert(10, "c11.png".to_string());
        files.insert(11, "c12.png".to_string());
        files.insert(12, "c13.png".to_string());
        files.insert(13, "d01.png".to_string());
        files.insert(14, "d02.png".to_string());
        files.insert(15, "d03.png".to_string());
        files.insert(16, "d04.png".to_string());
        files.insert(17, "d05.png".to_string());
        files.insert(18, "d06.png".to_string());
        files.insert(19, "d07.png".to_string());
        files.insert(20, "d08.png".to_string());
        files.insert(21, "d09.png".to_string());
        files.insert(22, "d10.png".to_string());
        files.insert(23, "d11.png".to_string());
        files.insert(24, "d12.png".to_string());
        files.insert(25, "d13.png".to_string());
        files.insert(26, "h01.png".to_string());
        files.insert(27, "h02.png".to_string());
        files.insert(28, "h03.png".to_string());
        files.insert(29, "h04.png".to_string());
        files.insert(30, "h05.png".to_string());
        files.insert(31, "h06.png".to_string());
        files.insert(32, "h07.png".to_string());
        files.insert(33, "h08.png".to_string());
        files.insert(34, "h09.png".to_string());
        files.insert(35, "h10.png".to_string());
        files.insert(36, "h11.png".to_string());
        files.insert(37, "h12.png".to_string());
        files.insert(38, "h13.png".to_string());
        files.insert(39, "s01.png".to_string());
        files.insert(40, "s02.png".to_string());
        files.insert(41, "s03.png".to_string());
        files.insert(42, "s04.png".to_string());
        files.insert(43, "s05.png".to_string());
        files.insert(44, "s06.png".to_string());
        files.insert(45, "s07.png".to_string());
        files.insert(46, "s08.png".to_string());
        files.insert(47, "s09.png".to_string());
        files.insert(48, "s10.png".to_string());
        files.insert(49, "s11.png".to_string());
        files.insert(50, "s12.png".to_string());
        files.insert(51, "s13.png".to_string());
        files.insert(52, "Rust.png".to_string());
        files.insert(53, "Bet.png".to_string());
        files.insert(54, "Max_Bet.png".to_string());
        files.insert(55, "Retry.png".to_string());
        files.insert(56, "Credit.png".to_string());
        files.insert(57, "Double.png".to_string());
        files.insert(58, "Red.png".to_string());
        files.insert(59, "Black.png".to_string());
        files.insert(60, "Ferris.png".to_string());

        let mut images = Vec::new();

        for index in 0..=60 {
            let filename = &files[&index];
            let image = graphics::Image::new(ctx, filename).unwrap();
            images.push(image);
        }
        Self { files, images }
    }
    pub fn get_image(&mut self, index: usize) -> graphics::Image {
        self.images[index].clone()
    }
}
