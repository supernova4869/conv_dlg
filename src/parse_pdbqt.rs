use std::fs;
use std::fmt::Formatter;
use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
pub struct PDBQT {
    pub models: Vec<PdbqtModel>
}

impl fmt::Display for PDBQT {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PDBQT with {} model(s)",
        self.models.len()
    )
}
}

impl PDBQT {
    pub fn from(fname: &str) -> PDBQT {
        let f = fs::read_to_string(fname).unwrap();
        let ms: Vec<&str> = f.split("MODEL").collect();
        let mut models: Vec<PdbqtModel> = vec![];
        for (model_id, m) in ms.iter().enumerate() {
            let mut t: Vec<&str> = m.split("\n").collect();
            t.retain(|&l| l.starts_with("ATOM") || l.starts_with("HETATM"));
            let t: Vec<String> = t.iter().map(|s| s.to_string()).collect();
            if t.len() > 0 {
                models.push(PdbqtModel::from(model_id, &t));
            }
        }
        PDBQT { models }
    }

    pub fn from_dlg(fname: &str) -> PDBQT {
        let f = fs::read_to_string(fname).unwrap();
        let ms: Vec<&str> = f.split("MODEL").collect();
        let mut models: Vec<PdbqtModel> = vec![];
        for (model_id, m) in ms.iter().enumerate() {
            let mut t: Vec<&str> = m.split("\n").collect();
            t.retain(|&l| l.starts_with("DOCKED: ATOM") || l.starts_with("DOCKED: HETATM"));
            let t: Vec<String> = t.iter().map(|s| s[8..].to_string()).collect();
            if t.len() > 0 {
                models.push(PdbqtModel::from(model_id, &t));
            }
        }
        PDBQT { models }
    }

    pub fn new(models: &Vec<PdbqtModel>) -> PDBQT {
        PDBQT {
            models: models.clone()
        }
    }

    pub fn to_file(&self, fname: &str) {
        let mut file = File::create(fname).unwrap();
        for m in &self.models {
            writeln!(file, "MODEL       {:2}", m.model_id).unwrap();
            for a in &m.atoms {
                writeln!(file, "{}", a).unwrap();
            }
        }
    }
}

#[derive(Clone)]
pub struct PdbqtModel {
    pub model_id: usize,
    pub atoms: Vec<PdbqtAtom>
}

impl PdbqtModel {
    pub fn from(model_id: usize, atom_strs: &Vec<String>) -> PdbqtModel {
        let model_id = model_id;
        let mut atoms: Vec<PdbqtAtom> = vec![];
        for a in atom_strs {
            atoms.push(PdbqtAtom::from(a));
        }
        PdbqtModel {
            model_id,
            atoms
        }
    }

    pub fn new(model_id: usize, atoms: &Vec<PdbqtAtom>) -> PdbqtModel{
        PdbqtModel {
            model_id,
            atoms: atoms.clone()
        }
    }
}

#[derive(Clone)]
pub struct PdbqtAtom {
    typ: String,
    atid: i32,
    pub atname: String,
    pub resname: String,
    chainname: String,
    pub resid: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    occupy: f64,
    bf: f64,
    pub charge: f64,
    pub attype: String
}

impl PdbqtAtom {
    pub fn from(line: &str) -> PdbqtAtom {
        // 01234567890123456789012345678901234567890123456789012345678901234567890123456789
        // ATOM      1  N   ALA A   2      26.338 -25.338  11.581  1.00 42.62     0.614 N 
        let typ = line[0..6].trim();
        let atid: i32 = line[9..11].trim().parse().unwrap();
        let atname = line[12..16].trim();
        let resname = line[17..20].trim();
        let chainname = line[21..22].trim();
        let resid: i32 = line[22..26].trim().parse().unwrap();
        let x: f64 = line[30..38].trim().parse().unwrap();
        let y: f64 = line[38..46].trim().parse().unwrap();
        let z: f64 = line[46..54].trim().parse().unwrap();
        let occupy: f64 = line[55..60].trim().parse().unwrap();
        let bf: f64 = line[61..66].trim().parse().unwrap();
        let charge: f64 = line[70..76].trim().parse().unwrap();
        let attype = line[77..79].trim();
        return PdbqtAtom {
            typ: typ.to_string(),
            atid,
            atname: atname.to_string(),
            resname: resname.to_string(),
            chainname: chainname.to_string(),
            resid,
            x,
            y,
            z,
            occupy,
            bf,
            charge,
            attype: attype.to_string()
        }
    }
}

impl fmt::Display for PdbqtAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // 01234567890123456789012345678901234567890123456789012345678901234567890123456789
        // ATOM      1  N   ALA A   2      26.338 -25.338  11.581  1.00 42.62     0.614 N 
        write!(f, "{:6} {:4} {:4} {:3} {:1}{:4}    {:8.3}{:8.3}{:8.3}  {:4.2}{:6.2}    {:6.3} {:2}",
        self.typ, self.atid, self.atname, self.resname, self.chainname, self.resid, self.x, self.y, self.z, self.occupy, self.bf, self.charge, self.attype
    )
}
}