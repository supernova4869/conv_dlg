mod parse_pdbqt;
use std::path::Path;

use parse_pdbqt::{PdbqtModel, PDBQT};
use clap::Parser;

/// Convert docking results (dlg) of AutoDock-GPU
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the docking results (dlg)
    #[arg(short, long)]
    dlg: String,

    /// Name of the receptor file (pdbqt)
    #[arg(short, long, default_value_t = String::new())]
    rec: String,

    /// Output total or split
    #[arg(short, long, default_value_t = true)]
    total: bool,

    /// Output type: pdbqt or pdb
    #[arg(short, long, default_value_t = String::from("pdbqt"))]
    out: String,
}

fn main() {
    let args = Args::parse();

    if args.rec.is_empty() {
        let lig = PDBQT::from_dlg(&args.dlg);
        let mut ligs: Vec<PdbqtModel> = vec![];
        for lig in &lig.models {
            ligs.push(PdbqtModel::new(lig.model_id, &lig.atoms));
        }
        let ligs_pdbqt = PDBQT::new(&ligs);
        ligs_pdbqt.write(format!("{}_out.pdbqt", &args.dlg[..&args.dlg.len() - 4]).as_str(), &args.out);
        println!("Finished writing to [lig]_out.pdbqt");
    } else {
        let rec = PDBQT::from(&args.rec);
        let mut lig = PDBQT::from_dlg(&args.dlg);
        let mut com: Vec<PdbqtModel> = vec![];
        for lig in &mut lig.models {
            let mut r_atoms = rec.models[0].atoms.clone();
            r_atoms.append(&mut lig.atoms);
            com.push(PdbqtModel::new(lig.model_id, &r_atoms));
        }
        let com_pdbqt = PDBQT::new(&com);
        let dlg_stem = Path::new(&args.dlg).file_stem().unwrap().to_str().unwrap();
        let rec_stem = Path::new(&args.rec).file_stem().unwrap().to_str().unwrap();
        if args.total {
            let fname = Path::new(&args.rec).parent().unwrap().join(format!("{}_{}.{}", rec_stem, dlg_stem, &args.out));
            let fname = fname.to_str().unwrap();
            com_pdbqt.write(fname, &args.out);
            println!("Finished writing to {}", fname);
        } else {
            let fname = Path::new(&args.rec).parent().unwrap().join(format!("{}_{}", rec_stem, dlg_stem));
            let fname = fname.to_str().unwrap();
            com_pdbqt.split(fname, &args.out);
            println!("Finished writing to {}", fname);
        }
    }
}
