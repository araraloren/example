use neure::{
    ctx::{re_policy, PolicyCtx},
    prelude::*,
};

#[derive(Debug, Clone)]
pub enum Ty<'a> {
    Layer0(&'a str),

    Layer1(&'a str, &'a str),

    Layer2(&'a str, &'a str, &'a str),
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    #[allow(unused)]
    ident: &'a str,

    #[allow(unused)]
    ty_name: Ty<'a>,

    public: bool,
}

impl<'a> Field<'a> {
    pub fn private(ident: &'a str, ty_name: Ty<'a>) -> Self {
        Self {
            ident,
            ty_name,
            public: false,
        }
    }

    pub fn public(name: &'a str, ty_name: Ty<'a>) -> Self {
        Self {
            ident: name,
            ty_name,
            public: true,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unit = neu::ascii_alphabetic()
        .or(neu::ascii_alphanumeric())
        .or('_');
    let ident = unit.repeat_one_more();
    let ty = neu::ascii_alphabetic()
        .or('_')
        .repeat_one()
        .then(ident)
        .pat();
    let layer1 = ty
        .then(ty.quote("<", ">"))
        .map(|(w, ty)| Ok(Ty::Layer1(w, ty)));
    let layer2 = ty
        .then(ty.then(ty.quote("<", ">")).quote("<", ">"))
        .map(|(w1, (w2, ty))| Ok(Ty::Layer2(w1, w2, ty)));
    let layer0 = ty.map(|ty| Ok(Ty::Layer0(ty)));
    let field = ident.sep_once(":", layer2.or(layer1.or(layer0)));
    let public = field
        .padded("pub")
        .map(|(name, ty_name)| Ok(Field::public(name, ty_name)));
    let private = field.map(|(name, ty_name)| Ok(Field::private(name, ty_name)));
    let parser = public.or(private).sep(",");
    
    let data = "abc: Option<i32>";
    let b_policy = re_policy(neu::whitespace().repeat_full());

    dbg!(PolicyCtx::new(CharsCtx::new(data), b_policy).ctor(&parser)?);

    Ok(())
}
