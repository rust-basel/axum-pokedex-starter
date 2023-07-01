# Pokédex Starter Template

This is a starter template - which will get you going for the start.
It is built up by the more commonly known `Model-View-Controller` pattern.

It currently consists of four modules:

- `model.rs`: Consists of structs, that represent your internal data representation.
- `view.rs`: Consists of structs, that define how the data is represented to the outside world. In a server context,
  this is the json incoming via a `POST`,`PATCH`, or getting returned by a e.g. `GET`. Your view gets transformed into your models
  and vice versa.
- `controller.rs`: The handlers, called from the Axum router and handling the logic. Here you can extract the json (
  view)
  coming in - and also have access to your shared state, like our hashmap database, that store your models. The
  controller usually also maps the model back to a view.
- `main.rs`: The main entry point of you web server, or rust executable in general. All the other modules are referenced
  here.

In a real world scenario you would have a real database and not just a hashmap.
But for our learning purpose that will suffice.

If you are fast enough - you can exchange it with an e.g. `SQlite` database (have a look [here](https://docs.rs/sqlx/latest/sqlx/)).

## Building

To build the solution, just type in `cargo build`, as you should know by now.

## Running

To run the webserver, which offers its endpoints in `127.0.0.1:3000`, you can just type in `cargo run`.
The starting endpoint could be fetched via

```sh
curl -i -X GET "Content-Type: application/json" http://localhost:3000/some-route/42
```

which should return a 
```json
{"some_field": "some_value"}
```