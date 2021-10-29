# Adom - A light weight Object Mapper for async-postgres in Rust

This crate is a set of macros used to generate simple CRUD functions for your Serde structs

Adom is not intended to be a full on ORM like diesel. Its for the people that
want to use the postgres driver directly. But want to eliminate some of the boiler plate
code on their simple table objects.

[A simple example of using adom in a project can be found here](https://github.com/lex148/adom_actix_web_example)

## Setup your structs

Add an attribute to the top of your struct to connect it to the DB table

```rust
#[AdomTable = "orders"]
pub struct Order{
  id: i32,
  customer: String,
  total: f64,
}
```

Congratulations nothing happened, but you can now use Adom

## Adom derive attribute

To use Adom add derive statements to add useful functions to your struct

```rust
#[derive(Deserialize, AdomSelect, AdomUpdate, AdomCreate, AdomDelete)]
#[AdomTable = "orders"]
pub struct Order{
  id: i32,
  customer: String,
  total: f64,
}
```

Each derive adds functions related to the DB action. Add only what you want or
need

AdomSelect

- Order::find_by_id(... )
- Order::one_where(... )
- Order::find_where(... )

AdomCreate

- order.create(.... )

AdomUpdate

- order.update(.... )

AdomDelete

- order.delete(.... )

## Helper attributes

There are a couple of helper attributes that modify the behavior of Adom

AdomColumn - used to if the struct field name doesn't match the DB columns name

```
#[derive(Deserialize, AdomSelect, AdomUpdate, AdomCreate, AdomDelete)]
#[AdomTable = "orders"]
pub struct Order{
  id: i32,
  #[AdomColumn = "customer_name"]
  customer: String,
  total: f64,
}
```

AdomIgnore - ignore a field on your struct

```
#[derive(Deserialize, AdomSelect, AdomUpdate, AdomCreate, AdomDelete)]
#[AdomTable = "orders"]
pub struct Order{
  id: i32,
  customer: String,
  total: f64,
  #[AdomIgnore]
  random_extra_field_that_has_nothing_to_do_with_db: Vec<u8>,
}
```

AdomAuditable - auto fill and update the traditional DB auditing fields

```
#[derive(Deserialize, AdomSelect, AdomUpdate, AdomCreate, AdomDelete)]
#[AdomTable = "orders"]
#[AdomAuditable]
pub struct Order{
  id: i32,
  customer: String,
  total: f64,
  updated_at: SystemTime,
  created_at: SystemTime,
  updated_by: String,
  created_by: String,
}
```
