use double_conversion_sys::{
  double_conversion_DoubleToStringConverter_EcmaScriptConverter, double_conversion_DoubleToStringConverter_ToShortest,
  double_conversion_StringBuilder, double_conversion_Vector,
};
use std::os::raw::c_char;

const F64_STRING_MAX_LEN: usize = 34;

pub struct Buffer {
  bytes: [u8; F64_STRING_MAX_LEN],
}

impl Buffer {
  pub fn new() -> Self {
    Self {
      bytes: [0; F64_STRING_MAX_LEN],
    }
  }

  pub fn format(&mut self, v: f64) -> &str {
    let len = unsafe {
      let bytes_ptr: *mut c_char = self.bytes.as_mut_ptr() as *mut c_char;
      let mut builder = double_conversion_StringBuilder {
        buffer_: double_conversion_Vector {
          start_: bytes_ptr,
          length_: self.bytes.len() as _,
          _phantom_0: Default::default(),
        },
        position_: 0,
      };
      let converter = double_conversion_DoubleToStringConverter_EcmaScriptConverter();
      let is_success = double_conversion_DoubleToStringConverter_ToShortest(converter, v, (&mut builder) as *mut _);
      assert!(is_success);
      builder.position_
    };
    let s = &self.bytes[0..(len as usize)];
    unsafe { std::str::from_utf8_unchecked(s) }
  }
}

#[cfg(test)]
mod tests {
  use crate::Buffer;

  #[test]
  fn it_works() {
    let mut b = Buffer::new();
    assert_eq!(b.format(1.0), "1");
    assert_eq!(b.format(0.0), "0");
    assert_eq!(b.format(-0.0), "0");
    assert_eq!(b.format(0.3), "0.3");
    assert_eq!(b.format(f64::NAN), "NaN");
    assert_eq!(b.format(f64::INFINITY), "Infinity");
    assert_eq!(b.format(f64::NEG_INFINITY), "-Infinity");
    assert_eq!(b.format(to_f64(0x84ba6d321a08ee10)), "-6.941998689674754e-286");
    assert_eq!(b.format(to_f64(0x84ba6d321a08ee11)), "-6.9419986896747545e-286");
    assert_eq!(b.format(to_f64(0x84ba6d321a08ee12)), "-6.941998689674755e-286");
  }

  fn to_f64(x: u64) -> f64 {
    let bytes = x.to_be_bytes();
    f64::from_be_bytes(bytes)
  }
}
