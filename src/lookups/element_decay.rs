use phf::phf_map;

pub static ELEMENT_DECAY_LOOKUP: phf::Map<&'static str, &'static str> = phf_map! {
    "H" => "H",
    "He" => "Hf.Pa.H.Ca.Li",
    "Li" => "He",
    "Be" => "Ge.Ca.Li",
    "B" => "Be",
    "C" => "B",
    "N" => "C",
    "O" => "N",
    "F" => "O",
    "Ne" => "F",
    "Na" => "Ne",
    "Mg" => "Pm.Na",
    "Al" => "Mg",
    "Si" => "Al",
    "P" => "Ho.Si",
    "S" => "P",
    "Cl" => "S",
    "Ar" => "Cl",
    "K" => "Ar",
    "Ca" => "K",
    "Sc" => "Ho.Pa.H.Ca.Co",
    "Ti" => "Sc",
    "V" => "Ti",
    "Cr" => "V",
    "Mn" => "Cr.Si",
    "Fe" => "Mn",
    "Co" => "Fe",
    "Ni" => "Zn.Co",
    "Cu" => "Ni",
    "Zn" => "Cu",
    "Ga" => "Eu.Ca.Ac.H.Ca.Zn",
    "Ge" => "Ho.Ga",
    "As" => "Ge.Na",
    "Se" => "As",
    "Br" => "Se",
    "Kr" => "Br",
    "Rb" => "Kr",
    "Sr" => "Rb",
    "Y" => "Sr.U",
    "Zr" => "Y.H.Ca.Tc",
    "Nb" => "Er.Zr",
    "Mo" => "Nb",
    "Tc" => "Mo",
    "Ru" => "Eu.Ca.Tc",
    "Rh" => "Ho.Ru",
    "Pd" => "Rh",
    "Ag" => "Pd",
    "Cd" => "Ag",
    "In" => "Cd",
    "Sn" => "In",
    "Sb" => "Pm.Sn",
    "Te" => "Eu.Ca.Sb",
    "I" => "Ho.Te",
    "Xe" => "I",
    "Cs" => "Xe",
    "Ba" => "Cs",
    "La" => "Ba",
    "Ce" => "La.H.Ca.Co",
    "Pr" => "Ce",
    "Nd" => "Pr",
    "Pm" => "Nd",
    "Sm" => "Pm.Ca.Zn",
    "Eu" => "Sm",
    "Gd" => "Eu.Ca.Co",
    "Tb" => "Ho.Gd",
    "Dy" => "Tb",
    "Ho" => "Dy",
    "Er" => "Ho.Pm",
    "Tm" => "Er.Ca.Co",
    "Yb" => "Tm",
    "Lu" => "Yb",
    "Hf" => "Lu",
    "Ta" => "Hf.Pa.H.Ca.W",
    "W" => "Ta",
    "Re" => "Ge.Ca.W",
    "Os" => "Re",
    "Ir" => "Os",
    "Pt" => "Ir",
    "Au" => "Pt",
    "Hg" => "Au",
    "Tl" => "Hg",
    "Pb" => "Tl",
    "Bi" => "Pm.Pb",
    "Po" => "Bi",
    "At" => "Po",
    "Rn" => "Ho.At",
    "Fr" => "Rn",
    "Ra" => "Fr",
    "Ac" => "Ra",
    "Th" => "Ac",
    "Pa" => "Th",
    "U" => "Pa",
};