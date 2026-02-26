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
    Interactive {
        /// path to the file
        filepath: String,
        /// threads for the analysis
        thread: String,
        /// search by taxonid
        taxonid: Option<String>,
        /// search by taxontds
        taxontds: Option<String>,
        /// search by taxonname
        taxonname: Option<String>,
        /// search by taxon path id
        taxonpathid: Option<String>,
        /// search by the taxon sub genus id
        taxonsubgenusid: Option<String>,
        /// search by the taxon genus name
        taxongenusname: Option<String>,
        /// search by taxon family id
        taxonfamilyid: Option<String>,
        /// search by taxon family name
        taxonfamilyname: Option<String>,
        /// search wild categoris
        wildsearch: Option<String>,
    },
}
