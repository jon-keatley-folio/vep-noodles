use vcf::{self, U8Vec, VCFHeader, VCFRecord};

pub mod utils;

pub struct VEPVCF
{
    header_count:u32,
    variant_count:u32,
    samples:Option<String>,
    header:Option<VCFHeader>,
    records:Vec<VCFRecord>
}

impl VEPVCF {
    pub fn new() -> VEPVCF
    {
        VEPVCF
        {
            header_count:0,
            variant_count:0,
            samples:None,
            header:None,
            records:Vec::new()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


}
