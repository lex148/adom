#![allow(unused_imports)]
use crate::utils;
use proc_macro::TokenStream;
use tokio_postgres::GenericClient;

pub fn impl_delete_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let tablename = utils::get_tablename(ast);
    let delete_sql = delete_text(&tablename);

    let gen = quote! {
        impl #name {

            pub async fn delete<C>(
                self,
                client: &C,
            ) -> std::result::Result<(), tokio_postgres::error::Error>
                where C: GenericClient
            {
                Self::delete_where(client, "id = $1", &[&self.id]).await?;
                Ok(())
            }

            pub async fn delete_where<C>(
                client: &C,
                where_clause: &str,
                params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
            ) -> std::result::Result<(), tokio_postgres::error::Error>
                where C: GenericClient
            {
                let sql = format!("{} WHERE {}", #delete_sql, where_clause);
                log::debug!("delete_where: {}", sql);
                let statement: tokio_postgres::Statement = client.prepare(&sql).await?;
                let rows = client.query(&statement, params).await?;
                Ok(())
            }


        }
    };

    gen.into()
}

fn delete_text(table: &str) -> String {
    format!("DELETE FROM {}", table)
}
