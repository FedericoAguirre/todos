-- CREATE TABLE IF NOT EXISTS "todos" ( "created_at" timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP, "updated_at" timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP, "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT, "todo" varchar NOT NULL, "priority" tinyint NOT NULL, "start_date" date_text NOT NULL, "end_date" date_text NULL, "status_id" integer NOT NULL, FOREIGN KEY ("status_id") REFERENCES "statuses" ("id") ON DELETE CASCADE ON UPDATE CASCADE );

INSERT INTO "todos" ("created_at", "updated_at", "todo", "priority", "start_date", "end_date", "status_id") VALUES
('2025-01-01T00:00:00.000Z', '2025-01-01T00:00:00.000Z', 'In /todos/{id}/edit, add status_id control (listbox)', 6, '2025-06-13', NULL, 1),
('2025-01-01T00:00:00.000Z', '2025-01-01T00:00:00.000Z', 'In /todos route, order by priority ascending', 2, '2025-06-13', NULL, 2),
('2025-01-01T00:00:00.000Z', '2025-01-01T00:00:00.000Z', 'In /todos route, add status_id and status', 3, '2025-06-13', NULL, 1);

-- update todos set status_id = 3 where priority = 3;
-- update todos set status_id = 3 where priority = 3;

-- update todos set status_id = 3, end_date = '2025-06-13' where id = 12;
