
use std::str::FromStr;
use std::collections::HashMap;

use vcf::{self, U8Vec, VCFHeader, VCFHeaderLine, VCFRecord};

#[derive(PartialEq, Debug)]
pub enum Errors
{
    UnableToOpenVCF,
    UnableToParseHeader,
    UnableToParseRecord,
    UnableToReadHeaders,
    UnableToFindCSQHeader,
    NoCSQValues,
    WIP
}

pub fn csq_desc_to_cols(desc:&str) -> Option<Vec<String>>
{
    let split_target = "Format: ";
    if desc.contains(split_target)
    {
        let desc_bit = desc.split("Format: ").last();
        if let Some(csq_str) = desc_bit
        {
            if csq_str.contains("|")
            {
                return Some(
                    csq_str
                    .split("|")
                    .map(|c| String::from(c))
                    .collect()
                );
            }
        }
    }
    
    None
}

pub fn get_csq_cols_from_header(header:&VCFHeader) -> Option<Vec<String>>
{ 
    let has_csq_header = header.info(b"CSQ");
    if let Some(csq_header) = has_csq_header
    {

        let desc = String::from_utf8_lossy(csq_header.description);
        
        return csq_desc_to_cols(&desc);
        
    }
    else
    {
        None
    }
}

pub fn is_header(line:&str) -> bool
{
    line.starts_with("##")
}

pub fn is_samples(line:&str) -> bool
{
    line.starts_with("#") && !line.starts_with("##")
}

pub fn str_to_headerline(line:&str) -> Result<VCFHeaderLine,Errors>
{
    let parse_header= VCFHeaderLine::from_str(line);
    
    if let Ok(header) = parse_header
    {
        Ok(header)
    }
    else
    {
        Err(Errors::UnableToParseHeader)
    }
}

pub fn sample_str_to_header(line:&str) -> VCFHeader
{
    VCFHeader::new(vec![],str_to_samples(line) )
}

pub fn str_to_samples(line:&str) -> Vec<U8Vec>
{
    let l = String::from(&line[1..]); // remove leading #
    let samples:Vec<U8Vec> = l.split('\t').into_iter()
    .map(|s| String::from(s).into_bytes())
    .collect();
    samples
}

pub fn str_to_record_with_header(header:&VCFHeader, line:&str) -> Result<VCFRecord, Errors>
{
    let mut rec = VCFRecord::new(header.clone());
    let parse_record = rec.parse_bytes(line.as_bytes(),1);
    if parse_record.is_ok()
    {
        return Ok(rec)
    }
    Err(Errors::UnableToParseRecord)  
}

pub fn str_to_record(samples:&str, line:&str) -> Result<VCFRecord, Errors>
{
    let header = sample_str_to_header(samples);
    
    let mut rec = VCFRecord::new(header);
    let parse_record = rec.parse_bytes(line.as_bytes(),1);
    if parse_record.is_ok()
    {
        return Ok(rec)
    }
    Err(Errors::UnableToParseRecord)
}

pub fn get_csq(rec:&VCFRecord, csq_headings:&[&str]) -> Result<Vec<HashMap<String,String>>,Errors>
{
    let has_csq = rec.info(b"CSQ");
    if let Some(csqs) = has_csq
    {
        let mut results:Vec<HashMap<String,String>> = Vec::new();
        for csq in csqs
        {
            let csq_str = String::from_utf8_lossy(csq);
            
            let csq_row = csq_str.split(',');
            
            for row in csq_row
            {
                let mut result_row:HashMap<String,String> = HashMap::new();
                let csq_columns:Vec<&str> = row.split('|').into_iter().collect();
                let max = csq_columns.len().min(csq_headings.len());
                
                for x in 0..max
                {
                    result_row.insert(
                        String::from(csq_headings[x]), String::from(csq_columns[x])
                    );
                }
                results.push(result_row);
            }
        }
        
        return Ok(results)
    }
    
    Err(Errors::NoCSQValues)
}


#[cfg(test)]
mod tests
{
    use super::*;
    
    const CSQ_DESC:&str = "Consequence annotations from Ensembl VEP. Format: Allele|Consequence|IMPACT|SYMBOL|Gene|Feature_type|Feature|BIOTYPE|EXON|INTRON|HGVSc|HGVSp|cDNA_position|CDS_position|Protein_position|Amino_acids|Codons|Existing_variation|REF_ALLELE|UPLOADED_ALLELE|DISTANCE|STRAND|FLAGS|SYMBOL_SOURCE|HGNC_ID|MANE|MANE_SELECT|MANE_PLUS_CLINICAL|TSL|APPRIS|SIFT|PolyPhen|AF|AFR_AF|AMR_AF|EAS_AF|EUR_AF|SAS_AF|gnomADe_AF|gnomADe_AFR_AF|gnomADe_AMR_AF|gnomADe_ASJ_AF|gnomADe_EAS_AF|gnomADe_FIN_AF|gnomADe_MID_AF|gnomADe_NFE_AF|gnomADe_REMAINING_AF|gnomADe_SAS_AF|CLIN_SIG|SOMATIC|PHENO|PUBMED|MOTIF_NAME|MOTIF_POS|HIGH_INF_POS|MOTIF_SCORE_CHANGE|TRANSCRIPTION_FACTORS|am_class|am_pathogenicity";
    
