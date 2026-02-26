use crate::interact::Interactive;
use crate::interact::InteractiveMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/*
Gaurav Sablok
codeprog@icloud.com
*/

/*
Mapping coordinates
pub sourcetaxonid: 0,
pub sourcetaxontds: 1
pub sourcetaxonname: 2,
pub sourcetaxonpathnames: 3,
pub sourcetaxonpathids: 4,
pub sourcetaxonsubgenusid: 11,
pub sourcetaxongenusname: 12,
pub sourcetaxonfamilyid: 14,
pub sourcetaxonfamilyname: 13,
pub reststring: Vec<String>,

*/

impl Interactive {
    pub fn intercative(&self) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let fileopen = Path::new("../interaction/filepath.tsv")
            .to_str()
            .unwrap()
            .to_string();
        let filereadopen = File::open(fileopen).expect("file not present");
        let filread = BufReader::new(filereadopen);
        let mut pathdatabase: Vec<InteractiveMap> = Vec::new();
        for i in filread.lines() {
            let line = i.expect("file not present");
            let linesplit = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            pathdatabase.push(InteractiveMap {
                sourcetaxonid: linesplit[0].clone(),
                sourcetaxontds: linesplit[1].clone(),
                sourcetaxonname: linesplit[2].clone(),
                sourcetaxonpathnames: linesplit[3].clone(),
                sourcetaxonpathids: linesplit[4].clone(),
                sourcetaxonsubgenusid: linesplit[11].clone(),
                sourcetaxongenusname: linesplit[11].clone(),
                sourcetaxonfamilyid: linesplit[14].clone(),
                sourcetaxonfamilyname: linesplit[13].clone(),
                reststring: linesplit.clone(),
            })
        }

        Ok(pathdatabase)
    }
}
