mod parse_pdbqt;
use std::{env::args, process::exit};

use parse_pdbqt::{PdbqtModel, PDBQT};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!("Usage: conv_dlg [rec].dlg [lig].dlg");
        exit(0);
    }
    let rec = PDBQT::from(&args[1]);
    let mut lig = PDBQT::from_dlg(&args[2]);
    let mut com: Vec<PdbqtModel> = vec![];
    for lig in &mut lig.models {
        let mut r_atoms = rec.models[0].atoms.clone();
        r_atoms.append(&mut lig.atoms);
        com.push(PdbqtModel::new(lig.model_id, &r_atoms));
    }
    let com_pdbqt = PDBQT::new(&com);
    com_pdbqt.to_file("complexes.pdbqt");
    println!("Finished writing to complex.pdbqt");
}
