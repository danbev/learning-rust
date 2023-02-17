## Serde framework
Is a framework for `ser`ializing and `de`serializing (serde) Rust data structurs.

The core is not about any particular format but more about the infrastructure to
do serialization and derserialization of data structures.

### Overview
```
+------------+       +------------------+         +-------------+
|  Data Type |       | Serde Data Model |         | Data Format |
|------------|       |------------------|         |-------------|
|Serialize   | <---> |                  | <-----> | Serializer  |
|Deserialize |       |                  |         | Deserializer|
+------------+       +------------------+         +-------------+
```
The `Serialize` and `Deserialize` types, in the Data Type above, only know about
the Serde Data Model. Likewise the `Serializer` and `Deserializer`, in the
Data Format above, also only have to know about the Data Model API.

### Data model
The consists mostly of a set of types which represents things we can represents
in Rust code.
So Serialize will describe how Rust data structures and map them to types in the
Data Model.
And Deserialize will desribe how types in the Data Model map to Rust data types. 

### Data Format
This side is concerned with how the bytes of the data formats map to the Rust
Data Model. And for the Deserializer is is concerned with mapping the Rust Data
Model types back in to bytes in the specific format.

So to add new formats one would only have to be concerned with the Serializer
and Deserializer, plus the Data Model API, but would not have to deal with
the Data Type side above.
