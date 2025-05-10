use std::default;
use std::{path::PathBuf, str::FromStr};

use noodles::vcf::{self, io::reader, Header, Record};
use noodles::vcf::header::Parser;


//To do. 
//- Parse CSQ description from header - done
//- Get header size - on hold
//- list variants - on hold
//- get all info fields for a select variant - select by id, select by 
//- parse csq info fields

pub enum Errors
{
    UnableToOpenVCF,
    UnableToParseHeader,
    UnableToParseRecord,
    UnableToReadHeaders,
    UnableToFindCSQHeader,
    WIP
}

pub fn csq_desc_to_cols(desc:&str) -> Option<Vec<String>>
{
    let split_target = "Format: ";
    if desc.contains(split_target)
    {
        let desc_bit = desc.split("Format: ").last();
        if let Some(col_str) = desc_bit
        {
            if col_str.contains("|")
            {
                return Some(
                    col_str
                    .split("|")
                    .map(|c| String::from(c))
                    .collect()
                );
            }
        }
    }
    
    None
}

pub fn is_header(line:&str) -> bool
{
    line.starts_with("#")
}

pub fn str_to_header(line:&str) -> Result<Header,Errors>
{
    //let parser = Parser::builder()
   
    let parse_header = Header::from_str(line);
    if let Ok(header) = parse_header
    {
        Ok(header)
    }
    else
    {
        Err(Errors::UnableToParseHeader)
    }
}

pub fn str_to_record(line:&str) -> Result<Record, Errors>
{
    
    //let parse_record = vcf::Record::default().
    Err(Errors::UnableToParseRecord)
}


pub struct VepTools
{
    vcf_file:String,
    header_length:u32,
    variant_count:u32,
    csq_columns:Vec<String>,
}

impl VepTools
{
    
    
    pub fn from_file(file:&str) -> Result<VepTools,Errors>
    {
        let open_vcf_result = vcf::io::reader::Builder::default().build_from_path(file);
        
        if let Ok(mut vcf) = open_vcf_result
        {
            let mut csq_columns:Vec<String> = vec![];
            
            let headers_result = vcf.read_header();
            if let Ok(headers) = headers_result
            {
                for (id,details) in headers.infos()
                {
                    if id == "CSQ"
                    {
                        let csq_option = csq_desc_to_cols(
                            details.description()
                            );
                        if let Some(csq_c) = csq_option
                        {
                            csq_columns = csq_c;
                        }
                    }
                }
            }
            else
            {
                return Err(Errors::UnableToReadHeaders)
            }
            
            return Ok(VepTools
            {
                vcf_file: String::from(file),
                header_length: 0,
                variant_count: 0,
                csq_columns: csq_columns,
            })
        }
        
        Err(Errors::UnableToOpenVCF)
    }
    
    fn get_csq_columns()
    {
        
    }
    
    fn get_counts(&mut self)
    {
        
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    
    #[test]
    fn test_csq_disc_to_cols()
    {
        let desc = "Consequence annotations from Ensembl VEP. Format: Allele|Consequence|IMPACT";
        
        let cols_option = csq_desc_to_cols(desc);
        assert!(cols_option.is_some());
        
        if let Some(cols) = cols_option
        {
            assert_eq!(cols.len(),3);
            assert_eq!(cols[0],"Allele");
            assert_eq!(cols[1],"Consequence");
            assert_eq!(cols[2],"IMPACT");
        }
    }
    
    #[test]
    fn test_csq_disc_to_cols_neagtive()
    {
        let missing_format_test = csq_desc_to_cols("This should fail");
        assert!(missing_format_test.is_none());
        
        let missing_pipe_test = csq_desc_to_cols("This should fail. Format: this,and,that");
        assert!(missing_pipe_test.is_none());
        
        let blank_test = csq_desc_to_cols("");
        assert!(blank_test.is_none());
    }
    
    #[test]
    fn test_csq_disc_real_world()
    {
        let desc="Consequence annotations from Ensembl VEP. Format: Allele|Consequence|IMPACT|SYMBOL|Gene|Feature_type|Feature|BIOTYPE|EXON|INTRON|HGVSc|HGVSp|cDNA_position|CDS_position|Protein_position|Amino_acids|Codons|Existing_variation|REF_ALLELE|UPLOADED_ALLELE|DISTANCE|STRAND|FLAGS|SYMBOL_SOURCE|HGNC_ID|MANE|MANE_SELECT|MANE_PLUS_CLINICAL|TSL|APPRIS|SIFT|PolyPhen|AF|AFR_AF|AMR_AF|EAS_AF|EUR_AF|SAS_AF|gnomADe_AF|gnomADe_AFR_AF|gnomADe_AMR_AF|gnomADe_ASJ_AF|gnomADe_EAS_AF|gnomADe_FIN_AF|gnomADe_MID_AF|gnomADe_NFE_AF|gnomADe_REMAINING_AF|gnomADe_SAS_AF|CLIN_SIG|SOMATIC|PHENO|PUBMED|MOTIF_NAME|MOTIF_POS|HIGH_INF_POS|MOTIF_SCORE_CHANGE|TRANSCRIPTION_FACTORS|am_class|am_pathogenicity";
        
        let real_test = csq_desc_to_cols(desc);
        assert!(real_test.is_some());
        
        if let Some(cols) = real_test
        {
            assert_eq!(cols.len(),59);
            assert_eq!("Allele",cols[0]);
            assert_eq!("am_pathogenicity",cols[cols.len() - 1]);
            
        }
    }
    
    #[test]
    fn test_is_header()
    {
        assert_eq!(is_header("##fileformat=VCFv4.3"),true);
        assert_eq!(is_header("sq0\t1\t.\tA\t.\t.\tPASS\t."),false);
    }

    #[test]
    fn test_str_to_header()
    {
        let parse_header = str_to_header("##fileformat=VCFv4.3");
        
        assert!(parse_header.is_ok());
        
        
    }
}