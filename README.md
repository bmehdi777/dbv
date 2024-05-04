# dbv - Database visualizer

`dbv` is a tool to visualize database schema, execute query etc.

## Technical thought - only for me

- only allow selection on other pane if previous is selected : if no db selected, no table etc.
- add a pane "are you sure?y/N" when deleting/droping db/droping database
- add a pane to add a connection : multiple input to register name + connection string


- Performance issue on rendering of list : if you keep button pressed, it goes down even when you don't. Maybe a problem with debug version ? -> indeed, it's only in debug mode
- reset view when changing connection/database/table

