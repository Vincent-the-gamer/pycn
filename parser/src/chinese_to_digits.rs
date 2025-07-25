use chinese2digits::take_number_from_string;

/// Converts Chinese numeral strings to their digit representation.
pub fn chinese_to_digits(chinese_num: String) -> String {
    let result = take_number_from_string(&chinese_num, true, true);
    result.replaced_text
}