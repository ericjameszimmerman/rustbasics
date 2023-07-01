use std::mem;

pub fn execute(value: String) -> () {
    println!("Base Converting {value}");
    convert_value(value.as_str());
}

fn convert_value(input: &str) -> () {
    let trimmed_input = input.trim();
    let mut handled: bool = false;

    if trimmed_input.starts_with("0x") || trimmed_input.starts_with("0X") {
        let hex_value = &trimmed_input[2..];
        if hex_value.chars().all(|c| c.is_ascii_hexdigit()) {
            let decimal_value = hexstr_to_decimal(input);
            print_output(decimal_value);
            handled = true;
        }
    } else if trimmed_input.starts_with("0b") || trimmed_input.starts_with("0B") {
        let binary_value = &trimmed_input[2..];
        if binary_value.chars().all(|c| c == '0' || c == '1') {
            let decimal_value = binstr_to_decimal(input);
            print_output(decimal_value);
            handled = true;
        }
    } else if is_float(trimmed_input) {
        let decimal_value = floatstr_to_raw(trimmed_input);
        print_output(decimal_value);
        handled = true;

    } else if trimmed_input.chars().all(|c| c.is_ascii_digit()) {
        let decimal_value = decstr_to_decimal(input);
        print_output(decimal_value);
        handled = true;
    }

    if !handled {
        print!("Invalid Input...");
    }
}

fn is_float(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

fn hexstr_to_decimal(hex_string: &str) -> u64 {
    let clean_hex = hex_string.trim_start_matches("0x").trim_start_matches("0X");
    let hex_value = u64::from_str_radix(clean_hex, 16).expect("Invalid Hexadecimal Input");
    hex_value
}

fn binstr_to_decimal(bin_string: &str) -> u64 {
    let clean_bin = bin_string.trim_start_matches("0b").trim_start_matches("0B");
    let bin_value = u64::from_str_radix(clean_bin, 2).expect("Invalid Binary Input");
    bin_value
}

fn decstr_to_decimal(dec_string: &str) -> u64 {
    let dec_value = u64::from_str_radix(dec_string, 10).expect("Invalid Decimal Input");
    dec_value
}

fn floatstr_to_raw(float_string: &str) -> u64 {
    let double_value = float_string.parse::<f64>().expect("Invalid Floating point Input");
    let dec_value: u64 = unsafe { mem::transmute(double_value) };
    dec_value
}

fn print_output(input: u64) {
    let decimal_display = format!("{}", input);
    let hex_display = format!("{:x}", input);
    let octal_display = format!("{:o}", input);
    let binary_display = format!("{:b}", input);
    let double_value: f64 = unsafe { mem::transmute(input) };
    let u32_value: u32 = input as u32;
    let float_value: f32 = unsafe { mem::transmute(u32_value) };

    println!("dec: {}", decimal_display);
    println!("hex: 0x{}", hex_display);
    println!("oct: 0o{}", octal_display);
    println!("bin: 0b{}", binary_display);
    println!("f64: {:.10} ({:.10e})", double_value, double_value);
    println!("f32: {:.10} ({:.10e})", float_value, float_value);
}