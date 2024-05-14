// structs for managing data formats 

// define rgb float struct
#[derive(Debug, Clone, Copy)] // not sure what this is for exactly?
#[allow(dead_code)]
pub struct RgbFloat {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

// define crt struct
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Crt {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

// enum for pb & pr averages
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Avg {
    Chroma(f32),
    Index(usize),
}

// quantized data struct
// TODO: need to check if enum type is necessary for coefficients
// TODO: check if enum type is necessary for pb and pr averages
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Quantized {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub pb_avg: f32,
    pub pr_avg: f32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EncodedQuanta {
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64, 
    pub pb_avg: u64, 
    pub pr_avg: u64,
}