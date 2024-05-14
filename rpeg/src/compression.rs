use array2::Array2;
use csc411_image::Rgb;
use bitpack::bitpack;
use csc411_arith::index_of_chroma;
use crate::structures::*;


/*
data formats for COMPRESSION, in order
-------------------------
ppm image
rgb int
rgb float
crt (Y Pb Pr)
packed crt blocks:
- discrete cosine function on 4 Y values to get a, b, c, d coefficients
- average Pb and Pr values
    - convert Pb and Pr averages to 4-bit values using index_of_chroma
- convert b, c, d coefficients to 5-bit values
pack all of the data from each block into codewords
*/


// rgb int to rgb float
fn rgb_int_to_rgb_float(pixel: Rgb) -> RgbFloat {
    let new_pixel = RgbFloat {
        red: pixel.red as f32 / 255.0,
        green: pixel.green as f32 / 255.0,
        blue: pixel.blue as f32 / 255.0,
    };
    new_pixel
}


// converts a whole array2 of rgb ints to an array2 of rgb floats
pub fn rgb_int_array_to_rgb_float_array(int_array2: Array2<Rgb>) -> Array2<RgbFloat> {
    
    // define new array2 of empty rgb floats
    let mut float_array2 = Array2::new(int_array2.width(), int_array2.height(), RgbFloat { red: 0.0, green: 0.0, blue: 0.0 });

    // iterate through each pixel in the array2, converting each to an rgb float using helper function
    for (c, r, pixel) in int_array2.iter_row_major() {
        let pixel_clone = Rgb {
            red: (*pixel).red,
            green: (*pixel).green,
            blue: (*pixel).blue,
        };
        *float_array2.get_mut(c, r).unwrap() = rgb_int_to_rgb_float(pixel_clone);
    }

    float_array2

}


// turn rgb float into crt
// TODO: double check
fn rgb_float_to_crt(pixel: RgbFloat) -> Crt {
    Crt {
        y: 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue,
        pb: -0.168736 * pixel.red - 0.331264 * pixel.green + 0.5 * pixel.blue,
        pr: 0.5 * pixel.red - 0.418688 * pixel.green - 0.081312 * pixel.blue,
    }
}


// turn rgb float array into crt array
pub fn rgb_float_array_to_crt_array(float_array: Array2<RgbFloat>) -> Array2<Crt> {
    
    // define new array2 of crt structs
    let mut crt_array2 = Array2::new(float_array.width(), float_array.height(), Crt { y: 0.0, pb: 0.0, pr: 0.0 });

    // iterate through float_array and convert each pixel to crt using helper function
    for (c, r, pixel) in float_array.iter_row_major() {
        *crt_array2.get_mut(c, r).unwrap() = rgb_float_to_crt(*pixel);
    }

    crt_array2
}


// create an array2 of empty quantized structs
pub fn new_quantized_array(width: usize, height: usize) -> Array2<Quantized> {

    // define empty quantized struct
    let empty_quantized: Quantized = Quantized {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        pb_avg: 0.0,
        pr_avg: 0.0
    };

    // define new array2 of empty quantized structs, and return it
    let quantized_array = Array2::new(width, height, empty_quantized);
    quantized_array
}


// perform discrete cosine transform (dct) on blocks of 4 Y values
// input: Array2<Crt>, Array2<Quantized>
// output: Array2<Quantized> (updated)
pub fn dct(crt_array: Array2<Crt>, quantized_array: Array2<Quantized>) -> Array2<Quantized> {
    
    let mut new_quantized_array = quantized_array.clone();

    // iterate through crt_array in 2x2 blocks
    for (c, r, _) in crt_array.iter_row_major() {

        // if we're at both an even column and row, we are at the start of a block
        if c % 2 == 0 && r % 2 == 0 {

            // get y values from each pixel in current block
            let y1 = crt_array.get(c, r).unwrap().y;
            let y2 = crt_array.get(c + 1, r).unwrap().y;
            let y3 = crt_array.get(c, r + 1).unwrap().y;
            let y4 = crt_array.get(c + 1, r + 1).unwrap().y;

            // perform dct on block
            let a_coeff = (y4 + y3 + y2 + y1) / 4.0;
            let b_coeff = (y4 + y3 - y2 - y1) / 4.0;
            let c_coeff = (y4 - y3 + y2 - y1) / 4.0;
            let d_coeff = (y4 - y3 - y2 + y1) / 4.0;

            // update quantized_array with new coefficients while keeping pb and pr values the same
            // quantized array is a quarter the size of the crt array, so divide c and r by 2
            *new_quantized_array.get_mut(c / 2, r / 2).unwrap() = Quantized {
                a: a_coeff,
                b: b_coeff,
                c: c_coeff,
                d: d_coeff,
                pb_avg: quantized_array.get(c / 2, r / 2).unwrap().pb_avg,
                pr_avg: quantized_array.get(c / 2, r / 2).unwrap().pb_avg,
            };
        }
    }
    new_quantized_array
}


