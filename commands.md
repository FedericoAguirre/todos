# Commands

## Método Ivy Lee

1. Escribe cada noche las 6 tareas más importantes para el día siguiente.
2. Ordena de mayor a menor importancia las tareas.
3. Céntrate en la primera tarea y no cambies de tarea hasta terminarla.
4. Piensa si mover o no las tareas sin completar a la lista de mañana.
5. Repite esto todos los días y verás la diferencia.

Task
  date date
  task varchar(255)
  priority smallint 1 to 6
  status_id fk status

## Status

```bash
cargo loco generate scaffold status \
    status:string \
    --htmx
```

cargo loco generate scaffold todo \
    todo:string! \
    priority:tiny_int! \
    start_date:date! \
    end_date:date \
    status:references \
    --htmx

cargo loco db migrate
cargo test

