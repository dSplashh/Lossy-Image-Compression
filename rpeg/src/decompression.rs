use array2::Array2;
use bitpack::bitpack::{getu, gets};
use csc411_image::Rgb;
use csc411_arith::chroma_of_index;
use crate::structures::*;


/*
DECOMPRESSION
-------------------------
rpeg image
codewords
get packed data from codewords
expand packed data (1 block to 4 crts)
- inverse dct
- chroma_of_index
crt (Y Pb Pr)
rgb float
rgb int
ppm image
*/


// extract quantized data from codewords
// input: Array2<u64>
// output: Array2<Quantized>
pub fn extract_encoded_data(codewords_array: Array2<u32>) -> Array2<EncodedQuanta> {

    // define new array2 of quantized structs
    let mut encoded_quanta_array = Array2::new(codewords_array.width(), codewords_array.height(), EncodedQuanta {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        pb_avg: 0,
        pr_avg: 0,
    });

    // iterate through codewords_array normally
    for (col, row, codeword) in codewords_array.iter_row_major() {

        // update quantized_array with new coefficients and averages
        *encoded_quanta_array.get_mut(col, row).unwrap() = EncodedQuanta {
            a: getu(*codeword as u64, 9, 23).unwrap(),
            b: gets(*codeword as u64, 5, 18).unwrap(),
            c: gets(*codeword as u64, 5, 13).unwrap(),
            d: gets(*codeword as u64, 5, 8).unwrap(),
            pb_avg: getu(*codeword as u64, 4, 4).unwrap(),
            pr_avg: getu(*codeword as u64, 4, 0).unwrap(),
        };
    }
    encoded_quanta_array    
}


// reverse index_of_chroma with chroma_of_index
// input: Array2<Quantized>
// output: Array2<Quantized> (updated)
pub fn chroma_of_index_array(encoded_quanta_array: Array2<EncodedQuanta>, decoded_quanta_array: Array2<Quantized>) -> Array2<Quantized> {
    
    // define new array2 of quantized structs
    let mut new_decoded_quanta_array = decoded_quanta_array.clone();

    // iterate through quantized_array normally
    for (c, r, encoded_quanta) in encoded_quanta_array.iter_row_major() {

        // update quantized_array with new pb and pr values while keeping a, b, c, and d values the same
        *new_decoded_quanta_array.get_mut(c, r).unwrap() = Quantized {
            a: decoded_quanta_array.get(c, r).unwrap().a,
            b: decoded_quanta_array.get(c, r).unwrap().b,
            c: decoded_quanta_array.get(c, r).unwrap().c,
            d: decoded_quanta_array.get(c, r).unwrap().d,
            pb_avg: chroma_of_index(encoded_quanta.pb_avg as usize),
            pr_avg: chroma_of_index(encoded_quanta.pr_avg as usize),
        };
    }
    new_decoded_quanta_array
}


// decode algorithm for b, c, d coefficients
fn decode(encoded: i64) -> f32 {
    encoded as f32 / 15.0 * 0.3
}


// decode coefficients from encoded quanta array
// input: Array2<EncodedQuanta>, Array2<Quantized>
// output: Array2<Quantized> (updated)
pub fn decode_coefficients_array(encoded_quanta_array: Array2<EncodedQuanta>, decoded_quanta_array: Array2<Quantized>) -> Array2<Quantized> {
    
    // define new array2 of quantized structs
    let mut new_decoded_quanta_array = decoded_quanta_array.clone();

    // iterate through quantized_array normally
    for (c, r, encoded_quanta) in encoded_quanta_array.iter_row_major() {

        // update quantized_array with new coefficients while keeping pb and pr values the same
        *new_decoded_quanta_array.get_mut(c, r).unwrap() = Quantized {
            a: encoded_quanta.a as f32 / 511.0,
            b: decode(encoded_quanta.b),
            c: decode(encoded_quanta.c),
            d: decode(encoded_quanta.d),
            pb_avg: decoded_quanta_array.get(c, r).unwrap().pb_avg,
            pr_avg: decoded_quanta_array.get(c, r).unwrap().pr_avg,
        };
    }
    new_decoded_quanta_array
}


// create new empty crt array
pub fn new_crt_array(rows: usize, cols: usize) -> Array2<Crt> {
    let empty_crt: Crt = Crt {
        y: 0.0,
        pb: 0.0,
        pr: 0.0,
    };
    let crt_array = Array2::new(rows, cols, empty_crt);
    crt_array
}


