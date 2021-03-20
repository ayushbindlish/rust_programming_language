Two data type subsets are:
**Scalar** - represents a single value 
   1. integers: numbers without fractional component
        - i8 : signed 8 bit
        - u8 : unsigned 8 bit
        - i16 : signed 16 bit
        - u16 : unsigned 16 bit
        - i32 (default) : signed 32 bit      
        - u32 : unsigned 32 bit
        - i64 : signed 64 bit
        - u64 : unsigned 64 bit
        - i128 : signed 128 bit
        - u128 : unsigned 128 bit
   2. Decimal : 98_222
   3. Hex : 0xff
   4. Octal : 0o77
   5. Binary : 0b1111_0000
   6. Byte (u8 only) : b'A'
    
  2. floating-point numbers
     - f32 : single precision float
     - f64 (default) : double precision float
  3. booleans
  4. characters  
 
**Compound** - can group multiple values into one type
   - Tuple
     - fixed size
     - can have different types for different values 
   - Array
