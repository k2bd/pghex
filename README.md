# pghex

[![CI](https://github.com/k2bd/pghex/actions/workflows/ci.yml/badge.svg)](https://github.com/k2bd/pghex/actions/workflows/ci.yml)

This is a Postgres extension adding Hex tiles and related operations to Postgres.

## Example

Here's an example where we create a couple tables representing a hero exploring a dungeon on a hexagonal grid.
One table represents the walls that restrict vision, and another represents units positioned in the dungeon.

```sql
create extension pghex;
```

```sql
create table obstacles ( coord hex );
insert into obstacles ( coord )
    values ('[0, -2]'), ('[1, -2]'), ('[3, -2]'), ('[4, -3]'), ('[4, -2]'), ('[-2, 3]'), ('[-2, 2]'), ('[-2, 1]'), ('[-3, 1]');
```

```sql
select * from obstacles;
```
```
     coord      
----------------
 {"q":0,"r":-2}
 {"q":1,"r":-2}
 {"q":3,"r":-2}
 {"q":4,"r":-3}
 {"q":4,"r":-2}
 {"q":-2,"r":3}
 {"q":-2,"r":2}
 {"q":-2,"r":1}
 {"q":-3,"r":1}
(9 rows)
```

```sql
create table units ( name varchar, vision_range int, position hex );
insert into units (name, vision_range, position)
    values ('Hero', 6, '[0, 1]'), ('Goblin 1', 3, '[5, 1]');
```

```sql
select * from units;
```
```
   name   | vision_range |   position    
----------+--------------+---------------
 Hero     |            6 | {"q":0,"r":1}
 Goblin 1 |            3 | {"q":5,"r":1}
(2 rows)
```

We can then, for example, create a query for the coordinates that are [visible to each unit](https://www.redblobgames.com/grids/hexagons/#field-of-view):

```sql
with available_tiles as (
    select
        name,
        position,
        hexes_in_range(position, vision_range) tile
    from units
)
select name, tile from available_tiles
where not exists(
    select coord from obstacles
    where coord in (select * from linedraw(position, tile))
);
```

```
  name   |      tile      
----------+-----------------
Hero     | {"q":-6,"r":0}
Hero     | {"q":-5,"r":-1}
Hero     | {"q":-5,"r":0}
Hero     | {"q":-4,"r":-2}
        ...
Goblin 1 | {"q":8,"r":-2}
Goblin 1 | {"q":8,"r":-1}
Goblin 1 | {"q":8,"r":0}
Goblin 1 | {"q":8,"r":1}
(110 rows)
```

## Installing

At the moment this can only be installed in a development environment using `cargo pgrx run`.
Packaging and distributing will be done later.

## Developing

Requirements:
- [Rust](https://www.rust-lang.org/tools/install)
- [pgrx](https://github.com/pgcentralfoundation/pgrx?tab=readme-ov-file#getting-started)

### Commands

- `cargo pgrx test` - Run tests
- `cargo pgrx run` - Open a local psql terminal to test out the extension

## Acknowledgements

- [Hexagonal Grids by Red Blob Games](https://www.redblobgames.com/grids/hexagons/)
