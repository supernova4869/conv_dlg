mod parse_pdbqt;
use std::{env::args, process::exit};

use parse_pdbqt::{PdbqtModel, PDBQT};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        println!("Usage 1: conv_dlg [lig].dlg");
        println!("Usage 2: conv_dlg [lig].dlg [rec].pdbqt");
        exit(0);
    } else if args.len() == 2 {
        let lig = PDBQT::from_dlg(&args[1]);
        let mut ligs: Vec<PdbqtModel> = vec![];
        for lig in &lig.models {
            ligs.push(PdbqtModel::new(lig.model_id, &lig.atoms));
        }
        let ligs_pdbqt = PDBQT::new(&ligs);
        ligs_pdbqt.write(format!("{}_out.pdbqt", &args[1][..&args[1].len() - 4]).as_str());
        println!("Finished writing to [lig]_out.pdbqt");
    } else {
        let rec = PDBQT::from(&args[2]);
        let mut lig = PDBQT::from_dlg(&args[1]);
        let mut com: Vec<PdbqtModel> = vec![];
        for lig in &mut lig.models {
            let mut r_atoms = rec.models[0].atoms.clone();
            r_atoms.append(&mut lig.atoms);
            com.push(PdbqtModel::new(lig.model_id, &r_atoms));
        }
        let com_pdbqt = PDBQT::new(&com);
        com_pdbqt.split(format!("{}_{}", &args[2][..&args[2].len() - 6], &args[1][..&args[1].len() - 4]).as_str());
        println!("Finished writing to [rec]_[lig]_conf[id].pdbqt");
    }
}
