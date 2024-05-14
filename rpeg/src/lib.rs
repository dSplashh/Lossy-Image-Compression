pub mod codec;
pub mod structures;
pub mod compression;
pub mod decompression;

// tests
#[cfg(test)]
mod tests {
    //use super::*;
    use array2::Array2;
    use csc411_image::{RgbImage, Rgb, Read, Write};
    //use csc411_rpegio::output_rpeg_data;
    use crate::compression::*;
    use crate::decompression::*;
    use crate::codec::*;

    #[test]
    fn up_to_dct() {

        // COMPRESSION STEP

        let filename = Some("black.ppm");

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
        let crt_array = rgb_float_array_to_crt_array(rgb_int_array);
        // create empty unencoded quantized array
        let quantized_array = new_quantized_array(crt_array.width() / 2, crt_array.height() / 2);
        // perform dct on crts
        let quantized_array = dct(crt_array.clone(), quantized_array);

        // print first few structs in crt_array
        eprintln!("{:?}", crt_array.get(0, 0).unwrap());
        eprintln!("{:?}", crt_array.get(1, 0).unwrap());
        eprintln!("{:?}", crt_array.get(0, 1).unwrap());
        eprintln!("{:?}", crt_array.get(1, 1).unwrap());

        // DECOMPRESSION STEP

        // perform inverse dct
        let crt_array = inverse_dct(quantized_array.clone(), crt_array);

        // print first few structs in crt_array
        eprintln!("{:?}", crt_array.get(0, 0).unwrap());
        eprintln!("{:?}", crt_array.get(1, 0).unwrap());
        eprintln!("{:?}", crt_array.get(0, 1).unwrap());
        eprintln!("{:?}", crt_array.get(1, 1).unwrap());
        
        // convert crt array to rgb float array
        let rgb_float_array = crt_array_to_rgb_float_array(crt_array);
        // convert rgb float array to rgb int array
        let rgb_int_array = rgb_float_array_to_rgb_int_array(rgb_float_array);

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
            width: final_width as u32,
            height: final_height as u32,
            pixels: pixels,
            denominator: 255
        };
        // write image
        img.write(Some("test.ppm")).unwrap();

    }

    // round-trip testing for first lossy operation
    #[test]
    fn up_to_averages() {

        // COMPRESSION STEP

        let filename = Some("black.ppm");

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
        let crt_array = rgb_float_array_to_crt_array(rgb_int_array);
        
        // print first few structs in crt_array
        for c in 0..8 {
            for r in 0..8 {
                if c % 2 == 0 && r % 2 == 0 {
                    eprintln!("Compressing... c: {} r: {}", c, r);
                    eprintln!("{:?}", crt_array.get(c, r).unwrap());
                    eprintln!("{:?}", crt_array.get(c + 1, r).unwrap());
                    eprintln!("{:?}", crt_array.get(c, r + 1).unwrap());
                    eprintln!("{:?}", crt_array.get(c + 1, r + 1).unwrap());
                }
            }
        }
        
        // create empty unencoded quantized array
        let quantized_array = new_quantized_array(crt_array.width() / 2, crt_array.height() / 2);
        // perform dct on crts
        let quantized_array = dct(crt_array.clone(), quantized_array);
        // average pb and pr values
        let quantized_array = average_pb_pr(crt_array.clone(), quantized_array);

        // DECOMPRESSION STEP
        let decoded_quanta_array = quantized_array.clone();

        // create empty crt array
        let crt_array = new_crt_array(final_width, final_height);
        // set pb and pr values of crt array to corresponding averages
        let crt_array = set_pb_pr(decoded_quanta_array.clone(), crt_array);

        // print first few structs in crt_array
        for c in 0..8 {
            for r in 0..8 {
                if c % 2 == 0 && r % 2 == 0 {
                    eprintln!("Decompressing... c: {} r: {}", c, r);
                    eprintln!("{:?}", crt_array.get(c, r).unwrap());
                    eprintln!("{:?}", crt_array.get(c + 1, r).unwrap());
                    eprintln!("{:?}", crt_array.get(c, r + 1).unwrap());
                    eprintln!("{:?}", crt_array.get(c + 1, r + 1).unwrap());
                }
            }
        }

        // perform inverse dct
        let crt_array = inverse_dct(decoded_quanta_array.clone(), crt_array);

        // convert crt array to rgb float array
        let rgb_float_array = crt_array_to_rgb_float_array(crt_array);
        // convert rgb float array to rgb int array
        let rgb_int_array = rgb_float_array_to_rgb_int_array(rgb_float_array);

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
            width: final_width as u32,
            height: final_height as u32,
            pixels: pixels,
            denominator: 255
        };
        // write image
        img.write(Some("test.ppm")).unwrap();

    }
}        

