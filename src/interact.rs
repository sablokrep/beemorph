#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Interactive {
    pub interactive: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct InteractiveMap {
    pub sourcetaxonid: String,
    pub sourcetaxontds: String,
    pub sourcetaxonname: String,
    pub sourcetaxonpathnames: String,
    pub sourcetaxonpathids: String,
    pub sourcetaxonsubgenusid: String,
    pub sourcetaxongenusname: String,
    pub sourcetaxonfamilyid: String,
    pub sourcetaxonfamilyname: String,
    pub reststring: Vec<String>,
}
