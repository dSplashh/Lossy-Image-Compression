**Darryl A. Fleurantin**

**Kai Maffucci**

This implementation utilizes Professor Daniels at the University of Rhode Island array2 and bitpack implementations.

Usage: 

Compression:

       rpeg -c [filename]

Decompression:

       rpeg -d [filename]
       

Architecture:

- Data Structures:
    - RgbFloat: Represents RGB colors in floating-point format.
    - Crt: Represents color space conversion in floating-point format.
    - Avg: Enum for different types of avergaes used in quantization.
    - Quantized: Struct for quantized data with coefficients and average pb/pr.
    - EncodedQuanta: Struct for encoded quantized data used in compression and decoding.


- Modules:
    - structures.rs: Defines the data structures (RgbFloat, Crt, Avg, Quantized, EncodedQuanta) for managing data formats.

    - compression.rs: Handles RGB to CRT conversion, to DCT, averaging, quantization,  encoding, and packing data into codewords.

    - decompression.rs: Extracts encoded data from codewords, reverses indexing, decoding coefficients and averages, setting pb/pr values, IDCT, and conversion from CRT to RGB.

- Compressing ppm image:
    1. Read ppm image with class crate
    2. Create array2 array of pixels from image
    3. Trim array if uneven dimensions with ``trim_array`` function and update dimensions
    4. Utilize ``rgb_int_array_to_rgb_float_array`` function to convert pixels to floats
    5. Utilize ``rgb_float_array_to_crt_array`` function to convert pixels floats to component video color space values
    6. Utilize ``new_quantized_array`` function to create an array2 of empty quantized structs
    7. Utilize ``dct`` function to perform DCT on CRT array (Y/Pb/Pr) and update the empty array2 quantized structs with the new coefficients
    8. Utilize ``average_pb_pr`` function to get average pb and pr values from CRT array and update the array2 quantized structs with the new pb and pr values
    9. Utilize ``new_encoded_quanta_array`` function to create an array2 of empty encoded quanta structs
    10. Utilize ``index_of_chroma_array`` function to convert pb and pr averages to 4-bit values from quantized structs and update empty encoded quanta structs with the new pb and pr values
    11. Utilize ``encode_coefficients_array`` function to perform quantization on quantized structs coefficients and update quanta structs with the new coefficients
    12. Utilize ``pack_encoded_quanta`` function to pack data from quanta structs into codewords
    13. Convert codewords into bytes and output with class crate

- Decompressing compressed image:
    1. Read rpeg data with class crate
    2. Convert rpeg data to codewords
    3. Convert codewords to an Array2 struct
    4. Utilize ``extract_encoded_data`` function to unpack codewords to encoded quanta array
    5. Utilize ``new_quantized_array`` function to create an array2 of empty quantized structs
    6. Utilize ``chroma_of_index_array``function to reverse index of chroma from quanta array and update the empty quantized structs with the new pb and pr values
    7. Utilize ``decode_coefficients_array`` function to dequantized quantized coefficients from quanta array and update quantized structs with the new coefficients
    8. Utilize ``new_crt_array`` function to create empty CRT array
    9. Utilize ``set_pb_pr`` function to set pb and pr values of CRT array respective to averages from quantized structs
    10. Utilize ``inverse_dct`` function to perform inverse DCT from quantized structs and update CRT array with the new y values
    11. Utilize ``crt_array_to_rgb_float_array`` function to convert CRT array to rgb float array
    12. Utilize ``rgb_float_array_to_rgb_int_array`` function to convert rgb float array to rgb int array
    13. Convert rgb int array into pixels
    14. Create rgb image from pixels
    15. Write image to standard out