// test to use each compression function and print information to standard error
/*#[cfg(test)]
mod tests {
    //use super::*;
    use array2::Array2;
    use csc411_image::{RgbImage, Read};
    use csc411_rpegio::output_rpeg_data;
    use crate::compression::*;
    use crate::codec::*;

    // compression debug
    // run with --nocapture
    #[test]
    fn compress_debug() {

        let filename = Some("black.ppm");

        // read image from file
        let img = RgbImage::read((filename).as_deref()).unwrap();

        // create array of pixels from image
        let pixels = Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
        // print width and height of pixels to standard error
        eprintln!("Pixels - Width: {}, Height: {}", pixels.width(), pixels.height());

        // trim array to ensure even dimensions
        let pixels = trim_array(pixels);
        // save width and height for later
        let final_width = pixels.width() as usize;
        let final_height = pixels.height() as usize;
        // print width and height of pixels to standard error
        eprintln!("Pixels after Trim - Width: {}, Height: {}", pixels.width(), pixels.height());

        // convert array of rgbs (int) to rgb floats
        let rgb_int_array = rgb_int_array_to_rgb_float_array(pixels.clone());
        // print width and height of rgb_int_array to standard error
        eprintln!("RGB Int Array - Width: {}, Height: {}", rgb_int_array.width(), rgb_int_array.height());

        // convert array of rgb floats to crts
        let crt_array = rgb_float_array_to_crt_array(rgb_int_array);
        // print width and height of crt_array to standard error
        eprintln!("CRT Array - Width: {}, Height: {}", crt_array.width(), crt_array.height());

        // create empty quantized array
        let quantized_array = new_quantized_array(crt_array.width() / 2, crt_array.height() / 2);
        // print width and height of quantized_array to standard error
        eprintln!("Empty Quantized Array - Width: {}, Height: {}", quantized_array.width(), quantized_array.height());
        
        // perform dct on crts
        let quantized_array = dct(crt_array.clone(), quantized_array);
        // print width and height of quantized_array to standard error
        eprintln!("Quantized Array after DCT - Width: {}, Height: {}", quantized_array.width(), quantized_array.height());

        // average pb and pr values
        let quantized_array = average_pb_pr(crt_array.clone(), quantized_array);
        // print width and height of quantized_array to standard error
        eprintln!("Quantized Array after Averaging - Width: {}, Height: {}", quantized_array.width(), quantized_array.height());

        // index_of_chroma on each pb and pr value
        let quantized_array = index_of_chroma_array(quantized_array);
        // print width and height of quantized_array to standard error
        eprintln!("Quantized Array after Indexing - Width: {}, Height: {}", quantized_array.width(), quantized_array.height());
        // print first 3 values of quantized_array to standard error
        eprintln!("First Three Values of Quantized Array: {:?}, {:?}, {:?}", quantized_array.get(0, 0).unwrap(), quantized_array.get(1, 0).unwrap(), quantized_array.get(2, 0).unwrap());

        // create codewords from quantized array
        let codewords = pack_quantized_data(quantized_array);
        // print width and height of codewords to standard error
        eprintln!("Codewords - Width: {}, Height: {}", codewords.width(), codewords.height());
        // print the first four bytes of codewords to standard error
        eprintln!("First Four Bytes of Codewords: {:08x}", codewords.get(0, 0).unwrap());

        // make vec of .to_be_bytes() from codewords
        let mut codewords_vec: Vec<[u8; 4]> = Vec::new();
        for (_, _, codeword) in codewords.iter_row_major() {
            codewords_vec.push(codeword.to_be_bytes());
        }

        output_rpeg_data(&codewords_vec, final_width, final_height).unwrap();
        // do debug version too?
    }

    // coefficients encoding test
    /*#[test]
    fn encode_bcd_test() {

        // generate 3,000 floats between -0.3 and 0.3
        let mut floats: Vec<f32> = Vec::new();
        for _ in 0..3000 {
            floats.push(rand::random::<f32>() * 0.6 - 0.3);
        }

        // run encode() function on each float, and asserting that it is within the range of -15 and 15
        
    }*/
}*/