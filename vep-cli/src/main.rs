use std::fs;
use std::env;
use std::io::BufRead;
use std::io::BufReader;

use vep_core::VEPVCF;

//goals 
//- load variant
//- list variants
//- print csq for a variant
//- ARGS: MODE PATH

#[derive(Debug)]
enum Mode 
{
    List,
    CSQ(usize),
    Error,
    Unknown
}

fn main() 
{
    let unexpected_usage:&str = "Unexpected usage! Requires Path[vcf path] Mode[list|csq] Index[if csq mode]";

    //validate args
    let args:Vec<String> =  env::args().skip(1).collect();
    if args.len() < 2 || args.len() > 3
    {
        println!("{}",unexpected_usage);
        return;
    }
    
    //get mode
    let mode:Mode = match args[1].to_lowercase().as_str()
    {
        "list" => Mode::List,
        "csq" => {
            let csq_mode:Mode;
            if args.len() >= 3
            {
                let parse_index = args[2].trim().parse::<usize>();
                if let Ok(index) = parse_index
                {
                    csq_mode = Mode::CSQ(index);
                }
                else
                {
                    csq_mode = Mode::Error;
                }
            }
            else
            {
                csq_mode = Mode::Error;
            }
            
            csq_mode
        },
        _ => Mode::Unknown
    };
    
    match mode
    {
        Mode::Error => {
            println!("Unable to parse index");
            println!("{}",unexpected_usage);
            return;
        },
        Mode::Unknown => {
            println!("{}",unexpected_usage);
            return;
        }
        _ => {}
    }
    
    //load vcf
    let path = args[0].trim();
    if !path.ends_with(".vcf")
    {
        println!("File must be a VCF {}", path);
        return; 
    }
    
    let has_path = fs::exists(&path);
    let mut vcf = VEPVCF::new();
    if let Ok(path_exists) = has_path
    {
        if !path_exists
        {
            println!("Unable to find file: {}", path);
            return; 
        }
        let open_file = fs::File::open(path);
        
        if let Ok(file) = open_file
        {
            let reader = BufReader::new(file);
            let mut counter = 0;
            
            for line in reader.lines()
            {
                counter += 1;
                if let Ok(l) = line
                {
                    if vcf.read(&l).is_err()
                    {
                        println!("Parse error at line {:?}",counter);
                        return;
                    }
                }
                else
                {
                    println!("Read line error at {:?}",counter);
                    return;
                }
            }
        }
        else
        {
            print!("Unable to read file");
            return; 
        }
    }
    else
    {
        println!("Unable to find file: {}", path);
        //return; 
    }
    
    match mode
    {
        Mode::List => 
        {
            let vars = vcf.list_variants();
            
            println!("Variants found");
            let mut line_count = 0;
            for v in vars
            {
                println!("{:?} {}",line_count, v);
                line_count += 1;
            }
        },
        Mode::CSQ(index) => 
        {
            let csq_results = vcf.get_csq(index);
            
            if let Ok(csq) = csq_results
            {
                let headers = vcf.get_csq_headings();
                let header_slice = &headers[0..8];
                
                //print headers
                for header in header_slice
                {
                    print!("{}\t",header);
                }
                print!("\n");
                
                for row in csq
                {
                    for header in header_slice
                    {
                        let key = *header;
                        let value_option = row.get(key);
                        if let Some(value) = value_option
                        {
                            print!("{}\t",value);
                        }
                        else
                        {
                            print!(".\t");
                        }
                    }
                    print!("\n");
                }
                
            }
            else
            {
                println!("Unable to get CSQ values for {}", index);
            }
        },
        _ => 
        {
            println!("Something went wrong!!");    
        }
    }
    
   /* println!("Just getting started");
    let pwd_result = env::current_dir();
    if let Ok(pwd) = pwd_result
    {
        
        println!("PWD:{}", pwd.clone().to_string_lossy());
        
        let dir_results = fs::read_dir(pwd);
        
        if let Ok(dir) = dir_results
        {
            for d in dir
            {
                if let Ok(de) = d
                {
                    println!("..{}", de.file_name().to_string_lossy());
                }
            }
        }
        else
        {
            println!("Unable to dir :( ");
        }
        
        
    }
    else
    {
        println!("Unable to get PWD :( ");
    }*/
    
    //perform action
    
}
