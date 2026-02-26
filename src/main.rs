mod args;
mod interact;
mod morph;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use figlet_rs::FIGfont;
mod search;
use crate::interact::Interactive;
/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("beemorph");
    println!("{}", repgenerate.unwrap());

    let args = CommandParse::parse();
    match &args.command {
        Commands::Interactive {
            filepath,
            thread,
            taxonid,
            taxontds,
            taxonname,
            taxonpathid,
            taxonsubgenusid,
            taxongenusname,
            taxonfamilyid,
            taxonfamilyname,
            wildsearch,
        } => {
            let n_threads = thread.parse::<usize>().expect("thread must be a number");
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let stringgraph = Interactive {
                    interactive: filepath.clone(),
                };
                let unpackfilepath = stringgraph.intercative().unwrap();
                if taxonid.unwrap() != None.unwrap() {
                    let value = stringgraph.searchtaxa(&taxonid.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxontds.unwrap() != None.unwrap() {
                    let value = stringgraph.searchtaxaon(&taxontds.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonname.unwrap() != None.unwrap() {
                    let value = stringgraph.searchname(&taxonname.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonpathid.unwrap() != None.unwrap() {
                    let value = stringgraph.searchid(&taxonpathid.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonsubgenusid.unwrap() != None.unwrap() {
                    let value = stringgraph.searchgenusid(&taxonsubgenusid.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxongenusname.unwrap() != None.unwrap() {
                    let value = stringgraph.searchgenusname(&taxongenusname.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonfamilyid.unwrap() != None.unwrap() {
                    let value = stringgraph.searchfamilyid(&taxonfamilyid.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonname.unwrap() != None.unwrap() {
                    let value = stringgraph.searchfamilyname(&taxonname.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if wildsearch.unwrap() != None.unwrap() {
                    let value = stringgraph.wildsearch(&wildsearch.unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id", "Source taxon tds", "Source taxon name", "Source path names", "Source path ids", "Source taxon subgenus ids", "Source taxon genus name", "Source family id", "Source family name", "Rest of the information", {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}, i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
            });
        }
    }
}
