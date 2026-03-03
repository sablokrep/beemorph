mod args;
mod interact;
mod morph;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use figlet_rs::FIGfont;
mod search;
use crate::interact::Interactive;
mod database;

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
        Commands::TaxonID {
            filepath,
            thread,
            taxonid,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let stringgraph = Interactive {
                    interactive: filepath.clone(),
                };
                let value = stringgraph.searchtaxa(&taxonid.clone()).unwrap();
                for i in value.iter(){
                print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
                }
            })
        }
        Commands::TaxonTds {
            filepath,
            thread,
            taxontds,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchtaxaon(&taxontds.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }})
        }
        Commands::TaxonName {
            filepath,
            thread,
            taxonname,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };

      let value = stringgraph.searchname(&taxonname.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }
      })
        }
        Commands::TaxonPathName {
            filepath,
            thread,
            taxonpathname,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchpath(&taxonpathname.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }});
        }
        Commands::TaxonpathID {
            filepath,
            thread,
            taxonpathid,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchid(&taxonpathid.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }
      });
        }
        Commands::TaxonSubGenusID {
            filepath,
            thread,
            taxonsubgenusid,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchgenusid(&taxonsubgenusid.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }
      });
        }
        Commands::TaxonGenusName {
            filepath,
            thread,
            taxongenusname,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchgenusname(&taxongenusname.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }
      });
        }
        Commands::TaxonFamilyID {
            filepath,
            thread,
            taxonfamilyid,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchfamilyid(&taxonfamilyid.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }
      });
        }
        Commands::TaxonFamilyName {
            filepath,
            thread,
            taxonfamilyname,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
          let stringgraph = Interactive {
              interactive: filepath.clone(),
          };
      let value = stringgraph.searchfamilyname(&taxonfamilyname.clone()).unwrap();
      for i in value.iter(){
      print!("Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n", i.sourcetaxonid, i.sourcetaxontds, i.sourcetaxonname, i.sourcetaxonpathnames, i.sourcetaxonpathids, i.sourcetaxonsubgenusid, i.sourcetaxongenusname, i.sourcetaxonfamilyid, i.sourcetaxonfamilyname, i.reststring);
      }
      });
        }
        Commands::WildSearch {
            filepath,
            thread,
            wildsearch,
        } => {
            let n_threads = thread.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
            let stringgraph = Interactive {
                interactive: filepath.clone(),
            };
            let value = stringgraph
                .wildsearch(&wildsearch.clone())
                .unwrap();
            for i in value.iter() {
                print!(
                    "Source taxon id:{}\nSource taxon tds:{}\nSource taxon name:{}\nSource path names:{}\nSource path ids:{}\nSource taxon subgenus ids:{}\nSource taxon genus name{}\nSource family id:{}\nSource family name:{}\nRest of the information:{:?}\n",
                    i.sourcetaxonid,
                    i.sourcetaxontds,
                    i.sourcetaxonname,
                    i.sourcetaxonpathnames,
                    i.sourcetaxonpathids,
                    i.sourcetaxonsubgenusid,
                    i.sourcetaxongenusname,
                    i.sourcetaxonfamilyid,
                    i.sourcetaxonfamilyname,
                    i.reststring
                );
            }
        });
        }
        Commands::DiversityDatabase { pathfile, threads } => {
            let numthreads = threads.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(numthreads)
                .build()
                .expect("Thread building failed");
            pool.install(|| {
                let command = Interactive {
                    interactive: pathfile.clone(),
                };
                _ = command.database();
                println!("The database has been build");
            })
        }
    }
}
