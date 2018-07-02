extern "C" {
    fn printi(c: i64);
    fn printui(c: u64);
    fn prints_l(bytes: *const u8, len: i32);
    fn printn(name: u64);
}

pub fn print_i64(i: i64) {
    unsafe {
        printi(i);
    }
}

pub fn print_u64(u: u64) {
    unsafe {
        printui(u);
    }
}

pub fn print_str(s: &str) {
    unsafe {
        let bytes = s.as_bytes();
        prints_l(bytes.as_ptr(), bytes.len() as i32);
    }
}

pub fn print_name(name: u64) {
    unsafe {
        printn(name);
    }
}

pub fn str_to_name(s: &str) -> u64 {
    let mut res: u64 = 0;
    let mut bytes = s.bytes();
    for i in 0..12 {
        let mut b: u64 = 0;
        if let Some(c) = bytes.next() {
            b = byte_to_base32(c) as u64;
        }
        if i < 12 {
            b &= 0x1f;
            b <<= 64 - 5 * (i + 1);
        } else {
            b &= 0x0f;
        }
        res |= b;
    }
    res
}

#[inline]
fn byte_to_base32(b: u8) -> u8 {
    match b {
        97...122 => b - 97 + 6,
        49...54 => b - 49 + 1,
        _ => 0,
    }
}

//    static constexpr  char char_to_symbol( char c ) {
//       if( c >= 'a' && c <= 'z' )
//          return (c - 'a') + 6;
//       if( c >= '1' && c <= '5' )
//          return (c - '1') + 1;
//       return 0;
//    }

//    static constexpr uint64_t string_to_name( const char* str ) {

//       uint32_t len = 0;
//       while( str[len] ) ++len;

//       uint64_t value = 0;

//       for( uint32_t i = 0; i <= 12; ++i ) {
//          uint64_t c = 0;
//          if( i < len && i <= 12 ) c = uint64_t(char_to_symbol( str[i] ));

//          if( i < 12 ) {
//             c &= 0x1f;
//             c <<= 64-5*(i+1);
//          }
//          else {
//             c &= 0x0f;
//          }

//          value |= c;
//       }

//       return value;
//    }
