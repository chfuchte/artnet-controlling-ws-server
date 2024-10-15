# Art-Net crate

Provides the neccessary interface to send Art-Net packets via an UDP socket. 

## Why passing the socket to the client?

When passing the socket connection as the `send_fn` to the client it is possible to use the same socket for multiple clients. This is necessary when using multiple universes. Also some operating systems limit the number of sockets using f.e. `0.0.0.0:65454`. Overall this approach may be more compilcated with lifetimes but it definitly is more flexible.

## Credits

- Art-Net™ Designed by and Copyright Artistic Licence
