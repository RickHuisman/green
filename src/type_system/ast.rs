#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Syntax {
    Lambda {
        v: String,
        body: Box<Syntax>,
    },
    Identifier {
        name: String,
    },
    Apply {
        func: Box<Syntax>,
        arg: Box<Syntax>,
    },
    Let {
        v: String,
        defn: Box<Syntax>,
        body: Box<Syntax>,
    },
    LetRec {
        v: String,
        defn: Box<Syntax>,
        body: Box<Syntax>,
    },
}
