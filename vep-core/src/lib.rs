use std::collections::HashMap;

use vcf::{self, U8Vec, VCFHeader, VCFHeaderLine, VCFRecord};

pub mod utils;



pub struct VEPVCF
{
    samples:Option<String>,
    header:Option<VCFHeader>,
    header_lines:Vec<VCFHeaderLine>,
    records:Vec<VCFRecord>,
    csq_headings:Vec<String>,
}

pub enum VEPVCFErrors
{
    VariantFoundBeforeHeader,
    VariantFountBeforeSamples,
    UnableToGetCSQCols,
    UnableToAddRecord,
    UnableToParseHeaderLine,
    UnableToAccessHeader,
    OutOfRange,
    NoError,
}

impl VEPVCF 
{
    pub fn new() -> VEPVCF
    {
        VEPVCF
        {
            samples:None,
            header:None,
            records:Vec::new(),
            header_lines:Vec::new(),
            csq_headings:Vec::new(),
        }
    }
    
    pub fn list_variants(&self) -> Vec<String>
    {
        let mut vars:Vec<String> = Vec::new();
        
        for var in &self.records
        {
            let chrom = String::from_utf8_lossy(&var.chromosome);
            let pos = var.position.to_string();
            let refe = String::from_utf8_lossy(&var.reference);
            let alt:Vec<String> = var.alternative.iter().map(|v|String::from(String::from_utf8_lossy(v))).collect();
            let mut id:Vec<String> = var.id.iter().map(|v|String::from(String::from_utf8_lossy(v))).collect();
            
            if id.len() == 0
            {
                id.push(String::from("."));
            }
            
            vars.push(
                format!("{} {} {} {} {}",chrom, pos,id.join(","), refe, alt.join(",") )
            );
        }
        
        vars
    }
    
    pub fn does_record_have_csq(&self, index:usize) -> Option<bool>
    {
        if index >= self.records.len()
        {
            return None;
        }
        
        let rec = &self.records[index];
        
        Some(rec.info(b"CSQ").is_some())  
    }
    
    pub fn get_csq(&self, index:usize) -> Result<Vec<HashMap<String,String>>,VEPVCFErrors>
    {
        if index >= self.records.len()
        {
            return Err(VEPVCFErrors::OutOfRange);
        }
        
        let rec = &self.records[index];
        
        let headings:Vec<&str> = self.csq_headings.iter().map(|h| h.as_str() ).collect();
        
        let csq_result =utils::get_csq(rec, &headings);
        
        if let Ok(csq) = csq_result
        {
            Ok(csq)
        }
        else
        {
            Err(VEPVCFErrors::UnableToParseHeaderLine)
        }
    }
    
    pub fn read(&mut self, line:&str) -> Result<(),VEPVCFErrors>
    {
        let error;
        if utils::is_header(line)
        {
            let parse_line = utils::str_to_headerline(line);
            
            if let Ok(header_line) = parse_line
            {
                self.header_lines.push(header_line);
                return Ok(());
            }
            else
            {
                error = VEPVCFErrors::UnableToParseHeaderLine
            }

        }
        else if utils::is_samples(line)
        {
            self.samples = Some(String::from(line));
            let header_samples = utils::str_to_samples(line);
            self.header = Some(VCFHeader::new(self.header_lines.clone(), header_samples));
            
            if let Some(header) = &self.header
            {
                //error = VEPVCFErrors::UnableToGetCSQCols;
                let has_csq_cols = utils::get_csq_cols_from_header(header);
                
                if let Some(csq_cols) = has_csq_cols
                {
                    self.csq_headings = csq_cols;
                    return Ok(());
                }
            }
            error = VEPVCFErrors::UnableToGetCSQCols;
        }
        else
        {
            if let Some(header) = &self.header
            {
                let rec_result = utils::str_to_record_with_header(header, line);
                
                match rec_result
                {
                    Ok(rec) =>
                    {
                        self.records.push(rec);
                        return Ok(());
                    },
                    Err(_) => error= VEPVCFErrors::UnableToAddRecord
                }
            }
            else
            {
                if self.samples.is_none()
                {
                    error = VEPVCFErrors::VariantFountBeforeSamples;
                }
                else
                {
                    error = VEPVCFErrors::VariantFoundBeforeHeader;
                }
            }
        }
             
        Err(error)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vepvcf()
    {
        let vcf_file:Vec<&str> = vec![
            "##fileformat=VCFv4.3",
            r#"##INFO=<ID=CSQ,Number=.,Type=String,Description="Consequence annotations from Ensembl VEP. Format: Allele|Consequence|IMPACT">"#,
            "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO",
            "1	65568	test1	A	C	.	.	CSQ=C|downstream_gene_variant|MODIFIER|",
            "2	265023	.	C	T	.	.	CSQ=C|missense_variant|MODIFIER|",
            "3	319780	.	GA	G	.	.	.",
        ];
        
        let mut test_vcf = VEPVCF::new();
        
        //test loading vcf
        for vcf_line in vcf_file
        {
            let add_result = test_vcf.read(vcf_line);

            assert!(add_result.is_ok(),"Error on {}",vcf_line);
        }
        
        let vars = test_vcf.list_variants();
        assert_eq!(vars.len(),3);
        
        assert_eq!(vars[0],"1 65568 test1 A C");
        assert_eq!(vars[1],"2 265023 . C T");
        
        assert_eq!(test_vcf.does_record_have_csq(0), Some(true));
        assert_eq!(test_vcf.does_record_have_csq(1), Some(true));
        assert_eq!(test_vcf.does_record_have_csq(2), Some(false));
        assert_eq!(test_vcf.does_record_have_csq(3), None);
        
        let csq_result = test_vcf.get_csq(0);
        
        assert!(csq_result.is_ok());
        if let Ok(csq) = csq_result
        {
            assert_eq!(csq.len(),1);
            
            let val = csq[0].get("IMPACT");
            assert!(val.is_some());
            assert_eq!(val, Some(&String::from("MODIFIER")));
        }
        
    }
}
