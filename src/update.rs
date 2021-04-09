use crate::utils;
use proc_macro::TokenStream;
use tokio_postgres::GenericClient;

pub fn impl_update_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let tablename = utils::get_tablename(ast);

    let my_struct = match &ast.data {
        syn::Data::Struct(d) => d,
        syn::Data::Enum(_) => panic!("Only Structs are supported by AdomCreate"),
        syn::Data::Union(_) => panic!("Only Structs are supported by AdomCreate"),
    };
    let my_fields = &my_struct.fields;
    let update_sql = update_text(&tablename, my_fields);

    //NOTE: we want an error if there is no id on the struct
    let reads_id = ["id"].iter().map(|_| quote! { &self.id });

    let reads_data = my_fields
        .iter()
        .filter(|&f| f.ident.as_ref().unwrap().to_string() != "id")
        .map(|f| f.ident.as_ref())
        .filter_map(|f| f)
        .map(|f| quote! { &self.#f });

    let reads = reads_id.chain(reads_data);

    let params_array = quote! { #(#reads),* };

    let gen = quote! {
        impl #name {

            pub async fn update<C>(&self, client: &C) -> std::result::Result<(), tokio_postgres::error::Error>
                where C: GenericClient
            {
                let sql = #update_sql;
                log::debug!("UPDATE: {}", sql);
                let statement: tokio_postgres::Statement = client.prepare(&sql).await?;
                client.query(&statement, &[#params_array]).await?;
                Ok(())
            }

        }
    };
    //println!("UPDATE: {}", gen);
    gen.into()
}

fn update_text(table: &str, fields: &syn::Fields) -> String {
    let cols = fields
        .iter()
        .filter(|&f| f.ident.as_ref().unwrap().to_string() != "id")
        .map(|f| utils::get_columnname(f));
    //build a list of $1, $2, $3 args for the sql params
    let args = cols
        .clone()
        .enumerate()
        /* NOTE: + 2 to skip over the ID params */
        .map(|(i, _c)| format!("${}", (i + 2)).to_string());
    let pairs: Vec<String> = cols
        .zip(args)
        .map(|(c, a)| format!("{}={}", c, a))
        .collect();
    let pairs = pairs.join(", ");
    format!("UPDATE {} SET {} WHERE id = $1", table, pairs)
}
