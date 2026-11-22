FooBar, despite its name, it is more than a bar. The components are as follows (in no particular order, also this list may change):

- Bar
- Selector menu: similar to dmenu. Is able to select between user/script provided option 
  - Application launcher: same as selector but open apps
  - Smart Finder: same as selector but is able to search and open files, apps, text in files etc...

May also be added later:
- File explorer
- A file picker dialog (based on the above): would be used as xdg file dialog opener
- An emoji picker
- A wayland clipboard manager (would also be searchable by finder)


All these application should follow the same design language / choices and the same global settings should apply. 
To achive this a user-space daemon is used (foodaemon):

- The daemon is launched as a user application (so NOT as a systemd / other system daemon) and is tied to a wayland session (1 instance / $WAYLAND_DISPLAY)

- The daemon is responsible for managing settings: saving them to disk, telling clients about changes, and listening from clients for changes

- The daemon is also responsible for sharing some global resources between clients. (None at the moment, but thing like wayland clipboard is planned)


The communication between the daemon and the clients is achieved with streamed unix (domain) sockets.
The communication frame is as follows: u32 + u8 (should be read in 1) where u32 is the following data size, and u8 is the opcode all followed by the payload.

A more detailed explanination can be found in communication.md.