// set 2x2 blocks of crts in crt array to have corresponding average pb and pr values
// input: Array2<Quantized>, Array2<Crt>
// output: Array2<Crt> (updated)
pub fn set_pb_pr(decoded_quanta_array: Array2<Quantized>, crt_array: Array2<Crt>) -> Array2<Crt> {
    
    // define new array2 of crt structs
    let mut new_crt_array = crt_array.clone();

    // iterate through quantized_array in 2x2 blocks
    for (c, r, decoded_quanta) in decoded_quanta_array.iter_row_major() {

        // update crt_array with new pb and pr values while keeping y values the same
        // crt array is twice the size of the quantized array, so multiply c and r by 2
        *new_crt_array.get_mut(c * 2, r * 2).unwrap() = Crt {
            y: crt_array.get(c * 2, r * 2).unwrap().y,
            pb: decoded_quanta.pb_avg,
            pr: decoded_quanta.pr_avg,
        };
        *new_crt_array.get_mut(c * 2 + 1, r * 2).unwrap() = Crt {
            y: crt_array.get(c * 2 + 1, r * 2).unwrap().y,
            pb: decoded_quanta.pb_avg,
            pr: decoded_quanta.pr_avg,
        };
        *new_crt_array.get_mut(c * 2, r * 2 + 1).unwrap() = Crt {
            y: crt_array.get(c * 2, r * 2 + 1).unwrap().y,
            pb: decoded_quanta.pb_avg,
            pr: decoded_quanta.pr_avg,
        };
        *new_crt_array.get_mut(c * 2 + 1, r * 2 + 1).unwrap() = Crt {
            y: crt_array.get(c * 2 + 1, r * 2 + 1).unwrap().y,
            pb: decoded_quanta.pb_avg,
            pr: decoded_quanta.pr_avg,
        };
    }
    new_crt_array
}


// perform inverse dct
// input: Array2<Quantized>, Array2<Crt>
// output: Array2<Crt> (updated)
pub fn inverse_dct(quantized_array: Array2<Quantized>, crt_array: Array2<Crt>) -> Array2<Crt> {

    // define new array2 of crt structs
    let mut new_crt_array = crt_array.clone();

    // iterate through quantized_array normally
    for (c, r, quantized) in quantized_array.iter_row_major() {

        // get coefficients from current block
        let a_coeff = quantized.a as f32;
        let b_coeff = quantized.b as f32;
        let c_coeff = quantized.c as f32;
        let d_coeff = quantized.d as f32;

        // calculate y values from coefficients
        let y1 = a_coeff - b_coeff - c_coeff + d_coeff;
        let y2 = a_coeff - b_coeff + c_coeff - d_coeff;
        let y3 = a_coeff + b_coeff - c_coeff - d_coeff;
        let y4 = a_coeff + b_coeff + c_coeff + d_coeff;

        // update crt_array with new y values while keeping pb and pr values the same
        // crt array is twice the size of the quantized array, so multiply c and r by 2
        *new_crt_array.get_mut(c * 2, r * 2).unwrap() = Crt {
            y: y1,
            pb: crt_array.get(c * 2, r * 2).unwrap().pb,
            pr: crt_array.get(c * 2, r * 2).unwrap().pr
        };
        *new_crt_array.get_mut(c * 2 + 1, r * 2).unwrap() = Crt {
            y: y2,
            pb: crt_array.get(c * 2 + 1, r * 2).unwrap().pb,
            pr: crt_array.get(c * 2 + 1, r * 2).unwrap().pr
        };
        *new_crt_array.get_mut(c * 2, r * 2 + 1).unwrap() = Crt {
            y: y3,
            pb: crt_array.get(c * 2, r * 2 + 1).unwrap().pb,
            pr: crt_array.get(c * 2, r * 2 + 1).unwrap().pr
        };
        *new_crt_array.get_mut(c * 2 + 1, r * 2 + 1).unwrap() = Crt {
            y: y4,
            pb: crt_array.get(c * 2 + 1, r * 2 + 1).unwrap().pb,
            pr: crt_array.get(c * 2 + 1, r * 2 + 1).unwrap().pr
        };
    }
    new_crt_array
}


// turn crt into rgb float
fn crt_to_rgb_float(pixel: Crt) -> RgbFloat {
    RgbFloat {
        red: pixel.y + 1.402 * pixel.pr,
        green: pixel.y - 0.344136 * pixel.pb - 0.714136 * pixel.pr,
        blue: pixel.y + 1.772 * pixel.pb,
    }
}

// turn crt array into rgb float array
pub fn crt_array_to_rgb_float_array(crt_array: Array2<Crt>) -> Array2<RgbFloat> {
    
    // define new array2 of rgb floats
    let mut float_array = Array2::new(crt_array.width(), crt_array.height(), RgbFloat { red: 0.0, green: 0.0, blue: 0.0 });

    // iterate through crt_array and convert each pixel to rgb float using helper function
    for (c, r, pixel) in crt_array.iter_row_major() {
        *float_array.get_mut(c, r).unwrap() = crt_to_rgb_float(*pixel);
    }
    float_array
}

// rgb float to rgb int
fn rgb_float_to_rgb_int(pixel: RgbFloat) -> Rgb {
    Rgb {
        red: (pixel.red * 255.0) as u16,
        green: (pixel.green * 255.0) as u16,
        blue: (pixel.blue * 255.0) as u16,
    }
}


// converts a whole array2 of rgb floats to an array2 of rgb ints
pub fn rgb_float_array_to_rgb_int_array(float_array: Array2<RgbFloat>) -> Array2<Rgb> {
    
    // define new array2 of empty rgb ints
    let mut int_array = Array2::new(float_array.width(), float_array.height(), Rgb { red: 0, green: 0, blue: 0 });

    // iterate through each pixel in the array2, converting each to an rgb int using helper function
    for (c, r, pixel) in float_array.iter_row_major() {
        let pixel_clone = RgbFloat {
            red: (*pixel).red,
            green: (*pixel).green,
            blue: (*pixel).blue,
        };
        *int_array.get_mut(c, r).unwrap() = rgb_float_to_rgb_int(pixel_clone);
    }
    int_array
}