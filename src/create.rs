#![allow(unused_imports)]
use crate::utils;
use proc_macro::TokenStream;
use tokio_postgres::GenericClient;

pub fn impl_create_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let tablename = utils::get_tablename(ast);

    let my_struct = match &ast.data {
        syn::Data::Struct(d) => d,
        syn::Data::Enum(_) => panic!("Only Structs are supported by AdomCreate"),
        syn::Data::Union(_) => panic!("Only Structs are supported by AdomCreate"),
    };
    let my_fields = &my_struct.fields;
    let insert_str = insert_text(&tablename, my_fields);

    let reads = my_fields
        .iter()
        .filter(|&f| f.ident.as_ref().unwrap().to_string() != "id")
        .filter(|f| !utils::is_ignore(f))
        .map(|f| f.ident.as_ref())
        .filter_map(|f| f)
        .map(|f| quote! { &self.#f });
    let params_array = quote! { #(#reads),* };

    let gen_create_no_audit = quote! {
        impl #name {
            pub async fn create<C>(&mut self, client: &C) -> std::result::Result<(), tokio_postgres::error::Error>
                where C: GenericClient
            {
                let sql: &str = #insert_str;
                let statement: tokio_postgres::Statement = client.prepare(sql).await?;
                let id_row = client
                    .query_one(
                        &statement,
                        &[#params_array],
                    )
                    .await?;
                self.id = id_row.try_get(0)?;
                Ok(())
            }
        }
    };
    if !utils::is_auditable(ast) {
        return gen_create_no_audit.into();
    }

    let created_by_field = my_fields
        .iter()
        .filter(|f| f.ident.is_some())
        .find(|i| i.ident.as_ref().unwrap().to_string() == "created_by")
        .expect(&format!("Struct {} to contain a field created_by", name));
    let created_by_type = &created_by_field.ty;

    let gen_create_with_audit = quote! {
        impl #name {
            pub async fn create<C>(&mut self, client: &C, by: &#created_by_type) -> std::result::Result<(), tokio_postgres::error::Error>
                where C: GenericClient
            {
                let now = std::time::SystemTime::now();
                self.created_by = by.clone();
                self.updated_by = by.clone();
                self.updated_at = now.clone();
                self.created_at = now;

                let sql: &str = #insert_str;
                let statement: tokio_postgres::Statement = client.prepare(sql).await?;
                let id_row = client
                    .query_one(
                        &statement,
                        &[#params_array],
                    )
                    .await?;
                self.id = id_row.try_get(0)?;
                Ok(())
            }
        }
    };

    let gen = if utils::is_auditable(ast) {
        gen_create_with_audit
    } else {
        gen_create_no_audit
    };

    gen.into()
}

fn insert_text(table: &str, fields: &syn::Fields) -> String {
    let cols: Vec<String> = fields
        .iter()
        .filter(|&f| f.ident.as_ref().unwrap().to_string() != "id")
        .map(|f| utils::get_columnname(f))
        .filter_map(|f| f)
        .collect();
    //build a list of $1, $2, $3 args for the sql params
    let args: Vec<String> = cols
        .iter()
        .enumerate()
        .map(|(i, _c)| format!("${}", (i + 1)).to_string())
        .collect();
    let cols = cols.join(", ");
    let args = args.join(", ");
    format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
        table, cols, args
    )
}