// average Pb and Pr values 
// input: Array2<Crt>, Array2<Quantized>
// output: Array2<Quantized> (updated)
pub fn average_pb_pr(crt_array: Array2<Crt>, quantized_array: Array2<Quantized>) -> Array2<Quantized> {
    
    // define new array2 of quantized structs
    let mut new_quantized_array = quantized_array.clone();

    // iterate through crt_array in 2x2 blocks
    for (c, r, _) in crt_array.iter_row_major() {

        // if we're at both an even column and row, we are at the start of a block
        if c % 2 == 0 && r % 2 == 0 {

            // get pb vaues in current block
            let pb1 = crt_array.get(c, r).unwrap().pb;
            let pb2 = crt_array.get(c + 1, r).unwrap().pb;
            let pb3 = crt_array.get(c, r + 1).unwrap().pb;
            let pb4 = crt_array.get(c + 1, r + 1).unwrap().pb;

            // get pr values in current block
            let pr1 = crt_array.get(c, r).unwrap().pr;
            let pr2 = crt_array.get(c + 1, r).unwrap().pr;
            let pr3 = crt_array.get(c, r + 1).unwrap().pr;
            let pr4 = crt_array.get(c + 1, r + 1).unwrap().pr;

            // calculate average pb and pr values for current block
            let new_pb_avg = (pb1 + pb2 + pb3 + pb4) / 4.0;
            let new_pr_avg = (pr1 + pr2 + pr3 + pr4) / 4.0;

            // update quantized_array with new pb and pr values while keeping a, b, c, and d values the same
            // quantized array is a quarter the size of the crt array, so divide c and r by 2
            *new_quantized_array.get_mut(c / 2, r / 2).unwrap() = Quantized {
                a: quantized_array.get(c / 2, r / 2).unwrap().a,
                b: quantized_array.get(c / 2, r / 2).unwrap().b,
                c: quantized_array.get(c / 2, r / 2).unwrap().c,
                d: quantized_array.get(c / 2, r / 2).unwrap().d,
                pb_avg: new_pb_avg,
                pr_avg: new_pr_avg,
            };
        }
    }
    new_quantized_array
}


// create an array2 of empty encoded quanta structs
pub fn new_encoded_quanta_array(width: usize, height: usize) -> Array2<EncodedQuanta> {

    // define empty encoded quanta struct
    let empty_encoded_quanta: EncodedQuanta = EncodedQuanta {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        pb_avg: 0,
        pr_avg: 0
    };

    // define new array2 of empty encoded quanta structs, and return it
    let encoded_quanta_array = Array2::new(width, height, empty_encoded_quanta);
    encoded_quanta_array
}


// convert average Pb and Pr values to 4-bit values using index_of_chroma
// input: Array2<Quantized>
// output: Array2<Quantized> (updated)
pub fn index_of_chroma_array(quantized_array: Array2<Quantized>, encoded_quanta_array: Array2<EncodedQuanta>) -> Array2<EncodedQuanta> {
    
    // define new array2 of quantized structs
    let mut new_encoded_quanta_array = encoded_quanta_array.clone();

    // iterate through quantized_array normally
    for (c, r, encoded_quanta) in encoded_quanta_array.iter_row_major() {

        // convert pb and pr averages to 4-bit values using index_of_chroma
        // update quantized_array with new pb and pr values while keeping a, b, c, and d values the same
        *new_encoded_quanta_array.get_mut(c, r).unwrap() = EncodedQuanta {
            a: encoded_quanta.a,
            b: encoded_quanta.b,
            c: encoded_quanta.c,
            d: encoded_quanta.d,
            pb_avg: index_of_chroma(quantized_array.get(c, r).unwrap().pb_avg) as u64,
            pr_avg: index_of_chroma(quantized_array.get(c, r).unwrap().pr_avg) as u64,
        };
    }
    new_encoded_quanta_array
}


// encoding algorithm for quantization
fn encode(value: f32) -> i64 {
    if value < -0.3 {
        -15
    } else if value > 0.3 {
        15
    } else {
        // Scale the float value to the integer range
        ((value / 0.3) * 15.0).round() as i64 //& 0b11111
    }
}


pub fn encode_coefficients_array(quantized_array: Array2<Quantized>, encoded_quanta_array: Array2<EncodedQuanta>) -> Array2<EncodedQuanta> {

    let mut new_encoded_quanta_array = encoded_quanta_array.clone();

    for (c, r, quanta) in quantized_array.iter_row_major() {

        let new_a = (quanta.a * 511.0).round() as u64;
        let new_b = encode(quanta.b);
        let new_c = encode(quanta.c);
        let new_d = encode(quanta.d);
        let new_pb: u64 = encoded_quanta_array.get(c, r).unwrap().pb_avg;
        let new_pr: u64 = encoded_quanta_array.get(c, r).unwrap().pr_avg;

        *new_encoded_quanta_array.get_mut(c, r).unwrap() = EncodedQuanta {
            a: new_a,
            b: new_b,
            c: new_c,
            d: new_d,
            pb_avg: new_pb,
            pr_avg: new_pr,
        };
    }
    new_encoded_quanta_array
}


// pack quantized data into codewords
// input: Array2<Quantized>
// output: Array2<u64>
pub fn pack_encoded_quanta(encoded_quanta_array: Array2<EncodedQuanta>) -> Array2<u32> {
    
    // define new array2 of u64s
    let mut codewords_array = Array2::new(encoded_quanta_array.width(), encoded_quanta_array.height(), 0);

    // iterate through quantized_array normally
    for (col, row, encoded_quanta) in encoded_quanta_array.iter_row_major() {

        // pack all quantized values into a single u64
        let mut codeword: u64 = 0;
        codeword = bitpack::newu(codeword, 9, 23, encoded_quanta.a).unwrap();
        codeword = bitpack::news(codeword, 5, 18, encoded_quanta.b).unwrap();
        codeword = bitpack::news(codeword, 5, 13, encoded_quanta.c).unwrap();
        codeword = bitpack::news(codeword, 5, 8, encoded_quanta.d).unwrap();
        codeword = bitpack::newu(codeword, 4, 4, encoded_quanta.pb_avg).unwrap();
        codeword = bitpack::newu(codeword, 4, 0, encoded_quanta.pr_avg).unwrap();
        
        // update codewords_array with new codeword
        *codewords_array.get_mut(col, row).unwrap() = codeword as u32;
    }
    codewords_array
}