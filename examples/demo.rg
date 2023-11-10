//*
* Multilined
* Block Comment
*/

main() {
    // single line comment
    println!("Hi mom!") // print line macro    
    {
        a u8 // immutable variable declaration of 8-bit unsigned integer
        assert!(a == 0) // default assignment is 0 when available
    }
    {
        s mut int = 5 // assign to mutable signed integer of arch-based size (ie 32/64-bit)
        s += 1 // incriment integer
        assert!(s == 6)
        s -= 2; // decrement integer
        b bool = s == 4 // test returns a bool of true
        assert!(b)
    }
    {
        n mut = 1.2 // implicit declaration of a mutable float
        n = (4 + n) * 2 // algebra and implicit conversion of int to float
    }
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
