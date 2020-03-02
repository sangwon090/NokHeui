pub mod err;

use crate::hangul::err::InvalidHangulError;

pub const CHOSEONG_LIST:  [char; 19] = ['ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];
pub const JUNGSEONG_LIST: [char; 21] = ['ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ'];
pub const JONGSEONG_LIST: [char; 28] = ['\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];

pub fn is_hangul(hangul_char: char) -> bool {
    let hangul_char: u32 = hangul_char as u32;

    if hangul_char >= 0xAC00 && hangul_char <= 0xD7A3 {
        true
    } else {
        false
    }
}

pub fn disassemble_hangul(hangul_char: char) -> Result<(char, char, char), InvalidHangulError> {
    if !is_hangul(hangul_char) {
        return Err(InvalidHangulError);
    }

    let hangul_char: u32 = hangul_char as u32;

    let choseong:  usize = ((hangul_char - 0xAC00) / (21 * 28)) as usize;
    let jungseong: usize = ((hangul_char - 0xAC00 - ((choseong as u32) * 21 * 28)) / 28) as usize;
    let jongseong: usize =  (hangul_char - 0xAC00 - ((choseong as u32) * 21 * 28) - ((jungseong as u32) * 28)) as usize;

    return Ok((CHOSEONG_LIST[choseong], JUNGSEONG_LIST[jungseong], JONGSEONG_LIST[jongseong]));
}