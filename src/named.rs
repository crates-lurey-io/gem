//! CSS named colors as [`Srgb`] constants.
//!
//! All 148 CSS Color Module Level 4 named colors are available here as
//! `const` statics. Values are normalized to `[0.0, 1.0]` per channel.
//!
//! The British "grey" spellings (`GREY`, `DARK_GREY`, ...) are provided as aliases of their
//! American "gray" counterparts, matching the CSS spec's treatment of the two spellings as the
//! same color.
//!
//! ## Examples
//!
//! ```rust
//! use gem::named;
//! use gem::space::{Hsl, Srgb};
//!
//! let gold = named::GOLD;
//! let lighter = Hsl::from(gold).lighten(0.1);
//! let result = Srgb::from(lighter);
//! assert!(result.r > gold.r || result.g > gold.g);
//! ```

use crate::space::Srgb;

macro_rules! named_color {
    ($name:ident, $r:expr, $g:expr, $b:expr, $hex:literal) => {
        #[doc = concat!("CSS named color `", stringify!($name), "` (`#", $hex, "`).")]
        #[allow(clippy::cast_precision_loss)]
        pub const $name: Srgb = Srgb::new($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0);
    };
}

named_color!(ALICE_BLUE, 240, 248, 255, "F0F8FF");
named_color!(ANTIQUE_WHITE, 250, 235, 215, "FAEBD7");
named_color!(AQUA, 0, 255, 255, "00FFFF");
named_color!(AQUAMARINE, 127, 255, 212, "7FFFD4");
named_color!(AZURE, 240, 255, 255, "F0FFFF");
named_color!(BEIGE, 245, 245, 220, "F5F5DC");
named_color!(BISQUE, 255, 228, 196, "FFE4C4");
named_color!(BLACK, 0, 0, 0, "000000");
named_color!(BLANCHED_ALMOND, 255, 235, 205, "FFEBCD");
named_color!(BLUE, 0, 0, 255, "0000FF");
named_color!(BLUE_VIOLET, 138, 43, 226, "8A2BE2");
named_color!(BROWN, 165, 42, 42, "A52A2A");
named_color!(BURLY_WOOD, 222, 184, 135, "DEB887");
named_color!(CADET_BLUE, 95, 158, 160, "5F9EA0");
named_color!(CHARTREUSE, 127, 255, 0, "7FFF00");
named_color!(CHOCOLATE, 210, 105, 30, "D2691E");
named_color!(CORAL, 255, 127, 80, "FF7F50");
named_color!(CORNFLOWER_BLUE, 100, 149, 237, "6495ED");
named_color!(CORNSILK, 255, 248, 220, "FFF8DC");
named_color!(CRIMSON, 220, 20, 60, "DC143C");
named_color!(CYAN, 0, 255, 255, "00FFFF");
named_color!(DARK_BLUE, 0, 0, 139, "00008B");
named_color!(DARK_CYAN, 0, 139, 139, "008B8B");
named_color!(DARK_GOLDENROD, 184, 134, 11, "B8860B");
named_color!(DARK_GRAY, 169, 169, 169, "A9A9A9");
#[doc = "CSS named color `DARK_GREY` (`#A9A9A9`), an alias of [`DARK_GRAY`]."]
pub const DARK_GREY: Srgb = DARK_GRAY;
named_color!(DARK_GREEN, 0, 100, 0, "006400");
named_color!(DARK_KHAKI, 189, 183, 107, "BDB76B");
named_color!(DARK_MAGENTA, 139, 0, 139, "8B008B");
named_color!(DARK_OLIVE_GREEN, 85, 107, 47, "556B2F");
named_color!(DARK_ORANGE, 255, 140, 0, "FF8C00");
named_color!(DARK_ORCHID, 153, 50, 204, "9932CC");
named_color!(DARK_RED, 139, 0, 0, "8B0000");
named_color!(DARK_SALMON, 233, 150, 122, "E9967A");
named_color!(DARK_SEA_GREEN, 143, 188, 143, "8FBC8F");
named_color!(DARK_SLATE_BLUE, 72, 61, 139, "483D8B");
named_color!(DARK_SLATE_GRAY, 47, 79, 79, "2F4F4F");
#[doc = "CSS named color `DARK_SLATE_GREY` (`#2F4F4F`), an alias of [`DARK_SLATE_GRAY`]."]
pub const DARK_SLATE_GREY: Srgb = DARK_SLATE_GRAY;
named_color!(DARK_TURQUOISE, 0, 206, 209, "00CED1");
named_color!(DARK_VIOLET, 148, 0, 211, "9400D3");
named_color!(DEEP_PINK, 255, 20, 147, "FF1493");
named_color!(DEEP_SKY_BLUE, 0, 191, 255, "00BFFF");
named_color!(DIM_GRAY, 105, 105, 105, "696969");
#[doc = "CSS named color `DIM_GREY` (`#696969`), an alias of [`DIM_GRAY`]."]
pub const DIM_GREY: Srgb = DIM_GRAY;
named_color!(DODGER_BLUE, 30, 144, 255, "1E90FF");
named_color!(FIREBRICK, 178, 34, 34, "B22222");
named_color!(FLORAL_WHITE, 255, 250, 240, "FFFAF0");
named_color!(FOREST_GREEN, 34, 139, 34, "228B22");
named_color!(FUCHSIA, 255, 0, 255, "FF00FF");
named_color!(GAINSBORO, 220, 220, 220, "DCDCDC");
named_color!(GHOST_WHITE, 248, 248, 255, "F8F8FF");
named_color!(GOLD, 255, 215, 0, "FFD700");
named_color!(GOLDENROD, 218, 165, 32, "DAA520");
named_color!(GRAY, 128, 128, 128, "808080");
#[doc = "CSS named color `GREY` (`#808080`), an alias of [`GRAY`]."]
pub const GREY: Srgb = GRAY;
named_color!(GREEN, 0, 128, 0, "008000");
named_color!(GREEN_YELLOW, 173, 255, 47, "ADFF2F");
named_color!(HONEYDEW, 240, 255, 240, "F0FFF0");
named_color!(HOT_PINK, 255, 105, 180, "FF69B4");
named_color!(INDIAN_RED, 205, 92, 92, "CD5C5C");
named_color!(INDIGO, 75, 0, 130, "4B0082");
named_color!(IVORY, 255, 255, 240, "FFFFF0");
named_color!(KHAKI, 240, 230, 140, "F0E68C");
named_color!(LAVENDER, 230, 230, 250, "E6E6FA");
named_color!(LAVENDER_BLUSH, 255, 240, 245, "FFF0F5");
named_color!(LAWN_GREEN, 124, 252, 0, "7CFC00");
named_color!(LEMON_CHIFFON, 255, 250, 205, "FFFACD");
named_color!(LIGHT_BLUE, 173, 216, 230, "ADD8E6");
named_color!(LIGHT_CORAL, 240, 128, 128, "F08080");
named_color!(LIGHT_CYAN, 224, 255, 255, "E0FFFF");
named_color!(LIGHT_GOLDENROD_YELLOW, 250, 250, 210, "FAFAD2");
named_color!(LIGHT_GRAY, 211, 211, 211, "D3D3D3");
#[doc = "CSS named color `LIGHT_GREY` (`#D3D3D3`), an alias of [`LIGHT_GRAY`]."]
pub const LIGHT_GREY: Srgb = LIGHT_GRAY;
named_color!(LIGHT_GREEN, 144, 238, 144, "90EE90");
named_color!(LIGHT_PINK, 255, 182, 193, "FFB6C1");
named_color!(LIGHT_SALMON, 255, 160, 122, "FFA07A");
named_color!(LIGHT_SEA_GREEN, 32, 178, 170, "20B2AA");
named_color!(LIGHT_SKY_BLUE, 135, 206, 250, "87CEFA");
named_color!(LIGHT_SLATE_GRAY, 119, 136, 153, "778899");
#[doc = "CSS named color `LIGHT_SLATE_GREY` (`#778899`), an alias of [`LIGHT_SLATE_GRAY`]."]
pub const LIGHT_SLATE_GREY: Srgb = LIGHT_SLATE_GRAY;
named_color!(LIGHT_STEEL_BLUE, 176, 196, 222, "B0C4DE");
named_color!(LIGHT_YELLOW, 255, 255, 224, "FFFFE0");
named_color!(LIME, 0, 255, 0, "00FF00");
named_color!(LIME_GREEN, 50, 205, 50, "32CD32");
named_color!(LINEN, 250, 240, 230, "FAF0E6");
named_color!(MAGENTA, 255, 0, 255, "FF00FF");
named_color!(MAROON, 128, 0, 0, "800000");
named_color!(MEDIUM_AQUAMARINE, 102, 205, 170, "66CDAA");
named_color!(MEDIUM_BLUE, 0, 0, 205, "0000CD");
named_color!(MEDIUM_ORCHID, 186, 85, 211, "BA55D3");
named_color!(MEDIUM_PURPLE, 147, 112, 219, "9370DB");
named_color!(MEDIUM_SEA_GREEN, 60, 179, 113, "3CB371");
named_color!(MEDIUM_SLATE_BLUE, 123, 104, 238, "7B68EE");
named_color!(MEDIUM_SPRING_GREEN, 0, 250, 154, "00FA9A");
named_color!(MEDIUM_TURQUOISE, 72, 209, 204, "48D1CC");
named_color!(MEDIUM_VIOLET_RED, 199, 21, 133, "C71585");
named_color!(MIDNIGHT_BLUE, 25, 25, 112, "191970");
named_color!(MINT_CREAM, 245, 255, 250, "F5FFFA");
named_color!(MISTY_ROSE, 255, 228, 225, "FFE4E1");
named_color!(MOCCASIN, 255, 228, 181, "FFE4B5");
named_color!(NAVAJO_WHITE, 255, 222, 173, "FFDEAD");
named_color!(NAVY, 0, 0, 128, "000080");
named_color!(OLD_LACE, 253, 245, 230, "FDF5E6");
named_color!(OLIVE, 128, 128, 0, "808000");
named_color!(OLIVE_DRAB, 107, 142, 35, "6B8E23");
named_color!(ORANGE, 255, 165, 0, "FFA500");
named_color!(ORANGE_RED, 255, 69, 0, "FF4500");
named_color!(ORCHID, 218, 112, 214, "DA70D6");
named_color!(PALE_GOLDENROD, 238, 232, 170, "EEE8AA");
named_color!(PALE_GREEN, 152, 251, 152, "98FB98");
named_color!(PALE_TURQUOISE, 175, 238, 238, "AFEEEE");
named_color!(PALE_VIOLET_RED, 219, 112, 147, "DB7093");
named_color!(PAPAYA_WHIP, 255, 239, 213, "FFEFD5");
named_color!(PEACH_PUFF, 255, 218, 185, "FFDAB9");
named_color!(PERU, 205, 133, 63, "CD853F");
named_color!(PINK, 255, 192, 203, "FFC0CB");
named_color!(PLUM, 221, 160, 221, "DDA0DD");
named_color!(POWDER_BLUE, 176, 224, 230, "B0E0E6");
named_color!(PURPLE, 128, 0, 128, "800080");
named_color!(REBECCA_PURPLE, 102, 51, 153, "663399");
named_color!(RED, 255, 0, 0, "FF0000");
named_color!(ROSY_BROWN, 188, 143, 143, "BC8F8F");
named_color!(ROYAL_BLUE, 65, 105, 225, "4169E1");
named_color!(SADDLE_BROWN, 139, 69, 19, "8B4513");
named_color!(SALMON, 250, 128, 114, "FA8072");
named_color!(SANDY_BROWN, 244, 164, 96, "F4A460");
named_color!(SEA_GREEN, 46, 139, 87, "2E8B57");
named_color!(SEASHELL, 255, 245, 238, "FFF5EE");
named_color!(SIENNA, 160, 82, 45, "A0522D");
named_color!(SILVER, 192, 192, 192, "C0C0C0");
named_color!(SKY_BLUE, 135, 206, 235, "87CEEB");
named_color!(SLATE_BLUE, 106, 90, 205, "6A5ACD");
named_color!(SLATE_GRAY, 112, 128, 144, "708090");
#[doc = "CSS named color `SLATE_GREY` (`#708090`), an alias of [`SLATE_GRAY`]."]
pub const SLATE_GREY: Srgb = SLATE_GRAY;
named_color!(SNOW, 255, 250, 250, "FFFAFA");
named_color!(SPRING_GREEN, 0, 255, 127, "00FF7F");
named_color!(STEEL_BLUE, 70, 130, 180, "4682B4");
named_color!(TAN, 210, 180, 140, "D2B48C");
named_color!(TEAL, 0, 128, 128, "008080");
named_color!(THISTLE, 216, 191, 216, "D8BFD8");
named_color!(TOMATO, 255, 99, 71, "FF6347");
named_color!(TURQUOISE, 64, 224, 208, "40E0D0");
named_color!(VIOLET, 238, 130, 238, "EE82EE");
named_color!(WHEAT, 245, 222, 179, "F5DEB3");
named_color!(WHITE, 255, 255, 255, "FFFFFF");
named_color!(WHITE_SMOKE, 245, 245, 245, "F5F5F5");
named_color!(YELLOW, 255, 255, 0, "FFFF00");
named_color!(YELLOW_GREEN, 154, 205, 50, "9ACD32");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_is_correct() {
        assert!((RED.r - 1.0).abs() < 1e-6);
        assert!(RED.g.abs() < 1e-6);
        assert!(RED.b.abs() < 1e-6);
    }

    #[test]
    fn black_is_zero() {
        assert!(BLACK.r.abs() < 1e-6);
        assert!(BLACK.g.abs() < 1e-6);
        assert!(BLACK.b.abs() < 1e-6);
    }

    #[test]
    fn white_is_one() {
        assert!((WHITE.r - 1.0).abs() < 1e-6);
        assert!((WHITE.g - 1.0).abs() < 1e-6);
        assert!((WHITE.b - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cornflower_blue_is_correct() {
        assert!((CORNFLOWER_BLUE.r - 100.0 / 255.0).abs() < 1e-5);
        assert!((CORNFLOWER_BLUE.g - 149.0 / 255.0).abs() < 1e-5);
        assert!((CORNFLOWER_BLUE.b - 237.0 / 255.0).abs() < 1e-5);
    }

    #[test]
    fn gold_is_correct() {
        assert!((GOLD.r - 1.0).abs() < 1e-6);
        assert!((GOLD.g - 215.0 / 255.0).abs() < 1e-5);
        assert!(GOLD.b.abs() < 1e-6);
    }
}
