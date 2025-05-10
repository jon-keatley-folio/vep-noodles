use std::fs;
use std::path::PathBuf;
use std::env;

pub mod vep;

fn main()
{
    //Need to check what mode we are running in
    println!("Just getting started");
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
    }
    

}



#[cfg(test)]
mod tests {
    use super::*;


}
