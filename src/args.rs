use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "beemorph",
    version = "1.0",
    about = "Interaction portal for bee interaction
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// browse the interactions
    TaxonID {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxonid
        taxonid: String,
    },
    /// browse according to taxontds
    TaxonTds {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxontds
        taxontds: String,
    },
    /// browse according to taxonname
    TaxonName {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxonname
        taxonname: String,
    },
    /// browse according to taxonpathname
    TaxonPathName {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxonpath
        taxonpathname: String,
    },
    /// browse according to taxonpathids
    TaxonpathID {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxon path id
        taxonpathid: String,
    },
    /// browse according to taxonsubgenusid
    TaxonSubGenusID {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by the taxon sub genus id
        taxonsubgenusid: String,
    },
    /// browse according to taxongenusname
    TaxonGenusName {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by the taxon genus name
        taxongenusname: String,
    },
    /// browse according to taxonfamilyid
    TaxonFamilyID {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxon family id
        taxonfamilyid: String,
    },
    /// browse according to taxonfamilyname
    TaxonFamilyName {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxon family name
        taxonfamilyname: String,
    },
    /// browse according to wildsearch
    WildSearch {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search wild categoris
        wildsearch: String,
    },
    /// make a database
    DiversityDatabase {
        /// filepath
        pathfile: String,
        /// number of threads
        threads: String,
    },
}