    const CSQ_RECORD:&str = "1	65568	.	A	C	.	.	CSQ=C|downstream_gene_variant|MODIFIER|OR4G11P|ENSG00000240361|Transcript|ENST00000492842.2|transcribed_unprocessed_pseudogene|||||||||||A|A/C|1681|1||HGNC|HGNC:31276||||||||||||||||||||||||||||||||||,C|missense_variant|MODERATE|OR4F5|ENSG00000186092|Transcript|ENST00000641515.2|protein_coding|2/3||||64|4|2|K/Q|Aag/Cag||A|A/C||1||HGNC|HGNC:14825|MANE_Select|NM_001005484.2|||P1|tolerated_low_confidence(0.06)|benign(0)|||||||||||||||||||||||||||,C|downstream_gene_variant|MODIFIER||ENSG00000290826|Transcript|ENST00000642116.1|lncRNA|||||||||||A|A/C|1452|1|||||||||||||||||||||||||||||||||||||,C|downstream_gene_variant|MODIFIER||ENSG00000290826|Transcript|ENST00000832531.1|lncRNA|||||||||||A|A/C|2042|1|||||||||||||||||||||||||||||||||||||";
    
    const SAMPLES:&str = "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO";
    
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
        let real_test = csq_desc_to_cols(CSQ_DESC);
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
    fn test_is_sample()
    {
        assert_eq!(is_samples("##fileformat=VCFv4.3"),false);
        assert_eq!(is_samples(SAMPLES),true);
    }
    
    #[test]
    fn test_get_csq_cols_from_header()
    {
        let test_csq = r#"##INFO=<ID=CSQ,Number=.,Type=String,Description="Test Format: One|Two|Three">"#;
        let parse_headerline = str_to_headerline(&test_csq);
        
        assert!(parse_headerline.is_ok());
        
        if let Ok(headerline) = parse_headerline
        {
            let test_header =VCFHeader::new(vec![headerline], vec![]);
            
            let try_csq = get_csq_cols_from_header(&test_header);
            
            assert!(try_csq.is_some());
            
            if let Some(csq) = try_csq
            {
                assert_eq!(csq.len(),3);
                assert_eq!(csq[0],"One");
                assert_eq!(csq[1],"Two");
                assert_eq!(csq[2],"Three");
                
            }
        }
        
    }

    #[test]
    fn test_str_to_header()
    {
        let test_header = format!(r#"##INFO=<ID=CSQ,Number=.,Type=String,Description="{}">"#, CSQ_DESC);
        let parse_header = str_to_headerline(&test_header);
        
        assert!(parse_header.is_ok());
        
        if let Ok(header_line) = parse_header
        {
            let test_header =VCFHeader::new(vec![header_line], vec![]);
            
            let vep = test_header.info(b"CSQ");
            
            assert!(vep.is_some());
            
            if let Some(vep_meta) = vep
            {
                let desc = String::from_utf8_lossy(vep_meta.description);
                assert_eq!(&desc, CSQ_DESC);
            }
        }
    }
    
    #[test]
    fn test_str_to_headerline()
    {
        let parse_header = str_to_headerline("##fileformat=VCFv4.1");
        
        assert!(parse_header.is_ok());
    }
    
    #[test]
    fn test_str_to_samples()
    {
        let header = sample_str_to_header(SAMPLES);
        assert_eq!(header.samples().len(), 8);
        
        let samples = header.samples();
        
        assert_eq!(b"CHROM",samples[0].as_slice());
        assert_eq!(b"INFO",samples[7].as_slice());
    }
    
    #[test]
    fn test_str_to_record()
    {
        let rec:&str = "sq0\t1\ttest\tA\t.\t.\tPASS\t.";

        let parse_record = str_to_record(SAMPLES, rec);
        
        assert!(parse_record.is_ok());
        
        if let Ok(record) = parse_record
        {
            assert_eq!(
                record.position,
                1
            );
            
            assert_eq!(
                record.chromosome,
                b"sq0"
            );
            
            assert_eq!(
                record.reference,
                b"A"
            );
            
            assert_eq!(
                record.id[0].as_slice(),
                b"test"
            );
        }
    }
    
    #[test]
    fn test_get_csq()
    {
        let csq_headings_option = csq_desc_to_cols(CSQ_DESC);
        assert!(csq_headings_option.is_some());
        
        if let Some(csq_headings) = csq_headings_option
        {
            let csq_slice:Vec<&str> = csq_headings.iter().map(|s| s.as_str()).collect();
            let parse_record = str_to_record(SAMPLES, CSQ_RECORD);
            assert!(parse_record.is_ok());
            
            if let Ok(record) = parse_record
            {
                let csq_results = get_csq
                (
                    &record,
                    &csq_slice
                );
                
                assert!(csq_results.is_ok());
                
                if let Ok(csq) = csq_results
                {
                    assert_eq!(csq.len(), 4);
                    
                    let csq_row = &csq[0];
                    let second_csq_row = &csq[1];
                    
                    assert_eq!(csq_row.get("Consequence"), Some(&String::from("downstream_gene_variant")));
                    assert_eq!(second_csq_row.get("Consequence"), Some(&String::from("missense_variant")));
                    
                    assert_eq!(csq_row.get("IMPACT"), Some(&String::from("MODIFIER")));
                    assert_eq!(second_csq_row.get("IMPACT"), Some(&String::from("MODERATE")));
                }
            }
        
        }
        
        
    }
}