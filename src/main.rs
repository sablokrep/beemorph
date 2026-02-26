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
                if taxonid.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchtaxa(&taxonid.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxontds.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchtaxaon(&taxontds.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonname.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchname(&taxonname.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonpathid.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchid(&taxonpathid.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonsubgenusid.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchgenusid(&taxonsubgenusid.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxongenusname.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchgenusname(&taxongenusname.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonfamilyid.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchfamilyid(&taxonfamilyid.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if taxonname.clone().unwrap().len() != 0usize {
                    let value = stringgraph.searchfamilyname(&taxonname.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
                if wildsearch.clone().unwrap().len() != 0usize {
                    let value = stringgraph.wildsearch(&wildsearch.clone().unwrap()).unwrap();
                    for i in value.iter(){
                    print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                    }
                }
            });
        }
    }
}
