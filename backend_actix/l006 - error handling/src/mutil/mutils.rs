/**
 * parameters are consumed (because are owned by this fn)
 * returns an owned String
 */
pub fn concat_and_consume(str1: String, str2: String) -> String {
    str1 + &str2
}
