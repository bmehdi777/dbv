# dbv - Database visualizer

`dbv` is a tool to visualize database schema, execute query etc.

## Technical thought - only for me

- add a pane "are you sure?y/N" when deleting/droping db/droping database
- add a pane to add a connection : multiple input to register name + connection string
- make a driver trait, which will be implemented into three different struct : mysql, postgres and sqlite
