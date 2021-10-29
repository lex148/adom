#![allow(unused_imports)]
use crate::utils;
use proc_macro::TokenStream;
use tokio_postgres::GenericClient;

pub fn impl_select_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let tablename = utils::get_tablename(ast);

    let my_struct = match &ast.data {
        syn::Data::Struct(d) => d,
        syn::Data::Enum(_) => panic!("Only Structs are supported by AdomCreate"),
        syn::Data::Union(_) => panic!("Only Structs are supported by AdomCreate"),
    };
    let my_fields = &my_struct.fields;
    let find_sql = select_text(&tablename, my_fields);

    let from_field_setters = my_fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        match utils::get_columnname(f) {
            Some(col) => quote! { #ident: row.try_get(#col)? },
            None => quote! { #ident: Default::default() },
        }
    });
    let from_fields = quote! { #(#from_field_setters),* };

    let gen = quote! {
        impl #name {

            pub async fn find_by_id<T,C>(client: &C, id: T) -> std::result::Result<Option<Self>, tokio_postgres::error::Error>
                where T: tokio_postgres::types::ToSql + Sync,
                C: GenericClient
            {
                let mut found = Self::find_where(client, "id = $1", &[&id]).await?;
                if found.len() == 0 {
                    return Ok(None);
                }
                let s:Self = found.pop().unwrap();
                Ok( Some(s) )
            }

            pub async fn with_ids<T>(client: &T, ids: &[i64]
                ) -> std::result::Result<Vec<Self>, tokio_postgres::error::Error>
                where T: GenericClient
            {
                if ids.len() == 0 {
                    return Ok( vec![] );
                }
                //NOTE: we are 100% sure we have i64 and there are more than one at this point
                let ids = ids
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let q = format!("id in ({}) ", ids);
                let found = Self::find_where(client, &q, &[]).await?;
                Ok( found )
            }

            pub async fn one_where<T>(
                client: &T,
                where_clause: &str,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> std::result::Result<Option<Self>, tokio_postgres::error::Error>
                where T: GenericClient
            {
                let sql = format!("{} WHERE {} LIMIT 1", #find_sql, where_clause);
                log::debug!("one_where: {}", sql);
                let statement: tokio_postgres::Statement = client.prepare(&sql).await?;
                let mut rows = client.query(&statement, params).await?;
                if rows.len() == 0 {
                    return Ok(None);
                }
                let row = rows.pop().unwrap();
                let s = Self::from_pg_row( &row )?;
                Ok( Some(s) )
            }

            pub async fn find_where<T>(
                client: &T,
                where_clause: &str,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> std::result::Result<Vec<Self>, tokio_postgres::error::Error>
                where T: GenericClient
            {
                let sql = format!("{} WHERE {}", #find_sql, where_clause);
                log::debug!("find_where: {}", sql);
                let statement: tokio_postgres::Statement = client.prepare(&sql).await?;
                let rows = client.query(&statement, params).await?;
                let found: std::result::Result<Vec<Self>, tokio_postgres::error::Error> =
                    rows.iter().map(|x| Self::from_pg_row(x) ).collect();
                Ok(found?)
            }

            fn select_text() -> String {
                #find_sql.to_string()
            }

            fn tablename() -> String {
                #tablename.to_string()
            }

            fn from_pg_row(row: &tokio_postgres::row::Row) ->
                std::result::Result<Self, tokio_postgres::error::Error> {
                log::debug!("from_pg_row");
                Ok(Self {
                    #from_fields
                })
            }

        }
    };
    gen.into()
}

fn select_text(table: &str, fields: &syn::Fields) -> String {
    let cols: Vec<String> = fields
        .iter()
        .map(|f| utils::get_columnname(f))
        .filter_map(|f| f)
        .map(|col| format!("{}.\"{}\"", table, col))
        .collect();
    let cols = cols.join(", ");
    format!("SELECT {} FROM {}", cols, table)
}
