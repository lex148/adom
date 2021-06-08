pub fn get_tablename(ast: &syn::DeriveInput) -> String {
    let name = &ast.ident;
    let mut tablename = name.to_string();
    for attr in ast.attrs.iter() {
        let option = attr.parse_meta().unwrap();
        match option {
            syn::Meta::NameValue(meta) => {
                // Match '#[ident = lit]' attributes. Match guard makes it '#[prefix = lit]'
                if meta.path.is_ident("AdomTable") {
                    if let syn::Lit::Str(lit) = meta.lit {
                        tablename = lit.value();
                    }
                }
            }
            _ => {}
        };
    }
    tablename
}

pub fn is_auditable(ast: &syn::DeriveInput) -> bool {
    for attr in ast.attrs.iter() {
        let option = attr.parse_meta().unwrap();
        match option {
            syn::Meta::Path(p) => {
                if p.is_ident("AdomAuditable") {
                    return true;
                }
            }
            _ => {}
        };
    }
    false
}

pub fn is_ignore(field: &syn::Field) -> bool {
    for attr in field.attrs.iter() {
        let meta = attr.parse_meta().unwrap();
        match meta {
            syn::Meta::Path(p) => {
                if p.is_ident("AdomIgnore") {
                    return true;
                }
            }
            _ => {}
        };
    }
    return false;
}

pub fn get_columnname(field: &syn::Field) -> Option<String> {
    for attr in field.attrs.iter() {
        let meta = attr.parse_meta().unwrap();
        match meta {
            syn::Meta::NameValue(meta_nv) => {
                // Match '#[ident = lit]' attributes. Match guard makes it '#[prefix = lit]'
                if meta_nv.path.is_ident("AdomColumn") {
                    if let syn::Lit::Str(lit) = meta_nv.lit {
                        return Some(lit.value());
                    }
                }
            }
            syn::Meta::Path(p) => {
                if p.is_ident("AdomIgnore") {
                    return None;
                }
            }
            _ => {}
        };
    }
    Some(field.ident.as_ref().unwrap().to_string())
}
