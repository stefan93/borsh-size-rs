extern crate borsh_size;

use borsh_size::BorshSize;


#[derive(BorshSize)]
struct AnStruct {
    pub f1: &'static str,
    pub f2: String,
    pub f3: u16,
    pub f4: &'static [u8],
    pub f5: &'static [u8; 3],
    pub f6: &'static [&'static str],
    pub f7: Vec<String>
}

#[derive(BorshSize)]
enum AnEnum {
    A {
        f1: u16, //2
        f2: Vec<String> // 4 + n*(4+x)
    },
    B {
        f2: u8, //1
        f3: Vec<String>, // 4 + n*(4+x)
        f4: String // 4 + x
    }
}

#[test]
fn test_struct_size() {
    let a = AnStruct {
        f1: "test",                                 // 4 + 4
        f2: "test2".to_string(),                    // 4 + 5
        f3: 8u16,                                   // 2
        f4: &[1u8, 2u8],                            // 4 + 2
        f5: &[1u8, 2u8, 3u8],                       // 4 + 3
        f6: &["a", "ab"],                           // 4 + 4+1 + 4+2 
        f7: vec![String::from("a"), String::from("ab"), String::from("abc")] // 4 + 4+1 + 4+2 + 4+3
    };

    assert_eq!(47 + 22, a.calculate_borsh_size());
}


#[test]
fn test_enum_size() {
    let e = AnEnum::A { 
        f1: 3u16, // 2
        f2: vec![String::from("test")] // 4 + 4+4
    };
    
    assert_eq!(e.calculate_borsh_size(), 14);
}

#[test]
fn test_enum_size2() {
    let e = AnEnum::B { 
        f2: 1u8, //1
        f3: vec![String::from("a"), String::from("ab")], // 4 + 4+1 + 4+2
        f4: String::from("abc") // 4 + 3
    };
    
    assert_eq!(e.calculate_borsh_size(), 23);
}
