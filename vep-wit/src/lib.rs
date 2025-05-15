#[allow(warnings)]
mod bindings;

use std::cell::RefCell;

use bindings::exports::component::vepvcf::glue::{Csq, CsqValue, Guest, GuestVcfvep, VepVcfErrors};


pub use vep_core::{VEPVCF, VEPVCFErrors};

struct VEPVCFWrapper
{
    cell:RefCell<VEPVCF>,
}

fn map_errors(err:VEPVCFErrors) -> VepVcfErrors
{
    match err {
        VEPVCFErrors::VariantFoundBeforeHeader => VepVcfErrors::Variantfoundbeforeheader ,
        VEPVCFErrors::VariantFountBeforeSamples => VepVcfErrors::Variantfountbeforesamples,
        VEPVCFErrors::UnableToGetCSQCols => VepVcfErrors::Unabletogetcsqcols,
        VEPVCFErrors::UnableToAddRecord => VepVcfErrors::Unabletoaddrecord,
        VEPVCFErrors::UnableToParseHeaderLine => VepVcfErrors::Unabletoparseheaderline,
        VEPVCFErrors::UnableToAccessHeader => VepVcfErrors::Unabletoaccessheader,
        VEPVCFErrors::OutOfRange => VepVcfErrors::Outofrange,
        VEPVCFErrors::NoError => VepVcfErrors::Noerror, 
    }
}

impl Guest for VEPVCFWrapper{
    type Vcfvep = VEPVCFWrapper;
}

/*impl GuestVcfvep for VEPVCFWrapper
{
    fn new() -> VEPVCFWrapper
    {
        //let cell = RefCell::new(VEPVCF::new());
        VEPVCFWrapper {
           // cell,
        }
    }
        
    fn read(&self, line:String) -> Result<bool, VepVcfErrors>
    {  
        Ok(false)
    }
    
    fn list_variants(&self) -> Vec<String>
    {
        vec![]
    }
    
    fn get_csq_headings(&self) -> Vec<String>
    {
        vec![]
    }
    
    fn does_record_have_csq(&self, index: u32) -> Option<bool>
    {
        None
    }
    
    fn get_csq(&self, index: u32) -> Result<Vec<Csq>, VepVcfErrors>
    {
        Err(VepVcfErrors::Noerror)
    }
    
    
}*/

impl GuestVcfvep for VEPVCFWrapper
{
    fn new() -> VEPVCFWrapper
    {
        let cell = RefCell::new(VEPVCF::new());
        VEPVCFWrapper {
            cell,
        }
    }
        
    fn read(&self, line:String) -> Result<bool, VepVcfErrors>
    {  
        let mut vcf = self.cell.borrow_mut();
        let result = vcf.read(&line);
        
        match result
        {
            Ok(()) => Ok(true),
            Err(err) => 
            {
                Err(map_errors(err))
            }
        }
    }
    
    fn list_variants(&self) -> Vec<String>
    {
        let vcf = self.cell.borrow();
        vcf.list_variants()
    }
    
    fn get_csq_headings(&self) -> Vec<String>
    {
        let vcf = self.cell.borrow();
        vcf.get_csq_headings().iter().map(|h|String::from(*h)).collect()
    }
    
    fn does_record_have_csq(&self, index: u32) -> Option<bool>
    {
        let vcf = self.cell.borrow();
        vcf.does_record_have_csq(index as usize)
    }
    
    fn get_csq(&self, index: u32) -> Result<Vec<Csq>, VepVcfErrors>
    {
        let vcf = self.cell.borrow();
        let csq_result = vcf.get_csq(index as usize);
        
        match csq_result
        {
            Ok(csq) =>
            {
                let mut result:Vec<Csq> = Vec::new();
                
                for row in csq
                {
                    let r:Csq = row.iter()
                        .map(|c| CsqValue { key:c.0.clone(), value:c.1.clone() })
                        .collect();
                    result.push(r);
                }

                Ok(result)
            },
            Err(e) =>
            {
                Err(map_errors(e))
            }
        }
    }
    
    
}

bindings::export!(VEPVCFWrapper with_types_in bindings);





