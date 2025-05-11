#[allow(warnings)]
mod bindings;

use std::borrow::BorrowMut;

use bindings::exports::component::vepvcf::glue::{GuestVcfvep, VepVcfErrors, Csq, CsqValue};


pub use vep_core::{VEPVCF, VEPVCFErrors};

struct Component
{
    vcf:VEPVCF,
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

impl GuestVcfvep for Component{
    fn new() -> Component
    {
        Component {
            vcf: VEPVCF::new(),
        }
    }
        
    fn read(&mut self, line:String) -> Result<bool, VepVcfErrors>
    {  
        let result = self.vcf.read(&line);
        
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
        vec![]
    }
    fn get_csq_headings(&self, index: u32) -> Vec<String>
    {
        vec![]
    }
    
    fn does_record_have_csq(&self, index: u32) -> Option<bool>
    {
        None
    }
    
    fn get_csq(&self, index: u32) -> Result<Csq, VepVcfErrors>
    {
        Err(VepVcfErrors::Noerror)
    }
    
    
}





