use crate::compression::*;
use crate::decompression::*;
use array2::Array2;
use csc411_image::{RgbImage, Rgb, Read, Write};
use csc411_rpegio::{input_rpeg_data, output_rpeg_data};


// trim an array2 to ensure it has even dimensions
pub fn trim_array(pixel_array: Array2<Rgb>) -> Array2<Rgb> {

    // define new width and height
    let new_width = pixel_array.width() - pixel_array.width() % 2;
    let new_height = pixel_array.height() - pixel_array.height() % 2;

    // create new array2 with new width and height
    let mut trimmed_pixel_array = Array2::new(new_width, new_height, Rgb { red: 0, green: 0, blue: 0 });
    for (c, r, pixel) in pixel_array.iter_row_major() {
        if c < new_width && r < new_height {
            *trimmed_pixel_array.get_mut(c, r).unwrap() = Rgb {
                red: (*pixel).red,
                green: (*pixel).green,
                blue: (*pixel).blue,
            };
        }
    }
    trimmed_pixel_array
}


// complete compress function
// input: filename of ppm from stdin
// output: saved rpeg image (from codewords) to stdout (into file)
pub fn compress(filename: Option<&str>) {
    
    // read image from file
    let img = RgbImage::read((filename).as_deref()).unwrap();
    // create array of pixels from image
    let pixels = Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();

    // trim array to ensure even dimensions
    let pixels = trim_array(pixels);
    // save width and height for later
    let final_width = pixels.width() as usize;
    let final_height = pixels.height() as usize;
    // convert array of rgbs (int) to rgb floats
    let rgb_int_array = rgb_int_array_to_rgb_float_array(pixels.clone());
    // convert array of rgb floats to crts
    let crt_array = rgb_float_array_to_crt_array(rgb_int_array.clone());
    // create empty unencoded quantized array
    let quantized_array = new_quantized_array(crt_array.width() / 2, crt_array.height() / 2);
    // perform dct on crts
    let quantized_array = dct(crt_array.clone(), quantized_array.clone());
    // average pb and pr values
    let quantized_array = average_pb_pr(crt_array.clone(), quantized_array.clone());

    // create new empty encoded quantized array
    let encoded_quanta_array = new_encoded_quanta_array(quantized_array.width(), quantized_array.height());
    // index_of_chroma on each pb and pr value
    let encoded_quanta_array = index_of_chroma_array(quantized_array.clone(), encoded_quanta_array.clone());
    // encode coefficients
    let encoded_quanta_array = encode_coefficients_array(quantized_array.clone(), encoded_quanta_array.clone());
    
    // create codewords from quantized array
    let codewords = pack_encoded_quanta(encoded_quanta_array.clone());

    // make vec of .to_be_bytes() from codewords
    let mut codewords_vec: Vec<[u8; 4]> = Vec::new();
    for (_, _, codeword) in codewords.iter_row_major() {
        codewords_vec.push(codeword.to_be_bytes());
    }
    output_rpeg_data(&codewords_vec, final_width, final_height).unwrap();
}


// complete decompress function
// input: filename from stdin
// output: saved ppm image (from codewords) to stdout (into file)
pub fn decompress(filename: Option<&str>) {
    
    // read rpeg from file
    let (compressed_data, width, height) = input_rpeg_data(filename).unwrap();

    // convert rpeg data (vec of 4-length arrays of u8s) to codewords (Array2 of u32s)
    let mut codewords_vec: Vec<u32> = Vec::new();
    for i in 0..compressed_data.len() {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&compressed_data[i]);
        codewords_vec.push(u32::from_be_bytes(bytes));
    }

    // convert codewords (vec of u32s) to Array2 struct
    let codewords = Array2::from_row_major(width as usize / 2, height as usize / 2, codewords_vec).unwrap();
    
    // unpack codewords to encoded quanta array
    let encoded_quanta_array = extract_encoded_data(codewords);
    // make a new empty decoded quantized array
    let decoded_quanta_array = new_quantized_array(encoded_quanta_array.width(), encoded_quanta_array.height());
    // reverse index_of_chroma by using chroma_of_index
    let decoded_quanta_array = chroma_of_index_array(encoded_quanta_array.clone(), decoded_quanta_array.clone());
    // decode coefficients
    let decoded_quanta_array = decode_coefficients_array(encoded_quanta_array.clone(), decoded_quanta_array.clone());

    // create empty crt array
    let crt_array = new_crt_array(width, height);
    // set pb and pr values of crt array to corresponding averages
    let crt_array = set_pb_pr(decoded_quanta_array.clone(), crt_array.clone());
    // perform inverse dct
    let crt_array = inverse_dct(decoded_quanta_array.clone(), crt_array.clone());

    // convert crt array to rgb float array
    let rgb_float_array = crt_array_to_rgb_float_array(crt_array.clone());
    // convert rgb float array to rgb int array
    let rgb_int_array = rgb_float_array_to_rgb_int_array(rgb_float_array.clone());

    // convert rgb int array to vector of rgb ints
    let mut pixels: Vec<Rgb> = Vec::new();
    for (_, _, rgb_int) in rgb_int_array.iter_row_major() {
        pixels.push(Rgb{
            red: rgb_int.red,
            green: rgb_int.green,
            blue: rgb_int.blue
        });
    }

    // create image from pixels
    let img = RgbImage {
        width: width as u32,
        height: height as u32,
        pixels: pixels,
        denominator: 255
    };
    // write image
    img.write(None).unwrap();

}