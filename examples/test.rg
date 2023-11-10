//*
* Multilined
* Block Comment
*/

main() {
    // single line comment
    println!("Hi mom!")    
}

/// Data Structure
ColorRGB {
    red: u8,
    green: u8,
    blue: u8,
}

/// Enumeration / Taged Union
FavoriteColor {
    Red,
    Green,
    Blue,

    // Sub-Structure
    CustomColor(ColorRGB),
}

impl FavoriteColor {
    /// Documentation comment
    /// can be multilined
    is_red pub (&self) bool {
        self == Red
    }
}
