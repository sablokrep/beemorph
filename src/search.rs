use crate::interact::Interactive;
use crate::interact::InteractiveMap;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl Interactive {
    pub fn searchtaxa(&self, taxonid: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonid == taxonid {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchtaxaon(&self, taxontds: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxontds == taxontds {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchname(&self, taxonname: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonname.contains(taxonname) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchpath(&self, taxonpath: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonpathnames.contains(taxonpath) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchid(&self, taxonpathid: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonpathids.contains(taxonpathid) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchgenusid(&self, genusid: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonsubgenusid.contains(genusid) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchgenusname(&self, genusname: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxongenusname.contains(genusname) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchfamilyid(&self, familyid: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonfamilyid.contains(familyid) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn searchfamilyname(
        &self,
        familyname: &str,
    ) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.sourcetaxonfamilyname.contains(familyname) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }

    pub fn wildsearch(&self, wildname: &str) -> Result<Vec<InteractiveMap>, Box<dyn Error>> {
        let interactivemap = self.intercative().unwrap();
        let mut searchvec: Vec<InteractiveMap> = Vec::new();
        for i in interactivemap.iter() {
            if i.reststring.concat().to_string().contains(wildname) {
                searchvec.push(InteractiveMap {
                    sourcetaxonid: i.sourcetaxonid.clone(),
                    sourcetaxontds: i.sourcetaxontds.clone(),
                    sourcetaxonname: i.sourcetaxonname.clone(),
                    sourcetaxonpathnames: i.sourcetaxonpathnames.clone(),
                    sourcetaxonpathids: i.sourcetaxonpathids.clone(),
                    sourcetaxonsubgenusid: i.sourcetaxonsubgenusid.clone(),
                    sourcetaxongenusname: i.sourcetaxongenusname.clone(),
                    sourcetaxonfamilyid: i.sourcetaxonfamilyid.clone(),
                    sourcetaxonfamilyname: i.sourcetaxonfamilyname.clone(),
                    reststring: i.reststring.clone(),
                });
            }
        }
        Ok(searchvec)
    }
}
